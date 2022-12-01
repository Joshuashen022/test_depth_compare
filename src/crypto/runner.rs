use super::decoder::{ListenKey, EmptyRespond};
use reqwest;
use url::Url;

// const TRADE_URL_PC: &str =      "wss://dstream.binance.com/stream?streams=btcusd_221230@trade";
// const TRADE_URL_PU: &str =      "wss://fstream.binance.com/stream?streams=btcusdt@trade";
const TRADE_URL_SPOT: &str = "https://api.binance.com";
// const TRADE_URL_SPOT_1: &str =    "https://api1.binance.com";
// const API_ORDER_TEST : &str = "/api/v3/order/test";
// const API_ORDER : &str = "/api/v3/order";
// const API_OPEN_ORDERS : &str = "/api/v3/openOrders";
// const API_ALL_ORDERS : &str = "/api/v3/allOrders";
const API_USER_DATA_STREAM : &str = "/api/v3/userDataStream";
pub const ACCESS_KEY: &str = "nifNGIXIzco8YXe3PpuD0zMXvJN33WpWdNNxHl1GLb1JIS5n9TttdcIxlZnHQhGA";
pub const SECRET_KEY: &str = "atl3kPizvOkgM366O2OPbotuQpbWIxH2M4IEbvAwwqxey6amjKODfb0mBsVNpji1";


// 3lUeRcWmZF8qBgDVXZ7v2LZKGjsKmowAH2PZjhwww5LbdQKZ71PHVyC77a6b

pub async fn send_request() {
    let listen_key = "3lUeRcWmZF8qBgDVXZ7v2LZKGjsKmowAH2PZjhwww5LbdQKZ71PHVyC77a6b";
    let url_str = format!("{}{}", TRADE_URL_SPOT, API_USER_DATA_STREAM);
    let body = format!("listenKey={}", listen_key);
    let res = delete_key(url_str, body).await;

    println!("client {:?}", res);
    assert_eq!(res, "{}");
    // let after: EmptyRespond = serde_json::from_str(&res).unwrap();
    // println!("after {:?}", after);
    println!("Done");

}

// url: https://api.binance.com/api/v3/order
async fn delete_key(url: String, body: String) -> String {
    println!("url {:?}", url);
    let url = Url::parse(&url).expect("Bad URL");
    
    let client = reqwest::Client::new();
    client
        .delete(url)
        .header("X-MBX-APIKEY", ACCESS_KEY)
        .body(body)
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap()
}
