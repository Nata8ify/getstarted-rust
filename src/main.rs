mod handlers;
mod models;
mod router;

use axum::serve;
use tokio::{
    net::TcpListener
};
use router::router;

#[tokio::main]
async fn main() {

    let router = router();

    let listener = TcpListener::bind("0.0.0.0:8080").await.unwrap();

    println!("Listening on http://localhost:8080 [START]");
    serve(listener, router).await.unwrap();
    println!("Listening on http://localhost:8080 [END]");
}