use axum::{
    routing::{get, post},
    Router,
};
use tracing::info;

pub mod routes;

#[tokio::main]
async fn main() {
    // initialize tracing
    tracing_subscriber::fmt::init();

    // build our application with a route
    let app = Router::new()
        .route("/s/:id", get(routes::get_short_url))
        .route("/s/", post(routes::create_short_url));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    info!(target: "test", "listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}
