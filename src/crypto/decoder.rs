// use ordered_float::OrderedFloat;
// use serde::de::{SeqAccess, Visitor};
use serde::{Deserialize, Serialize};
// use serde::Deserializer;
// use std::collections::BTreeMap;
// use std::fmt::{self, Debug};
// use std::time::{SystemTime, UNIX_EPOCH};
// use tokio_tungstenite::tungstenite;
// use tungstenite::protocol::Message;


#[derive(Clone, Deserialize, Serialize)]
pub struct BinanceOrder{
    symbol: String,
    
    side: Side,
    
    #[serde(rename = "type")]
    order_type: OrderType,
    
    #[serde(rename = "timeInForce")]
    time_in_force:Option<TimeInForce>,
    
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

impl BinanceOrder{

    pub fn new() -> Self{
        BinanceOrder{
            symbol: String::from("BTCUSDT"),
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
}


#[derive(Clone, Deserialize, Serialize)]
pub enum Side{
    #[serde(rename = "BUY")]
    Buy,
    #[serde(rename = "SELL")]
    Sell,
}

#[derive(Clone, Deserialize, Serialize)]
pub enum OrderType{
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
pub enum TimeInForce{
    /// Stop until success
    GTC,
    /// Stop if can't success inmediately
    /// Make as much as order as possible
    IOC,
    /// Stop if can't success all
    FOK,
}

#[derive(Clone, Deserialize, Serialize)]
pub enum NewOrderRespType{
    ACK,
    RESULT,
    FULL,
}



#[cfg(test)]
mod test{
    use super::*;

    #[test]
    fn test_none_serde(){
        
        let binance_order = BinanceOrder::new();

        let serde_str = serde_json::to_string(&binance_order).unwrap();
        println!("{}", serde_str) ;
    }

}