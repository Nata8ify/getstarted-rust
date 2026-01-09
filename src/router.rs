use axum::Router;
use axum::routing::get;
use crate::handlers::sample;

pub fn router() -> Router {
    Router::new()
        .route("/text", get(sample::hello))
        .route("/html", get(sample::html))
        .route("/json", get(sample::json))
}