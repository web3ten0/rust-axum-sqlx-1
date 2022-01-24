use axum::http::header::CONTENT_TYPE;
use std::net::SocketAddr;
use tower_http::cors::{any, CorsLayer, Origin};

mod controllers;
mod db;
mod models;
mod router;
mod error;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    let app = router::router().layer(cors());
    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

fn cors() -> CorsLayer {
    let swagger_url = "http://localhost:8001";
    CorsLayer::new()
        .allow_origin(Origin::exact(swagger_url.parse().unwrap()))
        .allow_methods(any())
        .allow_headers(vec![CONTENT_TYPE])
}
