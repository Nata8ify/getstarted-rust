
use axum::http::{StatusCode};
use axum::Json;
use axum::response::{Html, IntoResponse};
use crate::{
    models::sample::Account
};

pub async fn hello() -> &'static str {
    "Hello, World!"
}

pub async fn html() -> Html<String> {
    Html(String::from("<h1>Hello, <i style=\"color: orange;\">World!</i></h1>"))
}

pub async fn json() -> impl IntoResponse {
    (
        StatusCode::OK,
        [("ContentType", "application/json")],
        Json(Account {
            name: String::from("John"),
            balance: 100.0
        })
    )
}