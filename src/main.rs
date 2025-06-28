#![allow(unused)]

use axum::{
    http::StatusCode, 
    response:: {Html, Response, IntoResponse}, 
    routing::{ get, post }, 
    body::Body,
    Router,
    Json
};

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
    Router::new().route("/hello", get(get_handler_2).post(post_handler))
}

async fn hello_world() -> &'static str {
    "Hello from Axum, here we come, stockmarket!"
}

async fn post_handler() -> impl IntoResponse {
    (StatusCode::CREATED, "Post Created!")
}

async fn get_handler_2() -> Response {
    Response::builder()
        .status(StatusCode::CREATED)
        .header("Content-Type", "application/json")
        .body(Body::from(r#"{"dude": "Donald Dux"}"#))
        .unwrap()
}
//get(|| async { Html("hello <strong>World!!!</strong>") }),
