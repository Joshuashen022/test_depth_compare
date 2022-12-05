use super::decoder::{
    BinanceOrderUpdatePayload, OutboundAccountPositionPayload, PayloadType,
};
use super::maintain_key::send_get_key_request;
use futures_util::SinkExt;
use futures_util::StreamExt;
use reqwest;
use tokio_tungstenite::connect_async;
use tokio_tungstenite::tungstenite::Message;
use url::Url;
// const TRADE_URL_PC: &str =      "wss://dstream.binance.com/stream?streams=btcusd_221230@trade";
// const TRADE_URL_PU: &str =      "wss://fstream.binance.com/stream?streams=btcusdt@trade";
const TRADE_URL_SPOT: &str = "https://api.binance.com";
const BINANCE_SPOT_WEBSOCKET_ENDPOINT: &str = "wss://stream.binance.com:9443/ws/";
// const TRADE_URL_SPOT_1: &str =    "https://api1.binance.com";
// const API_ORDER_TEST : &str = "/api/v3/order/test";
// const API_ORDER : &str = "/api/v3/order";
// const API_OPEN_ORDERS : &str = "/api/v3/openOrders";
// const API_ALL_ORDERS : &str = "/api/v3/allOrders";
// const API_USER_DATA_STREAM : &str = "/api/v3/userDataStream";
pub const ACCESS_KEY: &str = "nifNGIXIzco8YXe3PpuD0zMXvJN33WpWdNNxHl1GLb1JIS5n9TttdcIxlZnHQhGA";
pub const SECRET_KEY: &str = "atl3kPizvOkgM366O2OPbotuQpbWIxH2M4IEbvAwwqxey6amjKODfb0mBsVNpji1";

pub async fn send_request() {

    let listen_key = send_get_key_request().await;
    println!("getting key {:?}", listen_key);

    let address = format!("{}{}", BINANCE_SPOT_WEBSOCKET_ENDPOINT, listen_key);
    println!("url {:?}", address);

    let url = Url::parse(&address).expect("Bad URL");
    let mut stream = match connect_async(url).await {
        Ok((connection, _)) => Ok(connection),
        Err(e) => Err(format!("{:?}", e)),
    }
    .unwrap();

    println!("connection success");

    while let Ok(message) = stream.next().await.unwrap() {
        match &message {
            Message::Ping(inner) => {
                println!("Receive ping {:?}", inner);
                stream.send(Message::Pong(inner.clone())).await.unwrap();
            }
            Message::Text(inner) => {
                let event: PayloadType = match serde_json::from_str(inner) {
                    Ok(e) => e,
                    Err(e) => {
                        println!("{:?}", e);
                        continue;
                    }
                };
                match event.event_type.as_str() {
                    "outboundAccountPosition" => {
                        let _payload: OutboundAccountPositionPayload =
                            match serde_json::from_str(inner) {
                                Ok(e) => e,
                                Err(e) => {
                                    println!(
                                        "Deserialize OutboundAccountPositionPayload error {:?}",
                                        e
                                    );
                                    continue;
                                }
                            };
                    }
                    "executionReport" => {
                        let payload: BinanceOrderUpdatePayload = match serde_json::from_str(inner) {
                            Ok(e) => e,
                            Err(e) => {
                                println!(
                                    "Deserialize BinanceOrderUpdatePayload error {:?}",
                                    e
                                );
                                continue;
                            }
                        };
                        if let Ok((trade, order)) =  payload.clone().into_trade_and_order_info(){
                            println!("trade info {:?}", trade);
                            println!("order info {:?}", order);
                        } else {
                            println!("get trade and order info error {:?}", payload);
                        }
                    }
                    _ => println!("unknown event type: {:?}", event.event_type),
                }
            }
            _ => println!("unknown message type: {:?}", message),
        }
    }

    // let after: EmptyRespond = serde_json::from_str(&res).unwrap();
    // println!("after {:?}", after);
    println!("Done");
}
