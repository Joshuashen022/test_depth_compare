mod deep;


use deep::{Event, BinanceSpotOrderBookSnapshot, get_infustructure};
use tokio_tungstenite::connect_async;
use url::Url;
// use tokio::net::TcpStream;
// use tokio::time::{sleep, Duration};
use futures_util::StreamExt;
use std::time::Instant;
// use anyhow::Result;
// use anyhow::anyhow;
const DEPTH_URL: &str = "wss://stream.binance.com:9443/ws/bnbbtc@depth@100ms";
// const MAX_BUFFER: usize = 30;
#[tokio::main]
async fn main(){

    loop{
        let url = Url::parse(DEPTH_URL).expect("Bad URL");
        let instance = Instant::now();
        let res = connect_async(url).await;
        let mut stream = match res{
            Ok((stream, _)) => stream,
            Err(e) => return ,
        };

        println!("now {}", instance.elapsed().as_millis());
        while let Ok(msg) = stream.next().await.unwrap(){ //
            println!("now1 {}", instance.elapsed().as_millis());
            if !msg.is_text() {
                continue
            }

            let text = msg.into_text().unwrap();

            let event: Event = match serde_json::from_str(&text){
                Ok(e) => e,
                Err(_) => continue,
            };
            println!("now2 {}", instance.elapsed().as_millis());
        };
    }
    
}

#[test]
fn event_test(){
    use std::io::Read;
    let mut f = std::fs::File::open("text").unwrap();
    let mut text = String::new();
    f.read_to_string(&mut text).unwrap();
    // println!("{}", text);

    let event: Event = serde_json::from_str(&text).unwrap();
    println!("{:#?}", event);

}

#[test]
fn http_snapshot(){
    use std::collections::VecDeque;
    let mut buffer = VecDeque::new();
    
    for i in 0..10{
        buffer.push_back(i);
    }

    while let Some(i) = buffer.pop_front() {
        println!("{}", i);
    }
}