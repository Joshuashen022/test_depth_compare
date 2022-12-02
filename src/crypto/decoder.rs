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
pub struct BinanceOrderUpdatePayload{
    #[serde(rename = "e")]
    event_type: String,
    #[serde(rename = "E")]
    event_time: i64,
    #[serde(rename = "s")]
    symbol: String,
    #[serde(rename = "c")]
    client_order_id: String,
    #[serde(rename = "S")]
    side: String,
    #[serde(rename = "o")]
    order_type: String,
    #[serde(rename = "f")]
    time_in_force: String,
    #[serde(rename = "q")]
    order_quantity: String,
    #[serde(rename = "p")]
    order_price: String,
    #[serde(rename = "P")]
    stop_price: String,
    ///partial visable
    #[serde(rename = "d")]
    trailing_delta: i64,
    #[serde(rename = "F")]
    iceberg_quantity: String,
    #[serde(rename = "g")]
    order_list_id: i64,
    #[serde(rename = "C")]
    origin_client_order_id: String,
    #[serde(rename = "x")]
    current_execution_type: String,
    #[serde(rename = "X")]
    current_order_status: String,
    #[serde(rename = "r")]
    order_reject_reason: String,
    #[serde(rename = "i")]
    order_id: i64,
    #[serde(rename = "l")]
    last_executed_quantity: String,
    #[serde(rename = "z")]
    cumulative_filled_quantity: String,
    #[serde(rename = "L")]
    last_executed_price: String,
    #[serde(rename = "n")]
    commission_amount: String,
    #[serde(rename = "N")]
    comession_asset: Option<String>,
    #[serde(rename = "T")]
    transaction_time: i64,
    #[serde(rename = "t")]
    trade_id: i64,
    #[serde(rename = "I")]
    ignore_1: i64,
    #[serde(rename = "w")]
    is_on_book: bool,
    #[serde(rename = "m")]
    is_trade_maker: bool,
    #[serde(rename = "M")]
    ignore_2: i64,
    #[serde(rename = "O")]
    order_creation_time: i64,
    #[serde(rename = "Z")]
    cumulative_transacted_quantity: String,
    #[serde(rename = "Y")]
    last_transacted_quantity: String,
    #[serde(rename = "Q")]
    quote_order_qty: String,
    #[serde(rename = "j")]
    strategy_id: i64,
    #[serde(rename = "J")]
    strategy_type: i64,

}

#[derive(Clone, Deserialize, Serialize)]
pub struct OrderInfo{
    // orderInfo.OrderPrice
    order_price: String,
    // orderInfo.Quantity 
    order_quantity: String,
    // orderInfo.OrderId 
    order_id: i64,
    // orderInfo.ClientOrderId
    client_order_id: String,
    // orderInfo.CreateTime 
    order_creation_time: i64,
    // orderInfo.UpdateTime
    event_time: i64,
    // orderInfo.CumuTradeQty
    cumulative_filled_quantity: String,
    // orderInfo.CumuTradeValue 
    cumulative_transacted_quantity: String,
    // orderInfo.FeeCurrency = msgMap["N"].String()
    comession_asset: String,
    // orderInfo.Status = 2
    status: i64,
    // if repOrderMsg.GetOrderStream().CumuTradeQty != 0 {
    //     orderInfo.AvgPrice = orderInfo.CumuTradeValue / orderInfo.CumuTradeQty
    // }
    average_price:i64
}

#[derive(Clone, Deserialize, Serialize)]
pub struct  TradeInfo{
    // tradeInfo.Fee 
    commission_amount: String,
    // tradeInfo.FeeCurrency
    comession_asset: Option<String>,
    // tradeInfo.TradeId
    trade_id: i64,
    // tradeInfo.TradeTime
    transaction_time: i64,
    // tradeInfo.TradeQty 
    last_executed_quantity: String,
    // tradeInfo.TradePrice
    last_executed_price: String,
    // tradeInfo.OrderId 
    order_id: i64,
    // tradeInfo.ClientOrderId
    client_order_id: String,
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
