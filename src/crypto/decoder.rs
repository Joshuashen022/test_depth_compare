use std::fmt::format;

// use ordered_float::OrderedFloat;
// use serde::de::{SeqAccess, Visitor};
use serde::{Deserialize, Serialize};
// use serde::Deserializer;
// use std::collections::BTreeMap;
// use std::fmt::{self, Debug};
use std::time::{SystemTime, UNIX_EPOCH};
// use tokio_tungstenite::tungstenite;
// use tungstenite::protocol::Message;
use hex::encode;
use hmac::{Hmac, Mac};
// use reqwest;
use sha2::Sha256;
type HmacSha256 = Hmac<Sha256>;

#[derive(Clone, Deserialize, Serialize)]
pub struct BinanceOrder {
    symbol: String,

    side: Side,

    #[serde(rename = "type")]
    order_type: OrderType,

    #[serde(rename = "timeInForce")]
    time_in_force: Option<TimeInForce>,

    quantity: Option<i64>,

    #[serde(rename = "quoteOrderQty")]
    quote_order_qty: Option<i64>,

    price: Option<i64>,

    #[serde(rename = "newClientOrderId")]
    new_client_order_id: Option<String>,

    #[serde(rename = "stopPrice")]
    stop_price: Option<i64>,

    #[serde(rename = "trailingDelta")]
    trailing_delta: Option<i64>,

    #[serde(rename = "icebergQty")]
    iceberg_qty: Option<i64>,

    #[serde(rename = "newOrderRespType")]
    new_order_resp_type: Option<NewOrderRespType>,

    #[serde(rename = "strategyId")]
    strategy_id: Option<i64>,

    #[serde(rename = "strategyType")]
    strategy_type: Option<i64>,

    #[serde(rename = "recvWindow")]
    recv_window: Option<i64>,

    timestamp: i64,
}

pub struct Hasher{
    pub secret_key: String,
    pub api_key: String,
    pub raw_message: String,
}

impl Hasher{

    pub fn hash(&self) -> String{
        
        let mut mac = HmacSha256::new_from_slice(self.secret_key.as_bytes()).unwrap();
        mac.update(self.raw_message.as_bytes());
        let hash_bytes = mac.finalize().into_bytes();
        encode(hash_bytes)
    }

}

impl BinanceOrder {
    pub fn new_default() -> Self {
        BinanceOrder {
            symbol: String::from("BUSDUSDT"),
            side: Side::Buy,
            order_type: OrderType::LimitMaker,
            time_in_force: None,
            quantity: None,
            quote_order_qty: None,
            price: None,
            new_client_order_id: None,
            stop_price: None,
            trailing_delta: None,
            iceberg_qty: None,
            new_order_resp_type: None,
            strategy_id: None,
            strategy_type: None,
            recv_window: None,
            timestamp: 0,
        }
    }
    //"symbol=BUSDUSDT&side=BUY&type=LIMIT&timeInForce=GTC&quantity=1&price=0.1&recvWindow=5000&timestamp=1499827319559"

    pub fn into_string(self) -> String {

        let symbol = "BUSDUSDT";
        let side = "BUY";
        let order_type = "LIMIT";
        let recv_window = 5000;
        let quantityt = 1;
        let time_in_force = "GTC";
        let price = 1.0;
        let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();

        format!(
            "symbol={}&side={}&type={}&timeInForce={}&quantity={}&price={}&recvWindow={}&timestamp={}", 
            symbol, side, order_type, time_in_force, quantityt, price, recv_window, now.as_millis() as i64
        )
    }
}

#[derive(Clone, Deserialize, Serialize)]
pub enum Side {
    #[serde(rename = "BUY")]
    Buy,
    #[serde(rename = "SELL")]
    Sell,
}

#[derive(Clone, Deserialize, Serialize)]
pub enum OrderType {
    #[serde(rename = "LIMIT")]
    Limit,
    #[serde(rename = "MARKET")]
    Maket,
    #[serde(rename = "STOP_LOSS")]
    StopLoss,
    #[serde(rename = "STOP_LOSS_LIMIT")]
    StopLossLimit,
    #[serde(rename = "TAKE_PROFIT")]
    TakeProfit,
    #[serde(rename = "TAKE_PROFIT_LIMIT")]
    TakeProfitLimit,
    #[serde(rename = "LIMIT_MAKER")]
    LimitMaker,
}

#[derive(Clone, Deserialize, Serialize)]
pub enum TimeInForce {
    /// Stop until success
    GTC,
    /// Stop if can't success inmediately
    /// Make as much as order as possible
    IOC,
    /// Stop if can't success all
    FOK,
}

#[derive(Clone, Deserialize, Serialize)]
pub enum NewOrderRespType {
    ACK,
    RESULT,
    FULL,
}

#[cfg(test)]
mod test {
    use super::*;
    use hex::encode;
    use hmac::{Hmac, Mac};
    use sha2::Sha256;

    #[test]
    fn test_none_serde() {
        let binance_order = BinanceOrder::new_default();

        let serde_str = serde_json::to_string(&binance_order).unwrap();
        println!("{}", serde_str);
    }

    #[test]
    fn sha256_test() {
        type HmacSha256 = Hmac<Sha256>;
        let input = b"NhqPtmdSJYdKjVHjA7PZj4Mge3R5YNiP1e3UZjInClVN65XAbvqqM6A7H5fATj0j";
        let info2 = b"symbol=LTCBTC&side=BUY&type=LIMIT&timeInForce=GTC&quantity=1&price=0.1&recvWindow=5000&timestamp=1499827319559";
        let mut mac = HmacSha256::new_from_slice(input).unwrap();

        mac.update(info2);
        let result = mac.finalize().into_bytes();
        let result = encode(result);

        assert_eq!(
            result,
            String::from("c8db56825ae71d6d79447849e617115f4a920fa2acdcab2b053c4b2838bd6b71")
        );
    }
}
