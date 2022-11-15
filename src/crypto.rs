use tokio_tungstenite::{connect_async, connect_async_with_config, tungstenite};
use tungstenite::handshake::client::Request;
use tungstenite::client::IntoClientRequest;
use tungstenite::protocol::Message;
use http::{Request as HttpRequest, method};
use futures_util::{Sink, SinkExt, StreamExt};
use url::Url;
use serde::{Deserialize, Serialize};
use std::pin::Pin;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Deserialize, Serialize)]
pub struct OrderRequest{
    id: i64,
    method:String,
    params:Params,
    nonce: i64,
}

#[derive(Deserialize, Serialize, PartialEq, Eq, Debug)]
pub struct OrderResponse{
    id: i64,
    code: i64,
    method: String,
}


#[derive(Deserialize, Serialize)]
struct Params{
    channels:Vec<String>
}

fn subscribe_message() -> String{
    let time = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    let inner = OrderRequest{
        id: 11, 
        method: String::from("SUBSCRIBE"),
        params: Params { 
            channels: vec![String::from("user.order.ETH_CRO")],
        },
        nonce: time.as_millis() as i64,
    };
    serde_json::to_string(&inner).unwrap()
}

pub async fn send_request(){
    
    const LEVEL_DEPTH_URL: &str = "wss://stream.crypto.com/v2/market";
    let url = Url::parse(LEVEL_DEPTH_URL).expect("Bad URL");
    let mut stream = match connect_async(url).await {
        Ok((connection, responce)) => connection,
        Err(e) => {
            println!("connection error {:?}", e);
            return ;
        }
    };

    println!("connection SUCCESS");

    let message = Message::from(subscribe_message());
    // let _ = Pin::new(&mut stream).start_send(message);
    
    stream.send(message).await.unwrap();

    println!("send SUCCESS");

    while let Ok(msg) = stream.next().await.unwrap() {
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

        let level_event: OrderResponse = match serde_json::from_str(&text) {
            Ok(event) => event,
            Err(e) => {
                println!("Error {}, {:?}", e, msg);
                continue;
            }
        };

        let response = OrderResponse{id:0,code:0,method:String::from("subscribe")};
        
        println!("{:?}", level_event);
        
        if level_event == response {
            println!("yes!")
        }

    
    };
    
}


#[test]
fn subscribe_message_out(){
    println!("{}", subscribe_message());
}