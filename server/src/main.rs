use axum::{
    routing::{get, post},
    Router,
};
use dotenvy::dotenv;
use tracing::info;
use sha2::Sha256;
use hmac::{Hmac, Mac};

pub mod routes;

#[tokio::main]
async fn main() {
    dotenv().ok();

    // initialize tracing
    tracing_subscriber::fmt::init();

    let key: Hmac<Sha256> = Hmac::new_from_slice(b"some-secret").expect("I except a value here");
    println!("{:?}", key);

    // build our application with a route
    let app = Router::new()
        .route("/s/:id", get(routes::get_short_url))
        .route("/s/", post(routes::create_short_url))
        .route("/login/", post(routes::login));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    info!(target: "test", "listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}
