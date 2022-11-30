use super::decoder::{BinanceOrder, Hasher};
use futures_util::{SinkExt, StreamExt};
use hex::encode;
use hmac::{Hmac, Mac};
use reqwest;
use sha2::Sha256;
use tokio_tungstenite::{connect_async, tungstenite};
use url::Url;

// const TRADE_URL_PC: &str =      "wss://dstream.binance.com/stream?streams=btcusd_221230@trade";
// const TRADE_URL_PU: &str =      "wss://fstream.binance.com/stream?streams=btcusdt@trade";
const TRADE_URL_SPOT: &str = "https://api.binance.com";
// const TRADE_URL_SPOT_1: &str =    "https://api1.binance.com";

const ACCESS_KEY: &str = "nifNGIXIzco8YXe3PpuD0zMXvJN33WpWdNNxHl1GLb1JIS5n9TttdcIxlZnHQhGA";
const SECRET_KEY: &str = "atl3kPizvOkgM366O2OPbotuQpbWIxH2M4IEbvAwwqxey6amjKODfb0mBsVNpji1";

type HmacSha256 = Hmac<Sha256>;

pub async fn send_request() {
    let api_a = "/api/v3/order/test?";
    // let api_b = "/v3/order/test?";

    let client = reqwest::Client::new();
    let order = BinanceOrder::new_default();

    let body = order.into_string();
    // let body = String::new();
    let hasher = Hasher{
        api_key: ACCESS_KEY.to_string(),
        secret_key: SECRET_KEY.to_string(),
        raw_message: body.clone(),
    };

    let hash = hasher.hash();

    let url_str = format!("{}{}{}&signature={}", TRADE_URL_SPOT, api_a, body, hash);
    // let url_str = format!("{}{}{}&signature={}", TRADE_URL_SPOT, api_a, body, hash);
    println!("url: {}", url_str);
    let url = Url::parse(&url_str).expect("Bad URL");
    let res = client
        .post(url)
        .header("X-MBX-APIKEY", ACCESS_KEY)
        // .body(body)
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();
    // let head = res.headers().iter();
    // for v in head{
    //     println!("head {:?}", v);
    // }

    println!("client {:?}", res);
    println!("Done");
}
