#![allow(unused)]

use axum::{debug_handler,Json,Router};
use axum::body::Body;
use axum::http::StatusCode;
use axum::response::{Html,IntoResponse,Response};
use axum::routing::{get,post};

//use axum::response::Html;

//use std::net::SocketAddr;

mod stockprice;
mod phantom;

use stockprice::{get_stockprice_handler, post_stockprice_handler, post_stockprice_handler_2};

#[tokio::main]
async fn main() {
    let addr = "127.0.0.1:5050";

    println!("Server listening on {addr:?}\n");

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();

    axum::serve(listener, router()).await.unwrap();
}

fn router() -> Router {
    Router::new()
        .route("/hello", get(get_handler_2).post(post_handler))
        .route(
            "/price",
            get(get_stockprice_handler).post(post_stockprice_handler),
        )
        .route("/price2", post(post_stockprice_handler_2))
}

async fn hello_world() -> &'static str {
    "Hello from Axum, here we come, stockmarket!"
}

#[debug_handler]
async fn post_handler() -> impl IntoResponse {
    (StatusCode::CREATED, "Post Created!")
}

#[debug_handler]
async fn get_handler_2() -> Response {
    let pwd = phantom::PasswordManager::new(String::from("secret")); 
    println!("VERSION: {}", pwd.version());
    let pwd2 = pwd.unlock(String::from("secret"));
    println!("VERSION: {}", pwd2.version());

    println!("{}", pwd2.demo());
    Response::builder()
        .status(StatusCode::CREATED)
        .header("Content-Type", "application/json")
        .body(Body::from(r#"{"dude": "Donald Dux"}"#))
        .unwrap()
}
//get(|| async { Html("hello <strong>World!!!</strong>") }),
