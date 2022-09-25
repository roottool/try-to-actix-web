use axum::{response::IntoResponse, routing::get, Json, Router};
use serde::Serialize;
use std::net::SocketAddr;
use tracing_subscriber::FmtSubscriber;

#[tokio::main]
async fn main() {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(tracing::Level::INFO)
        .finish();
    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    let app = Router::new().route("/", get(hello));

    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    tracing::info!("listening on {}", addr);

    axum::Server::bind(&"127.0.0.1:8080".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

#[tracing::instrument]
async fn hello() -> impl IntoResponse {
    tracing::info!("Access health check");

    let hello_world = HelloWorld {
        country: "Japan",
        hello_world: "ハローワールド",
    };
    Json(hello_world)
}

#[derive(Serialize)]
struct HelloWorld<'a> {
    country: &'a str,
    hello_world: &'a str,
}
