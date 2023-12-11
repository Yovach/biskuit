use axum::{http::StatusCode, Json};
use serde::Serialize;

#[derive(Serialize)]
pub struct ShortenedUrl {
    id: String,
    url: String,
}

pub async fn get_shortened_url() -> (StatusCode, Json<ShortenedUrl>) {
    return (
        StatusCode::OK,
        Json(ShortenedUrl {
            id: String::new(),
            url: String::new(),
        }),
    )
}
