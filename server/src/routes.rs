use std::{
    collections::{BTreeMap, HashMap},
    env,
};

use axum::{body::Body, extract::Path, http::StatusCode, response::Response, Json};
use biskuit::{establish_connection, models::ShortUrl, schema::short_urls};
use diesel::{
    result::Error::{self},
    ExpressionMethods, Insertable, QueryDsl, RunQueryDsl, SelectableHelper,
};
use hmac::{Hmac, Mac};
use jwt::{SignWithKey, Header, AlgorithmType, Token};
use nanoid::nanoid;
use serde::{Deserialize, Serialize};
use sha2::Sha256;
use url::Url;

#[derive(Serialize)]
pub struct GetShortUrlResponse {
    data: Option<ShortUrl>,
}

pub async fn get_short_url(
    Path(params): Path<HashMap<String, String>>,
) -> (StatusCode, Json<GetShortUrlResponse>) {
    println!("{:?}", params);

    let short_url_id = params.get("id");
    println!(" short url id : {:?}", short_url_id);
    if short_url_id.is_none() {
        return (
            StatusCode::NOT_FOUND,
            Json(GetShortUrlResponse { data: None }),
        );
    }

    let unwrapped_id = short_url_id.expect("");

    let conn = &mut establish_connection();

    let result = short_urls::table
        .filter(short_urls::token.eq(unwrapped_id.to_string()))
        .select(ShortUrl::as_select())
        .first(conn);
    match result {
        Ok(url) => {
            return (
                StatusCode::OK,
                Json(GetShortUrlResponse { data: Some(url) }),
            );
        }
        Err(err) => match err {
            Error::NotFound => {
                return (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(GetShortUrlResponse { data: None }),
                )
            }
            _ => {
                panic!("Database error : {:?}", err);
            }
        },
    }
}

#[derive(Debug, Deserialize)]
pub struct CreateShortUrl {
    url: String,
}

#[derive(Insertable)]
#[diesel(table_name = short_urls)]
struct InsertShortUrl {
    id: Option<i32>,
    token: String,
    url: String,
}

pub async fn create_short_url(
    Json(payload): Json<CreateShortUrl>,
) -> (StatusCode, Json<GetShortUrlResponse>) {
    let url = &payload.url;

    let validation = Url::parse(url);
    if validation.is_err() {
        return (
            StatusCode::BAD_REQUEST,
            Json(GetShortUrlResponse { data: None }),
        );
    }

    let new_short_url = InsertShortUrl {
        id: None,
        token: nanoid!(6),
        url: url.to_string(),
    };

    let conn = &mut establish_connection();
    let result = diesel::insert_into(short_urls::table)
        .values(&new_short_url)
        .get_result(conn);
    match result {
        Ok(url) => {
            return (
                StatusCode::OK,
                Json(GetShortUrlResponse { data: Some(url) }),
            );
        }
        Err(err) => match err {
            Error::NotFound => {
                return (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(GetShortUrlResponse { data: None }),
                )
            }
            _ => {
                panic!("Database error : {:?}", err);
            }
        },
    }
}

#[derive(Serialize)]
pub struct LoginResponse {
    data: u32,
}

#[derive(Debug, Deserialize)]
pub struct LoginPayload {
    username: String,
    password: String,
}

pub async fn login(Json(payload): Json<LoginPayload>) -> Response {
    let username = payload.username;
    let password = payload.password;
    if username == "admin" && password == "admin" {
        let secret_env = env::var("JWT_SECRET").expect("i expected a value here");
        // return Response::builder()
        //     .status(StatusCode::INTERNAL_SERVER_ERROR)
        //     .body(Body::from("can't read env"))
        //     .unwrap();

        let key: Hmac<Sha256> = Hmac::new_from_slice(secret_env.as_bytes()).unwrap();

        let header = Header {
            algorithm: AlgorithmType::Hs256,
            ..Default::default()
        };
        let mut claims = BTreeMap::new();
        claims.insert("username", username);
        
        
        let token = Token::new(header, claims).sign_with_key(&key);
        if token.is_err() {
            println!("err: {:?}", token.err());
            return Response::builder()
            .status(StatusCode::OK)
            .body(Body::from("invalid token"))
            .unwrap();
        }


        let token_result = token.unwrap();
        return Response::builder()
            .status(StatusCode::OK)
            .body(Body::from(token_result.as_str().to_string()))
            .unwrap();
    }
    return Response::builder()
        .status(StatusCode::INTERNAL_SERVER_ERROR)
        .body(Body::from("can't read env"))
        .unwrap();
}
