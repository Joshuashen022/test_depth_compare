use tokio_tungstenite::{connect_async, tungstenite};
use tungstenite::protocol::Message;
use futures_util::{SinkExt, StreamExt};
use url::Url;
use crate::crypto::ticker_decoder::{
    EventTicker
};

// const TRADE_URL_PC: &str =      "wss://dstream.binance.com/stream?streams=btcusd_221230@trade";
// const TRADE_URL_PU: &str =      "wss://fstream.binance.com/stream?streams=btcusdt@trade";
const TRADE_URL_SPOT: &str =    "wss://stream.binance.com:9443/ws/bnbbtc@trade";
pub async fn send_request(){
    let url = Url::parse(TRADE_URL_SPOT).expect("Bad URL");
    let mut stream = match connect_async(url).await {
        Ok((connection, _)) => connection,
        Err(e) => {
            println!("connection error {:?}", e);
            return ;
        }
    };

    println!("connection SUCCESS");

    while let Some(Ok(msg))= stream.next().await{
        println!("receive SUCCESS");
        
        if !msg.is_text() {
            println!("msg is empty");
            continue;
        }

        let text = match msg.clone().into_text() {
            Ok(e) => e,
            Err(e) => {
                println!("msg.into_text {:?}", e);
                continue;
            }
        };

        let response: EventTicker = match serde_json::from_str(&text) {
            Ok(response) => {
                // println!("Receive confirm message {:?}", response);
                response
            },
            Err(e) => {
                println!("Error {}, {:?}", e, msg);
                continue;
            }
        };

        println!("receive TradeEvent {:?}", response);

    }
    
    
}

