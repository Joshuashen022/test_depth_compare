use super::decoder::{BinanceCheckAllOrder, BinanceCheckAllOrderResponse};
use reqwest;
use url::Url;

// const TRADE_URL_PC: &str =      "wss://dstream.binance.com/stream?streams=btcusd_221230@trade";
// const TRADE_URL_PU: &str =      "wss://fstream.binance.com/stream?streams=btcusdt@trade";
const TRADE_URL_SPOT: &str = "https://api.binance.com";
// const TRADE_URL_SPOT_1: &str =    "https://api1.binance.com";
// const API_ORDER_TEST : &str = "/api/v3/order/test";
// const API_ORDER : &str = "/api/v3/order";
// const API_OPEN_ORDERS : &str = "/api/v3/openOrders";
const API_ALL_ORDERS : &str = "/api/v3/allOrders";
pub const ACCESS_KEY: &str = "nifNGIXIzco8YXe3PpuD0zMXvJN33WpWdNNxHl1GLb1JIS5n9TttdcIxlZnHQhGA";
pub const SECRET_KEY: &str = "atl3kPizvOkgM366O2OPbotuQpbWIxH2M4IEbvAwwqxey6amjKODfb0mBsVNpji1";

pub async fn send_request() {

    let url_str = format!("{}{}", TRADE_URL_SPOT, API_ALL_ORDERS);
    
    let order = BinanceCheckAllOrder::new();
    let body = order.get_body();

    let res = check_order(url_str, body).await;

    println!("client {:?}", res);

    let after: BinanceCheckAllOrderResponse = serde_json::from_str(&res).unwrap();
    println!("after {:?}", after);
    println!("Done");

}

// url: https://api.binance.com/api/v3/order
async fn check_order(url: String, body: String) -> String {
    let quarry_string = format!("{}?{}", url, body);
    println!("quarry_string: {}", quarry_string);
    let url = Url::parse(&quarry_string).expect("Bad URL");
    let client = reqwest::Client::new();
    client
        .get(url)
        .header("X-MBX-APIKEY", ACCESS_KEY)
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap()
}
