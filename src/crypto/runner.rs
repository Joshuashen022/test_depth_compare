use super::decoder::{ListenKey, BinanceOrderUpdatePayload, OutboundAccountPositionPayload, PayloadType};
use reqwest;
use url::Url;
use tokio_tungstenite::connect_async;
use tokio_tungstenite::tungstenite::Message;
use futures_util::StreamExt;
use futures_util::SinkExt;
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


// 3lUeRcWmZF8qBgDVXZ7v2LZKGjsKmowAH2PZjhwww5LbdQKZ71PHVyC77a6b

pub async fn send_request() {
    let listen_key = "TkpK82Ab5yGqEqY71MH2OF78HvbmEMZwvtbXVw8SiibyNUw20iT1aTDR1FLA";

    let address = format!("{}{}", BINANCE_SPOT_WEBSOCKET_ENDPOINT, listen_key);
    println!("url {:?}", address);

    let url = Url::parse(&address).expect("Bad URL");
    let mut stream = match connect_async(url).await {
        Ok((connection, _)) => Ok(connection),
        Err(e) => Err(format!("{:?}", e)),
    }.unwrap();

    println!("connection success");

    while let Ok(message) = stream.next().await.unwrap() {
        
        match &message{
            Message::Ping(inner) => {
                println!("Receive ping {:?}", inner);
                stream.send(Message::Pong(inner.clone())).await.unwrap();
            },
            Message::Text(inner) => {
                let event: PayloadType = match serde_json::from_str(inner){
                    Ok(e) => e,
                    Err(e) => {
                        println!("{:?}",e);
                        continue;
                    }
                };
                match event.event_type.as_str(){
                    "outboundAccountPosition" => {
                        let event: OutboundAccountPositionPayload = match serde_json::from_str(inner){
                            Ok(e) => e,
                            Err(e) => {
                                println!("{:?}",e);
                                continue;
                            }
                        };
                        println!("{:?}", event)
                    },
                    "executionReport" => {
                        let event: BinanceOrderUpdatePayload = match serde_json::from_str(inner){
                            Ok(e) => e,
                            Err(e) => {
                                println!("{:?}",e);
                                continue;
                            }
                        };
                        println!("{:?}", event)
                    },
                    _ => println!("unknown event type: {:?}", inner),
                }
            },
            _ => println!("unknown message type: {:?}", message),
        }
        
        
    }
    
    // let after: EmptyRespond = serde_json::from_str(&res).unwrap();
    // println!("after {:?}", after);
    println!("Done");

}
