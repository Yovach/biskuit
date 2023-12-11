use axum::{http::StatusCode, Json};
use biskuit::{establish_connection, models::ShortUrl};
use diesel::{QueryDsl, RunQueryDsl, SelectableHelper};
use serde::Serialize;

#[derive(Serialize)]
pub struct GetShortUrl {
    data: Vec<ShortUrl>,
}

pub async fn get_short_url() -> (StatusCode, Json<GetShortUrl>) {
    use biskuit::schema::short_urls::dsl::*;
    let conn = &mut establish_connection();

    let urls = short_urls
        .select(ShortUrl::as_select())
        .load(conn)
        .expect("error while loading short urls");

    (StatusCode::OK, Json(GetShortUrl { data: urls }))
}
