#![allow(unused)]

use axum::body::Body;
use axum::http::StatusCode;
use axum::response::{Html, IntoResponse, Response};
use axum::routing::{get, post};
use axum::{Json, Router, debug_handler};
use clap::Parser;
use std::net::SocketAddr;

//use axum::response::Html;

//use std::net::SocketAddr;

mod phantom;
mod stockoptions;
mod stockprice;
//mod yfinance;

#[derive(Parser)]
struct Args {
    #[arg(short, long, default_value = "3000")]
    port: u16,
}

use stockprice::{get_stockprice_handler, post_stockprice_handler, post_stockprice_handler_2};

use crate::stockoptions::{fetch_option_prices, fetch_option_prices_proxy};

#[tokio::main]
async fn main() {
    /*
    let nhy = stockoptions::fetch_option_prices("NHY").await.unwrap();
    let yar = stockoptions::fetch_option_prices("YAR").await.unwrap();
    let eqn = stockoptions::fetch_option_prices("EQN").await.unwrap();
    */
    let yar = stockoptions::fetch_option_prices("YAR").await.unwrap();

    let args = Args::parse();
    let addr = SocketAddr::from(([127, 0, 0, 1], args.port));

    println!("Server listening on {addr:?}\n");

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, router()).await.unwrap();

    /*
    let addr = "127.0.0.1:5050";

    println!("Server listening on {addr:?}\n");

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();

    axum::serve(listener, router()).await.unwrap();
    */
}

fn router() -> Router {
    Router::new()
        .route("/hello", get(get_handler_2).post(post_handler))
        .route(
            "/price",
            get(get_stockprice_handler).post(post_stockprice_handler),
        )
        .route("/price2", post(post_stockprice_handler_2))
        .route("/proxy/{ticker}", get(fetch_option_prices_proxy))
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
