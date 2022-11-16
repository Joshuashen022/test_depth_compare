use ordered_float::OrderedFloat;
use serde::de::{SeqAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde::de::DeserializeOwned;
use std::collections::BTreeMap;
use std::fmt;
use std::time::{SystemTime, UNIX_EPOCH};
use tracing::debug;
use tokio_tungstenite::tungstenite;
use tungstenite::protocol::Message;

#[derive(Deserialize, Serialize, Debug)]
pub struct HeartbeatRequest{
    pub id: i64,
    pub method:String,
    pub code: i64,
}

#[derive(Deserialize, Serialize)]
pub struct HeartbeatRespond{
    pub id: i64,
   
    pub method:String,
}

pub fn heartbeat_respond(id: i64) -> Message{
    let inner = HeartbeatRespond{
        id,
        method: String::from("public/respond-heartbeat"),
    };
    let inner = serde_json::to_string(&inner).unwrap();
    Message::from(inner)
}

pub fn order_respond(id: i64) -> Message{
    let inner = HeartbeatRespond{
        id,
        method: String::from("public/respond-heartbeat"),
    };
    let inner = serde_json::to_string(&inner).unwrap();
    Message::from(inner)
}

#[derive(Deserialize, Serialize)]
pub struct OrderRequest{
    pub id: i64,
    pub method:String,
    pub params:Params,
}
#[derive(Deserialize, Serialize)]
pub struct Params{
    pub channels:Vec<String>
}

#[derive(Deserialize, Serialize, PartialEq, Eq, Debug)]
pub struct GeneralResponse{
    pub id: i64,
    pub code: i64,
    pub method: String,
}

//Text("{\"id\":1,\"code\":0,\"method\":\"subscribe\",\"channel\":\"book.BTCUSD-PERP\"}")
#[derive(Deserialize, Serialize, PartialEq, Eq, Debug)]
pub struct OrderResponse{
    pub id: i64,
    pub code: i64,
    pub method: String,
    pub channel: String,
}

pub fn subscribe_message(channel: String) -> String{
    let _time = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    let inner = OrderRequest{
        id: 1, 
        method: String::from("subscribe"),
        params: Params { 
            channels: vec![channel],
        },
    };
    serde_json::to_string(&inner).unwrap()
}

pub fn subscribe_success() -> bool{
    
    false
}


pub struct Shared {
    instrument: String,
    last_update_id: i64,
    send_time: i64,
    receive_time: i64,
    asks: BTreeMap<OrderedFloat<f64>, f64>,
    bids: BTreeMap<OrderedFloat<f64>, f64>,
}


#[derive(Deserialize, Debug)]
pub struct LevelEventStream<Event> {
    /// Usually constant value `-1`
    pub id: i64,

    /// Something like: "public/get-block"
    pub method: String,

    /// Usually constant value `0`
    pub code: i64,

    pub result: Event,
}


#[derive(Deserialize, Debug, Clone)]
pub struct TradeEvent{
    pub channel: String,

    pub subscription: String,

    /// Something like "BTC_USDT"
    pub instrument_name: String,

    pub data: Vec<TradeData>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct TradeData{
    #[serde(rename = "s")]
    pub side: String,

    #[serde(rename = "p")]
    pub price: String,

    #[serde(rename = "q")]
    pub quantity: String,
    
    #[serde(rename = "t")]
    pub trade_time: i64,

    #[serde(rename = "d")]
    pub trade_id: String,

    #[serde(rename = "i")]
    pub instrument_name: String,
}


#[derive(Deserialize, Debug, Clone)]
pub struct BookEvent {

    pub channel: String,

    pub subscription: String,

    /// Something like "BTC_USDT"
    pub instrument_name: String,

    pub data: Vec<Data>,

    /// Usually constant value `20` or `50`
    pub depth: i64,

}

#[derive(Deserialize, Debug, Clone)]
pub struct Data {
    /// Some timestamp server tells us
    #[serde(rename = "t")]
    pub publish_time: i64,

    #[serde(rename = "tt")]
    pub last_update_time: i64,

    #[serde(rename = "u")]
    pub update_sequence: i64,
    
    #[serde(rename = "cs")]
    pub other: i64,

    pub asks: Vec<Quotes>,

    pub bids: Vec<Quotes>,
}

#[derive(Debug, Copy, Clone)]
pub struct Quotes {
    price: f64,
    amount: f64,
    order_numbers: i64,
}

impl<'de> Deserialize<'de> for Quotes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_tuple(3, QuotesVisitor)
    }
}

struct QuotesVisitor;

impl<'de> Visitor<'de> for QuotesVisitor {
    type Value = Quotes;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "a map with keys 'first' and 'second'")
    }

    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
    where
        A: SeqAccess<'de>,
    {
        let mut price = None;
        let mut amount = None;
        let mut order_numbers = None;

        if let Some(val) = seq.next_element::<&str>()? {
            match val.parse::<f64>() {
                Ok(num) => price = Some(num),
                Err(_) => return Err(serde::de::Error::custom("Fail to convert price str to f64")),
            }
        }

        if let Some(val) = seq.next_element::<&str>()? {
            match val.parse::<f64>() {
                Ok(num) => amount = Some(num),
                Err(_) => {
                    return Err(serde::de::Error::custom(
                        "Fail to convert amount str to f64",
                    ))
                }
            }
        }

        if let Some(val) = seq.next_element::<&str>()? {
            match val.parse::<i64>() {
                Ok(num) => order_numbers = Some(num),
                Err(_) => {
                    return Err(serde::de::Error::custom(
                        "Fail to convert order_numbers str to u64",
                    ))
                }
            }
        }

        if price.is_none() {
            return Err(serde::de::Error::custom("Missing price field"));
        }

        if amount.is_none() {
            return Err(serde::de::Error::custom("Missing amount field"));
        }

        if order_numbers.is_none() {
            return Err(serde::de::Error::custom("Missing order_numbers field"));
        }

        Ok(Quotes {
            price: price.unwrap(),
            amount: amount.unwrap(),
            order_numbers: order_numbers.unwrap(),
        })
    }
}
