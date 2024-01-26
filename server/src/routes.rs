use std::{
    collections::HashMap,
    env,
    time::{Duration, SystemTime},
};

use axum::{extract::Path, http::StatusCode, Json};
use biskuit::{establish_connection, is_jwt_valid, models::ShortUrl, schema::short_urls};
use diesel::{
    result::Error::{self},
    ExpressionMethods, Insertable, QueryDsl, RunQueryDsl, SelectableHelper,
};
use hmac::{Hmac, Mac};
use jwt::{AlgorithmType, Claims, Header, RegisteredClaims, SignWithKey, Token};
use nanoid::nanoid;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sha2::Sha256;
use url::Url;

pub async fn get_short_url(
    Path(params): Path<HashMap<String, String>>,
) -> (StatusCode, Json<GetShortUrlResponse>) {
    println!("{:?}", params);

    let short_url_id = params.get("id");
    println!(" short url id : {:?}", short_url_id);
    if short_url_id.is_none() {
        return (
            StatusCode::NOT_FOUND,
            Json(GetShortUrlResponse {
                error: Some("can't create a short url for invalid url".to_string()),
                data: None,
            }),
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
                Json(GetShortUrlResponse {
                    error: None,
                    data: Some(url),
                }),
            );
        }
        Err(err) => match err {
            Error::NotFound => {
                return (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(GetShortUrlResponse {
                        error: Some("I was not able to create your shortened url".to_string()),
                        data: None,
                    }),
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

    jwt: String,
}

#[derive(Insertable)]
#[diesel(table_name = short_urls)]
struct InsertShortUrl {
    id: Option<i32>,
    token: String,
    url: String,
}

#[derive(Serialize)]
pub struct GetShortUrlResponse {
    data: Option<ShortUrl>,
    error: Option<String>,
}

pub async fn create_short_url(
    Json(payload): Json<CreateShortUrl>,
) -> (StatusCode, Json<GetShortUrlResponse>) {
    let jwt = &payload.jwt;
    let verification = is_jwt_valid(jwt, &"auth".to_string());

    if let Err(error_code) = verification {
        return (
            StatusCode::BAD_REQUEST,
            Json(GetShortUrlResponse {
                error: Some(error_code.to_string()),
                data: None,
            }),
        );
    }

    let url = &payload.url;

    let validation = Url::parse(url);
    if validation.is_err() {
        return (
            StatusCode::BAD_REQUEST,
            Json(GetShortUrlResponse {
                error: Some("can't parse URL".to_string()),
                data: None,
            }),
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
                Json(GetShortUrlResponse {
                    error: None,
                    data: Some(url),
                }),
            );
        }
        Err(err) => match err {
            Error::NotFound => {
                return (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(GetShortUrlResponse {
                        error: Some("i wasn't able to insert your short url".to_string()),
                        data: None,
                    }),
                );
            }
            _ => {
                panic!("Database error : {:?}", err);
            }
        },
    }
}

#[derive(Serialize)]
pub struct LoginSuccessData {
    jwt: String,
}

#[derive(Debug, Deserialize)]
pub struct LoginPayload {
    username: String,
    password: String,
}

#[derive(Serialize)]
pub struct LoginResponseError {
    error: String,
}

#[derive(Serialize)]
pub struct LoginDataResponse {
    data: Option<LoginSuccessData>,
    error: Option<String>,
}

pub async fn login(Json(payload): Json<LoginPayload>) -> (StatusCode, Json<LoginDataResponse>) {
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

        let issued_at = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH);
        let expired_at = SystemTime::now()
            .checked_add(Duration::new(5 * 60, 0))
            .unwrap()
            .duration_since(SystemTime::UNIX_EPOCH);

        let mut claims = Claims::new(RegisteredClaims {
            not_before: None,
            issuer: None,
            audience: None,
            json_web_token_id: None,

            issued_at: Some(issued_at.unwrap().as_secs()),
            expiration: Some(expired_at.unwrap().as_secs()),
            subject: Some("auth".to_string()),
        });
        claims
            .private
            .insert("username".to_string(), Value::String(username));

        let token = Token::new(header, claims).sign_with_key(&key);
        if token.is_err() {
            println!("err: {:?}", token.err());
            return (
                StatusCode::BAD_REQUEST,
                Json(LoginDataResponse {
                    error: Some("invalid token".to_string()),
                    data: None,
                }),
            );
        }

        let token_result = token.unwrap();
        println!("claims: {:?}", token_result.header());

        return (
            StatusCode::OK,
            Json(LoginDataResponse {
                error: None,
                data: Some(LoginSuccessData {
                    jwt: token_result.as_str().to_string(),
                }),
            }),
        );
    }

    return (
        StatusCode::BAD_REQUEST,
        Json(LoginDataResponse {
            error: Some("An error has been occured".to_string()),
            data: None,
        }),
    );
}
