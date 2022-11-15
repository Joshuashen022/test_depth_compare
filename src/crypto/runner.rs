use tokio_tungstenite::{connect_async, tungstenite};
use tungstenite::protocol::Message;
use futures_util::{SinkExt, StreamExt};
use url::Url;
use crate::crypto::decoder::{OrderResponse, subscribe_message, LevelEventStream};



pub async fn send_request(){
    
    const LEVEL_DEPTH_URL: &str = "wss://stream.crypto.com/v2/market";
    let url = Url::parse(LEVEL_DEPTH_URL).expect("Bad URL");
    let mut stream = match connect_async(url).await {
        Ok((connection, _)) => connection,
        Err(e) => {
            println!("connection error {:?}", e);
            return ;
        }
    };

    println!("connection SUCCESS");
    let channel = String::from("book.BTCUSD-PERP");
    let message = Message::from(subscribe_message(channel));
    
    match stream.send(message).await{
        Ok(()) => (),
        Err(e) => println!("{:?}",e ),
    };

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

        let level_event: LevelEventStream = match serde_json::from_str(&text) {
            Ok(event) => event,
            Err(e) => {
                println!("Error {}, {:?}", e, msg);
                continue;
            }
        };

        // let response = OrderResponse{id:0,code:0,method:String::from("subscribe")};
        
        level_event.debug();
        
        // if level_event.method == response.method {
        //     println!("yes!");
        //     break
        // }

    
    };

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
        
        if level_event.method == response.method {
            println!("yes!");
            break
        }

    
    };
    
}


#[test]
fn subscribe_message_out(){
    let channel = String::from("book.BTCUSD-PERP");
    println!("{}", subscribe_message(channel));
}