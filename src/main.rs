#![allow(unused)]

use axum::{http::StatusCode, response:: {Html, IntoResponse}, routing::{ get, post }, Router};
//use axum::response::Html;

use std::net::SocketAddr;

#[tokio::main]
async fn main() {

    let addr = "127.0.0.1:5050";

    println!("Server listening on {addr:?}\n");

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();

    axum::serve(listener, router())
        .await
        .unwrap();
}

fn router() -> Router {
    Router::new().route("/hello", get(hello_world).post(post_handler))
}

async fn hello_world() -> &'static str {
    "Hello from Axum!"
}

async fn post_handler() -> impl IntoResponse {
    (StatusCode::CREATED, "Post Created!")
}

//get(|| async { Html("hello <strong>World!!!</strong>") }),
