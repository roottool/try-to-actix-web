use axum::{extract::Query, response::IntoResponse, routing::get, Json, Router};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::net::SocketAddr;
use tracing_subscriber::FmtSubscriber;

#[tokio::main]
async fn main() {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(tracing::Level::INFO)
        .finish();
    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    let app = Router::new()
        .route("/", get(hello))
        .route("/hello", get(hello_with_query));

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

    let hello_world = HelloWorldResponse {
        country: "Japan",
        hello_world: "ハローワールド",
    };
    Json(hello_world)
}

#[tracing::instrument]
async fn hello_with_query(Query(params): Query<HelloWorldPayload>) -> Json<Value> {
    tracing::info!("Access health check");

    let hello_world = HelloWorldResponse {
        country: &params.country,
        hello_world: "Hello World",
    };
    Json(json!(hello_world))
}

#[derive(Deserialize, Debug)]
struct HelloWorldPayload {
    country: String,
}

#[derive(Serialize)]
struct HelloWorldResponse<'a> {
    country: &'a str,
    hello_world: &'a str,
}
