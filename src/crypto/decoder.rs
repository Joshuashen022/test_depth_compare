use super::runner::*;

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
#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct ListenKey{
    #[serde(rename = "listenKey")]
    listen_key: String
}


#[derive(Clone, Deserialize, Serialize)]
pub struct BinanceCheckAllOrder{
    symbol: String,
    order_id: i64,
    start_time: i64,
    end_time: i64,
    limit: String,
    receive_window: i64,
    timestamp: i64,
}

impl BinanceCheckAllOrder {
    pub fn new() -> Self {
        BinanceCheckAllOrder{
            symbol: String::new(),
            order_id: 0,
            start_time: 0,
            end_time: 0,
            limit: String::new(),
            receive_window: 0,
            timestamp: 0,
        }
    }

    // ./target/release/ac-rust --ca -s "BUSDUSDT"
    // ./target/release/ac-rust --co -s "BUSDUSDT" --oid 786706435 --cid "s6iHMeEj6Se2NSXImlBFbA"
    // ./target/release/ac-rust --da -s "BUSDUSDT"
    // order_id: 786706435, order_list_id: -1, client_order_id: "s6iHMeEj6Se2NSXImlBFbA"
    pub fn into_string(&self) -> String {

        let symbol = "BUSDUSDT";
        // let order_id = 785460948;
        // let orgin_client_order_id = "xiyCOYg0CddVT0nedEAg35";
        let receive_window = 5000;
        let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();

        format!(
            "symbol={}&recvWindow={}&timestamp={}", 
            symbol, receive_window, now.as_millis() as i64
        )
    }

    pub fn get_body(&self) -> String {
        let params = self.into_string();
    
        let hasher = Hasher{
            api_key: ACCESS_KEY.to_string(),
            secret_key: SECRET_KEY.to_string(),
            raw_message: params.clone(),
        };
    
        let hash = hasher.hash();
    
        format!("{}&signature={}", params, hash)
    }

}

// "[{\"symbol\":\"BUSDUSDT\",\"origClientOrderId\":\"MBsclqrSGO3C8zqUDZJvwO\",
// \"orderId\":785401796,\"orderListId\":-1,\"clientOrderId\":\"KOO4AHSGsCbuMXRVKJveJ2\",
// \"price\":\"1.00000000\",\"origQty\":\"10.00000000\",\"executedQty\":\"0.00000000\",
// \"cummulativeQuoteQty\":\"0.00000000\",\"status\":\"CANCELED\",\"timeInForce\":\"GTC\",
// \"type\":\"LIMIT\",\"side\":\"BUY\"}]"

#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct BinanceCheckAllOrderResponse(Vec<BinanceCheckOrderResponse>);

#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct BinanceCheckOrderResponse {
    symbol: String,

    #[serde(rename = "orderId")]
    order_id: i64,

    #[serde(rename = "orderListId")]
    order_list_id: i64,

    #[serde(rename = "clientOrderId")]
    client_order_id: String,

    price: String,

    #[serde(rename = "origQty")]
    origin_qty: String,

    #[serde(rename = "executedQty")]
    executed_qty: String,

    #[serde(rename = "cummulativeQuoteQty")]
    cummulative_quote_qty: String,

    status: String,

    #[serde(rename = "timeInForce")]
    time_in_force: String,

    #[serde(rename = "type")]
    order_type: String,

    side: String,

    #[serde(rename = "stopPrice")]
    stop_price:String,
    
    #[serde(rename = "icebergQty")]
    iceberg_qty:String,

    time: i64,

    #[serde(rename = "updateTime")]
    update_time: i64,

    #[serde(rename = "isWorking")]
    is_working: bool,

    #[serde(rename = "origQuoteOrderQty")]
    origin_quote_order_qty: String,

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




#[cfg(test)]
mod test {
    use hex::encode;
    use hmac::{Hmac, Mac};
    use sha2::Sha256;

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
