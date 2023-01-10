mod crypto;
mod crypto_decode;
mod deep;

use deep::{get_infustructure, BinanceSpotOrderBookSnapshot, DepthRow, Event, LevelEvent};
use futures_util::StreamExt;
// use serde::de::Error;
use std::{borrow::Cow, ops::Deref, time::Instant};
use tokio_tungstenite::connect_async;
use url::Url;
const LEVEL_DEPTH_URL: &str = "wss://stream.binance.com:9443/ws/bnbbtc@depth20@100ms";
// const MAX_BUFFER: usize = 30;
#[tokio::main]
async fn main() {
    crypto::runner::send_http_request().await;

    loop {
        let url = Url::parse(LEVEL_DEPTH_URL).expect("Bad URL");
        let instance = Instant::now();
        let res = connect_async(url).await;
        let mut stream = match res {
            Ok((stream, _)) => stream,
            Err(_) => return,
        };

        println!("now {}", instance.elapsed().as_millis());
        while let Ok(msg) = stream.next().await.unwrap() {
            //

            if !msg.is_text() {
                continue;
            }

            let text = msg.into_text().unwrap();

            let level_event: LevelEvent = match serde_json::from_str(&text) {
                Ok(e) => e,
                Err(_) => continue,
            };
            println!(
                "now2 {} {:?} ",
                instance.elapsed().as_millis(),
                level_event.last_update_id,
            );
        }
    }
}

#[test]
fn event_test() {
    use std::io::Read;
    let mut f = std::fs::File::open("text").unwrap();
    let mut text = String::new();
    f.read_to_string(&mut text).unwrap();
    // println!("{}", text);

    let event: Event = serde_json::from_str(&text).unwrap();
    println!("{:#?}", event);
}

#[test]
fn http_snapshot() {
    use std::collections::VecDeque;
    let mut buffer = VecDeque::new();

    for i in 0..3 {
        buffer.push_back(i);
    }

    let mut b = buffer.clone();

    let mut c = buffer
        .clone()
        .iter()
        .map(|x| x + 1)
        .collect::<VecDeque<_>>();

    c.append(&mut b);

    while let Some(i) = c.pop_front() {
        println!("{}", i);
    }
    println!("b.len {}", b.len());
}

#[test]
fn time_stamp() {
    use std::time::{SystemTime, UNIX_EPOCH};
    let time = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();

    println!("{}", time.as_millis());
}

#[test]
fn while_let() {
    let mut v = vec![0, 1, 2, 3];

    while let Some(s) = v.pop() {
        if s == 2 {
            break;
        }
    }
    println!("{:?}", v);
}

#[test]
fn sender_receiver() {
    use tokio::{
        sync::mpsc::{self, Receiver},
        time::{sleep, Duration},
    };

    use tokio::runtime::Runtime;

    let rt = Runtime::new().unwrap();

    rt.block_on(async {
        let (tx, mut rx) = mpsc::channel(3);

        tokio::spawn(async move {
            for i in 0..5 {
                tx.send(i).await.unwrap();
                println!("send {}", i);
            }
        });

        tokio::spawn(async move {
            while let Some(i) = rx.recv().await {
                println!("rec {}", i);
                sleep(Duration::from_millis(1000)).await;
            }
        });
    });
}

#[test]
fn join_handle_useage() {
    // use tokio::{
    //     time::{sleep, Duration},
    //     sync::mpsc::{self, Receiver},
    // };

    use tokio::runtime::Runtime;
    use tokio::task::spawn_blocking;

    async fn number() -> i32 {
        6
    }

    let rt = Runtime::new().unwrap();

    let _ = rt.block_on(async {
        let res = tokio::spawn(async { number().await });

        res.await
    });
}

#[test]
fn split_contract() {
    let s1 = String::from("btcusdt_swap");
    let s2 = String::from("btcusd_221230_swap");
    let s3 = String::from("bnbbtc");
    fn valid_symbol(symbol: &str) -> bool {
        symbol.split("_").collect::<Vec<_>>().len() <= 3
    }
    let v1 = s1.split("_swap").collect::<Vec<_>>();

    let v2 = s2.split("_swap").collect::<Vec<_>>();
    let v3 = s3.split("_swap").collect::<Vec<_>>();

    println!("{:?}", v1);
    println!("{:?}", v2);
    println!("{:?}", v3);

    println!("{:?}", valid_symbol(&s1));
    println!("{:?}", valid_symbol(&s2));
    println!("{:?}", valid_symbol(&s3));
}

#[test]
fn none_or_error_question_mark() {

    // fn question_mark_function() -> Result<String, String>{
    //     let a = Some(1);
    //     let b: Result<String, String> = Ok("2".to_string());
    //     let res1 = a.or();
    //     b
    // }
}

#[test]
fn decode() {
    use std::fs::OpenOptions;
    use std::io::Read;
    {
        let mut reader = OpenOptions::new().read(true).open("crypto").unwrap();
        let mut buffer = String::new();
        assert!(reader.read_to_string(&mut buffer).is_ok());
        let res: crypto_decode::LevelEventStream = serde_json::from_str(&buffer).unwrap();
    }

    {
        let mut reader = OpenOptions::new().read(true).open("depth").unwrap();
        let mut buffer = String::new();
        assert!(reader.read_to_string(&mut buffer).is_ok());
        let res: deep::DepthRow = serde_json::from_str(&buffer).unwrap();
        println!("{:?}", res);
    }

    {
        let mut reader = OpenOptions::new().read(true).open("abc").unwrap();
        let mut buffer = String::new();
        assert!(reader.read_to_string(&mut buffer).is_ok());
        let res: crypto_decode::Quotes = serde_json::from_str(&buffer).unwrap();
        println!("{:?}", res);
    }
}

#[test]
fn dynamic_box() {

    // use tokio::sync::mpsc::UnboundedReceiver;
    // use tokio::sync::mpsc;
    // use tokio::runtime::Runtime;

    // trait Moew{
    //     fn moew(&self) -> String{
    //         String::from("noew")
    //     }
    // }
    // struct AmeShort;

    // struct Orange;

    // impl Moew for AmeShort{}

    // impl Moew for Orange{}

    // fn be_cat() -> Box<dyn Moew>{
    //     let a = AmeShort;
    //     Box::new(a)
    // }

    // async fn cat_in_a_box() -> Option<UnboundedReceiver<Box<dyn Moew>>>{
    //     let (mut sender, receiver) = mpsc::unbounded_channel();
    //     let a_cat = AmeShort;
    //     sender.send(Box::new(a_cat));

    //     Some(receiver)
    // }

    // let rt = Runtime::new().unwrap();
    // rt.block_on(async{

    //     let _ = be_cat().moew();
    // });
}

#[test]
fn csv_example() -> Result<(), Box<dyn std::error::Error>> {
    use csv::Writer;
    use serde::Serialize;

    #[derive(Serialize)]
    struct Row<'a> {
        city: &'a str,
        country: &'a str,
        population: i64,
    }
    let i = 5;
    let mut wtr = Writer::from_path("abc")?;
    // let string_city = format!("a,{i}", );
    let city = "a {i}";
    for _ in 0..5 {
        let r2 = Row {
            city,
            country: "b",
            population: 10,
        };
        wtr.serialize(r2)?;
        wtr.flush()?;
    }

    Ok(())
}

#[test]
fn trait_example() {
    #[derive(Debug, Clone, Copy)]
    pub struct Quote {
        pub price: f64,
        pub amount: f64,
    }
}
