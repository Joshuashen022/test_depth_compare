use tokio_tungstenite::{connect_async, tungstenite};
use tungstenite::protocol::Message;
use reqwest;
use futures_util::{SinkExt, StreamExt};
use url::Url;
use super::decoder::BinanceOrder;

// const TRADE_URL_PC: &str =      "wss://dstream.binance.com/stream?streams=btcusd_221230@trade";
// const TRADE_URL_PU: &str =      "wss://fstream.binance.com/stream?streams=btcusdt@trade";
const TRADE_URL_SPOT: &str =    "https://api.binance.com";
// const TRADE_URL_SPOT_1: &str =    "https://api1.binance.com";
pub async fn send_request(){
    
    let api_a = "/api/v3/order/test";
    // let api_b = "/v3/order/test";
    let url_str = format!("{}{}",TRADE_URL_SPOT, api_a);
    let url = Url::parse(&url_str).expect("Bad URL");
    let client = reqwest::Client::new();
    let order = BinanceOrder::new();
    let body = serde_json::to_string(&order).unwrap();
    let res = client.post(url).body(body).send().await.unwrap();
    let head = res.headers().iter();
    println!("head {:?}", head);
    println!("Done");
}

