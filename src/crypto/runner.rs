use tokio_tungstenite::{connect_async, tungstenite};
use tungstenite::protocol::Message;
use futures_util::{SinkExt, StreamExt};
use url::Url;
use crate::crypto::decoder::{
    OrderResponse, 
    subscribe_message, heartbeat_respond,
    LevelEventStream, HeartbeatRequest,
    BookEvent, GeneralResponse
};



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
    let channel = String::from("book.BTC_USDT");
    let message = Message::from(subscribe_message(channel));
    
    match stream.send(message).await{
        Ok(()) => (),
        Err(e) => println!("{:?}",e ),
    };

    println!("send SUCCESS");

    while let Some(Ok(msg))= stream.next().await{
        
        
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

        let response: GeneralResponse = match serde_json::from_str(&text) {
            Ok(response) => {
                // println!("Receive confirm message {:?}", response);
                response
            },
            Err(e) => {
                println!("Error {}, {:?}", e, msg);
                continue;
            }
        };

        match (response.method.as_str(), response.id){
            ("public/heartbeat", _) => {
                // wrap this in a async fn returns Result<>
                // use ? to repplace match
            
                let heartbeat_request: HeartbeatRequest = match serde_json::from_str(&text) {
                    Ok(event) => {
                        event
                    },
                    Err(e) => {
                        println!("Error {}, {:?}", e, msg);
                        continue;
                    }
                };
                
                println!("Receive {:?}", heartbeat_request);

                let message = heartbeat_respond(heartbeat_request.id);
                match stream.send(message).await{
                    Ok(()) => (),
                    Err(e) => println!("{:?}",e ),
                };
                continue
            },
            ("subscribe", 1)=> {
                // wrap this in a async fn returns Result<>
                // use ? to repplace match
                let order_response: OrderResponse = match serde_json::from_str(&text) {
                    Ok(event) => {
                        event
                    },
                    Err(e) => {
                        println!("Error {}, {:?}", e, msg);
                        continue;
                    }
                };
                println!("Receive {:?}, initialize success", order_response);
                continue;
            }, // initialize
            ("subscribe", -1)=> (),// snapshot
            _ => {
                println!("Unknown respond {:?}", response);
                continue;
            },
        }

        let event: LevelEventStream<BookEvent> = match serde_json::from_str(&text) {
            Ok(event) => event,
            Err(e) => {
                println!("Error {}, {:?}", e, msg);
                continue;
            }
        };

        println!("receive BookEvent {:?}", event.data());

    }
    
    
}


#[test]
fn subscribe_message_out(){
    let channel = String::from("book.BTCUSD-PERP");
    println!("{}", subscribe_message(channel));
}