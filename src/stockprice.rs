#![allow(unused)]

use axum::extract::Query;
use axum::{Json, debug_handler};
use serde::{Serialize,Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct StockPrice {
    ticker: String,
    price: f32,
}

#[debug_handler]
pub async fn get_stockprice_handler() -> Json<StockPrice> {
    println!("get_stockprice_handler");
    Json::from(StockPrice {
        ticker: String::from("YAR"),
        price: 245.65,
    })
}

#[debug_handler]
pub async fn post_stockprice_handler(Json(p): Json<StockPrice>) {
    println!("get_stockprice_handler");
    println!("Ticker: {0}, Price: {1}", p.ticker, p.price);
}

#[debug_handler]
pub async fn post_stockprice_handler_2(Query(p): Query<StockPrice>) -> Json<StockPrice> {
    println!("get_stockprice_handler_2");
    println!("Ticker: {0}, Price: {1}", p.ticker, p.price);
    Json::from(p)
}
