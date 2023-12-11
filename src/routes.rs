use std::collections::HashMap;

use axum::{extract::Path, http::StatusCode, Json};
use biskuit::{establish_connection, models::ShortUrl, schema::short_urls};
use diesel::{
    result::Error::{self},
    ExpressionMethods, QueryDsl, RunQueryDsl, SelectableHelper,
};
use serde::Serialize;

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
