use axum::{
    http::{HeaderValue, Method, header::{self}},
    routing::{get, post},
    Router,
};
use biskuit::is_database_updated;
use dotenvy::dotenv;
use tower_http::cors::CorsLayer;
use tracing::info;

pub mod routes;

#[tokio::main]
async fn main() {
    dotenv().ok();

    if !is_database_updated() {
        panic!("Database is not up. Please execute migrations before continue")
    }

    // initialize tracing
    tracing_subscriber::fmt::init();

    // build our application with a route
    let app = Router::new()
        .route("/s/:id", get(routes::get_short_url))
        .route("/s/", post(routes::create_short_url))
        .route("/login/", post(routes::login))
        .layer(
            CorsLayer::new()
                .allow_origin("*".parse::<HeaderValue>().unwrap())
                .allow_methods([Method::GET, Method::POST])
                .allow_headers([header::CONTENT_TYPE])
        );
    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    info!(target: "test", "listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}
