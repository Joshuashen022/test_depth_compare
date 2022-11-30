use super::decoder::{BinanceOrder, Hasher, BinanceOrderResponse};
use reqwest;
use url::Url;

// const TRADE_URL_PC: &str =      "wss://dstream.binance.com/stream?streams=btcusd_221230@trade";
// const TRADE_URL_PU: &str =      "wss://fstream.binance.com/stream?streams=btcusdt@trade";
const TRADE_URL_SPOT: &str = "https://api.binance.com";
// const TRADE_URL_SPOT_1: &str =    "https://api1.binance.com";
// const API_ORDER_TEST : &str = "/api/v3/order/test";
const API_ORDER : &str = "/api/v3/order";
const ACCESS_KEY: &str = "nifNGIXIzco8YXe3PpuD0zMXvJN33WpWdNNxHl1GLb1JIS5n9TttdcIxlZnHQhGA";
const SECRET_KEY: &str = "atl3kPizvOkgM366O2OPbotuQpbWIxH2M4IEbvAwwqxey6amjKODfb0mBsVNpji1";

pub async fn send_request() {
    let client = reqwest::Client::new();

    let url_str = format!("{}{}", TRADE_URL_SPOT, API_ORDER,);
    let url = Url::parse(&url_str).expect("Bad URL");
    let order = BinanceOrder::new_default();
    let params = order.into_string();
    
    let hasher = Hasher{
        api_key: ACCESS_KEY.to_string(),
        secret_key: SECRET_KEY.to_string(),
        raw_message: params.clone(),
    };

    let hash = hasher.hash();

    let body = format!("{}&signature={}", params, hash);
    
    println!("url: {}", url_str);
    println!("body: {}", body);

    let res = client
        .post(url)
        .header("X-MBX-APIKEY", ACCESS_KEY)
        .body(body)
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();

    println!("client {:?}", res);

    let after: BinanceOrderResponse = serde_json::from_str(&res).unwrap();
    println!("after {:?}", after);
    println!("Done");
}
