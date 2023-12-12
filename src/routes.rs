use std::collections::HashMap;

use axum::{extract::Path, http::StatusCode, Json};
use biskuit::{establish_connection, models::ShortUrl, schema::short_urls};
use diesel::{
    result::Error::{self},
    ExpressionMethods, Insertable, QueryDsl, RunQueryDsl, SelectableHelper,
};
use nanoid::nanoid;
use serde::{Deserialize, Serialize};
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
