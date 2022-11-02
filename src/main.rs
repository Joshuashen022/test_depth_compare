mod deep;


use deep::{Event, BinanceSpotOrderBookSnapshot};
// use tokio_tungstenite::connect_async;
// use url::Url;
// use tokio::net::TcpStream;
// use tokio::time::{sleep, Duration};
// use futures_util::StreamExt;
use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {


    println!("prepare for snap shot");
    let snapshot: BinanceSpotOrderBookSnapshot = reqwest::get("https://api.binance.com/api/v3/depth?symbol=BNBBTC&limit=1000")
    .await?
    .json()
    .await?;


    // abc
    // println!("{}", text);

    println!("{:#?}", snapshot);
    Ok(())
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

}