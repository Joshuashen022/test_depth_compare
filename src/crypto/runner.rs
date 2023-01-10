use super::decoder::*;
use super::maintain_key::*;
use super::crypto_deocder::*;
use futures_util::SinkExt;
use futures_util::StreamExt;

use tokio_tungstenite::connect_async;
use tokio_tungstenite::tungstenite::Message;
use url::Url;
// const CRYPTO_TRADE_WSS1: &str = "ws://p38.crypto.local:8080/v2/user";
const CRYPTO_WEBSOCKET_WSS2: &str = "wss://stream.crypto.com/v2/user";
// const TRADE_URL_SPOT: &str = "https://api.binance.com";
const BINANCE_SPOT_WEBSOCKET_ENDPOINT: &str = "wss://stream.binance.com:9443/ws/";
// const TRADE_URL_SPOT_1: &str =    "https://api1.binance.com";
// const API_ORDER_TEST : &str = "/api/v3/order/test";
// const API_ORDER : &str = "/api/v3/order";
// const API_OPEN_ORDERS : &str = "/api/v3/openOrders";
// const API_ALL_ORDERS : &str = "/api/v3/allOrders";
// const API_USER_DATA_STREAM : &str = "/api/v3/userDataStream";
pub const ACCESS_KEY: &str = "cs2ZGo3aWaf7dJYD9CLTX7";
pub const SECRET_KEY: &str = "jZbRjKQYKPLXE28tBYCBQw";

// pub const ACCESS_KEY: &str = "nifNGIXIzco8YXe3PpuD0zMXvJN33WpWdNNxHl1GLb1JIS5n9TttdcIxlZnHQhGA";
// pub const SECRET_KEY: &str = "atl3kPizvOkgM366O2OPbotuQpbWIxH2M4IEbvAwwqxey6amjKODfb0mBsVNpji1";

pub async fn send_request() {
    let instrument_name = "USD_USDT";
    let is_buy = true;
    let amount = "10";
    let price = "1";
    let client_oid = "3a941ae3-d1b8-4889-8aff-777a78529ce5";
    let is_maker = false;

    let param = CreateOrder::new(instrument_name, is_buy, price, amount, client_oid, is_maker);
    let method = "private/create-order";
    let id = 11;
    let mut req = CryptoRequest::new(method, id, param);
    req.sign(ACCESS_KEY, SECRET_KEY);
    println!("Crypto request {:?}", req);

    let address = CRYPTO_WEBSOCKET_WSS2.to_string();
    println!("url {:?}", address);

    let url = Url::parse(&address).expect("Bad URL");
    let mut stream = match connect_async(url).await {
        Ok((connection, _)) => Ok(connection),
        Err(e) => Err(format!("{:?}", e)),
    }
    .unwrap();

    println!("connection success");

    let message = serde_json::to_string(&req).unwrap();
    stream.send(Message::Text(message)).await.unwrap();

    while let Ok(message) = stream.next().await.unwrap() {
        match &message {
            Message::Ping(inner) => {
                println!("Receive ping {:?}", inner);
                stream.send(Message::Pong(inner.clone())).await.unwrap();
            }
            Message::Text(inner) => {
                println!("{}", inner);
            }
            _ => println!("unknown message type: {:?}", message),
        }
    }

    // let after: EmptyRespond = serde_json::from_str(&res).unwrap();
    // println!("after {:?}", after);
    println!("Done");
}
