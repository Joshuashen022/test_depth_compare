use anyhow::{Error, Result};
use serde::ser::SerializeTuple;
use serde::{de::SeqAccess, de::Visitor, Deserialize, Deserializer, Serialize, Serializer};
use std::fmt;
#[derive(Deserialize, Serialize, Debug)]
pub struct Event {
    #[serde(rename = "e")]
    pub ttype: String,
    #[serde(rename = "E")]
    pub ts: i64,
    #[serde(rename = "s")]
    pub pair: String,
    #[serde(rename = "U")]
    pub first_update_id: i64,
    #[serde(rename = "u")]
    pub last_update_id: i64,
    #[serde(rename = "b")]
    pub bids: Vec<DepthRow>,
    #[serde(rename = "a")]
    pub asks: Vec<DepthRow>,
}

#[derive(Deserialize, Debug)]
pub struct LevelEvent {
    #[serde(rename = "lastUpdateId")]
    pub last_update_id: i64,

    pub bids: Vec<DepthRow>,

    pub asks: Vec<DepthRow>,
}

#[derive(Debug)]
pub struct DepthRow {
    pub price: f64,
    pub amount: f64,
}

impl<'de> Deserialize<'de> for DepthRow {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_tuple(2, DepthRowVisitor)
    }
}

impl Serialize for DepthRow {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut seq = serializer.serialize_tuple(2)?;
        seq.serialize_element(&self.price)?;
        seq.serialize_element(&self.amount)?;
        seq.end()
    }
}

struct DepthRowVisitor;

impl<'de> Visitor<'de> for DepthRowVisitor {
    type Value = DepthRow;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "a map with keys 'first' and 'second'")
    }

    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
    where
        A: SeqAccess<'de>,
    {
        let mut price = None;
        let mut amount = None;

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

        if price.is_none() {
            return Err(serde::de::Error::custom("Missing price field"));
        }

        if amount.is_none() {
            return Err(serde::de::Error::custom("Missing amount field"));
        }

        Ok(DepthRow {
            price: price.unwrap(),
            amount: amount.unwrap(),
        })
    }
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct BinanceSpotOrderBookSnapshot {
    pub last_update_id: i64,
    pub time_stamp: i64,
    pub bids: Vec<DepthRow>,
    pub asks: Vec<DepthRow>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct OrderBookStore {
    pub last_update_id: i64,
    pub time_stamp: i64,
    pub bids: Vec<(f64, f64)>,
    pub asks: Vec<(f64, f64)>,
}

impl BinanceSpotOrderBookSnapshot {
    /// Transform into data that could be serialized
    pub fn transform_to_local(&self) -> OrderBookStore {
        let bids: Vec<_> = self.bids.iter().map(|x| (x.price, x.amount)).collect();
        let asks: Vec<_> = self.asks.iter().map(|x| (x.price, x.amount)).collect();
        OrderBookStore {
            last_update_id: self.last_update_id,
            time_stamp: self.time_stamp,
            bids,
            asks,
        }
    }

    pub fn transform_from_local(data: OrderBookStore) -> Self {
        let bids: Vec<_> = data
            .bids
            .iter()
            .map(|(price, amount)| DepthRow {
                price: *price,
                amount: *amount,
            })
            .collect();
        let asks: Vec<_> = data
            .asks
            .iter()
            .map(|(price, amount)| DepthRow {
                price: *price,
                amount: *amount,
            })
            .collect();

        BinanceSpotOrderBookSnapshot {
            last_update_id: data.last_update_id,
            time_stamp: data.time_stamp,
            bids,
            asks,
        }
    }

    /// Get string from local store
    fn from_string(data: String) -> Self {
        let raw: OrderBookStore = serde_json::from_str(&data).unwrap();
        Self::transform_from_local(raw)
    }
}

#[test]
fn write_file() {
    use std::fs::OpenOptions;
    use std::io::prelude::*;
    let mut file = OpenOptions::new();
    let mut reader = file.create(true).append(true).open("abc").unwrap();

    let order_book_origin = BinanceSpotOrderBookSnapshot {
        time_stamp: 20,
        last_update_id: 20,
        bids: vec![DepthRow {
            price: 1.0,
            amount: 3.0,
        }],
        asks: vec![DepthRow {
            price: 1.0,
            amount: 3.0,
        }],
    };
    let transformed = order_book_origin.transform_to_local();
    let serd_trans = serde_json::to_string(&transformed).unwrap();
    let raw = format!("{}\n", serd_trans);

    reader.write_all(raw.as_bytes()).unwrap();
}

#[test]
fn read_file() {
    use std::fs::OpenOptions;

    use std::io::Read;
    let mut file = OpenOptions::new();
    let mut reader = file.read(true).open("abc").unwrap();

    let mut buffer = String::new();

    reader.read_to_string(&mut buffer).unwrap();

    // println!("{}", buffer);
    buffer.pop();
    let a: Vec<_> = buffer.split("\n").collect();

    for data in a {
        let raw: OrderBookStore = serde_json::from_str(&data).unwrap();
        let finassl = BinanceSpotOrderBookSnapshot::transform_from_local(raw);
        println!("{:?}", finassl);
    }
}

pub fn get_infustructure() {
    tokio::spawn(async move {
        loop {
            let res: Result<()> = {
                let snapshot: BinanceSpotOrderBookSnapshot =
                    reqwest::get("https://api.binance.com/api/v3/depth?symbol=BNBBTC&limit=1000")
                        .await?
                        .json()
                        .await?;
                Ok(())
            };
        }
        Ok::<(), Error>(())
    });
}
