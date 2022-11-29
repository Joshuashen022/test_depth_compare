use tokio_tungstenite::{connect_async, tungstenite};

use sha2::Sha256;
use hmac::{Hmac, Mac};
use reqwest;
use futures_util::{SinkExt, StreamExt};
use url::Url;
use super::decoder::BinanceOrder;

// const TRADE_URL_PC: &str =      "wss://dstream.binance.com/stream?streams=btcusd_221230@trade";
// const TRADE_URL_PU: &str =      "wss://fstream.binance.com/stream?streams=btcusdt@trade";
const TRADE_URL_SPOT: &str =    "https://api.binance.com";
// const TRADE_URL_SPOT_1: &str =    "https://api1.binance.com";

const ACCESS_KEY: &str = "nifNGIXIzco8YXe3PpuD0zMXvJN33WpWdNNxHl1GLb1JIS5n9TttdcIxlZnHQhGA";
const SECRET_KEY: &str = "atl3kPizvOkgM366O2OPbotuQpbWIxH2M4IEbvAwwqxey6amjKODfb0mBsVNpji1";

type HmacSha256 = Hmac<Sha256>;


pub async fn send_request(){
    
    let api_a = "/api/v3/order/test?";
    // let api_b = "/v3/order/test?";
    
    let client = reqwest::Client::new();
    let order = BinanceOrder::new_default();
    
    let body = order.into_str();
    let url_str = format!("{}{}",TRADE_URL_SPOT, api_a);

    let url = Url::parse(&url_str).expect("Bad URL");

    let mut mac = HmacSha256::new_from_slice(SECRET_KEY.as_bytes()).unwrap();
    mac.update(body.as_bytes());
    let res = mac.finalize().into_bytes();
    
    let res = client.post(url).header("X-MBX-APIKEY", ACCESS_KEY)
    .body(body)
    .send().await.unwrap()
    .text().await.unwrap();
    // let head = res.headers().iter();
    // for v in head{
    //     println!("head {:?}", v);
    // }
    
    println!("client {:?}", res);
    println!("Done");
}

