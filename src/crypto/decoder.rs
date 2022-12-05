use super::runner::*;
use anyhow::{Error, Result, anyhow};
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
pub struct ListenKey {
    #[serde(rename = "listenKey")]
    listen_key: String,
}

#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct PayloadType {
    #[serde(rename = "e")]
    pub event_type: String,
}

#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct BinanceOrderUpdatePayload {
    #[serde(rename = "e")] //
    event_type: String,
    #[serde(rename = "E")] //
    event_time: i64,
    #[serde(rename = "s")] //
    symbol: String,
    #[serde(rename = "c")] //
    client_order_id: String,
    #[serde(rename = "S")] //
    side: String,
    #[serde(rename = "o")] //
    order_type: String,
    #[serde(rename = "f")] //
    time_in_force: String,
    #[serde(rename = "q")] //
    order_quantity: String,
    #[serde(rename = "p")] //
    order_price: String,
    #[serde(rename = "P")] //
    stop_price: String,
    ///partial visable
    #[serde(rename = "d")]
    trailing_delta: Option<i64>,
    #[serde(rename = "F")] //
    iceberg_quantity: String,
    #[serde(rename = "g")] //
    order_list_id: i64,
    #[serde(rename = "C")]
    origin_client_order_id: Option<String>,
    #[serde(rename = "x")] //
    current_execution_type: String,
    #[serde(rename = "X")] //
    current_order_status: String,
    #[serde(rename = "r")] //
    order_reject_reason: String,
    #[serde(rename = "i")] //
    order_id: i64,
    #[serde(rename = "l")] //
    last_executed_quantity: String,
    #[serde(rename = "z")] //
    cumulative_filled_quantity: String,
    #[serde(rename = "L")] //
    last_executed_price: String,
    #[serde(rename = "n")] //
    commission_amount: String,
    #[serde(rename = "N")] //
    comession_asset: Option<String>,
    #[serde(rename = "T")] //
    transaction_time: i64,
    #[serde(rename = "t")] //
    trade_id: i64,
    #[serde(rename = "I")] //
    ignore_1: i64,
    #[serde(rename = "w")] //
    is_on_book: bool,
    #[serde(rename = "m")] //
    is_trade_maker: bool,
    #[serde(rename = "M")] //
    ignore_2: bool,
    #[serde(rename = "O")] //
    order_creation_time: i64,
    #[serde(rename = "Z")] //
    cumulative_transacted_quantity: String,
    #[serde(rename = "Y")] //
    last_transacted_quantity: String,
    #[serde(rename = "Q")] //
    quote_order_qty: String,
    #[serde(rename = "j")]
    strategy_id: Option<i64>,
    #[serde(rename = "J")]
    strategy_type: Option<i64>,
}

impl BinanceOrderUpdatePayload {
    
    #[allow(dead_code)]
    pub fn into_trade_and_order_info(self) -> Result<(Option<TradeInfo>, Option<OrderInfo>)> {
        let mut trade_info = None;
        let mut order_info = None;

        let average_price = self.average_price();
        
        let comession_asset = self.comession_asset.ok_or(anyhow!("comession_asset is empty"))?;

        match self.current_execution_type.as_str() {
            "TRADE" => {
                trade_info = Some(TradeInfo {
                    commission_amount: self.commission_amount,
                    comession_asset: Some(comession_asset.clone()),
                    trade_id: self.trade_id,
                    transaction_time: self.transaction_time,
                    last_executed_quantity: self.last_executed_quantity,
                    last_executed_price: self.last_executed_price,
                    order_id: self.order_id,
                    client_order_id: self.client_order_id.clone(),
                });
                if &self.current_order_status == "FILLED" {

                    let average_price = average_price?;

                    order_info = Some(OrderInfo {
                        order_price: self.order_price,
                        order_quantity: self.order_quantity,
                        order_id: self.order_id,
                        client_order_id: self.client_order_id,
                        order_creation_time: self.order_creation_time,
                        event_time: self.event_time,
                        cumulative_filled_quantity: self.cumulative_filled_quantity,
                        cumulative_transacted_quantity: self.cumulative_transacted_quantity,
                        comession_asset,
                        status: 2,
                        average_price,
                    })
                }
            }
            "NEW" | "CANCELED" | "REJECTED" | "EXPIRED" => {
                let status = match self.current_order_status.as_str(){
                    "NEW" | "PARTIALLY_FILLED" => 1,
                    "CANCELED" | "FILLED" | "REJECTED" | "EXPIRED" => 2,
                    _ => return Err(anyhow!("comession_asset is empty"))
                };
                
                let average_price = average_price?;

                order_info = Some(OrderInfo {
                    order_price: self.order_price,
                    order_quantity: self.order_quantity,
                    order_id: self.order_id,
                    client_order_id: self.client_order_id,
                    order_creation_time: self.order_creation_time,
                    event_time: self.event_time,
                    cumulative_filled_quantity: self.cumulative_filled_quantity,
                    cumulative_transacted_quantity: self.cumulative_transacted_quantity,
                    comession_asset,
                    status,
                    average_price,
                })

            }
            _ => {}
        }

        Ok((trade_info, order_info))
    }

    pub fn into_trade_and_order_info_debug(self) -> Result<(Option<TradeInfo>, Option<OrderInfo>)> {
        let mut trade_info = None;
        let mut order_info = None;
        
        if self.comession_asset.is_none(){
            println!("comession_asset is empty")
        }

        let comession_asset = String::new();

        match self.current_execution_type.as_str() {
            "TRADE" => {
                trade_info = Some(TradeInfo {
                    commission_amount: self.commission_amount,
                    comession_asset: Some(comession_asset.clone()),
                    trade_id: self.trade_id,
                    transaction_time: self.transaction_time,
                    last_executed_quantity: self.last_executed_quantity,
                    last_executed_price: self.last_executed_price,
                    order_id: self.order_id,
                    client_order_id: self.client_order_id.clone(),
                });
                if &self.current_order_status == "FILLED" {

                    let average_price = 1.0;

                    order_info = Some(OrderInfo {
                        order_price: self.order_price,
                        order_quantity: self.order_quantity,
                        order_id: self.order_id,
                        client_order_id: self.client_order_id,
                        order_creation_time: self.order_creation_time,
                        event_time: self.event_time,
                        cumulative_filled_quantity: self.cumulative_filled_quantity,
                        cumulative_transacted_quantity: self.cumulative_transacted_quantity,
                        comession_asset,
                        status: 2,
                        average_price,
                    })
                }
            }
            "NEW" | "CANCELED" | "REJECTED" | "EXPIRED" => {
                let status = match self.current_order_status.as_str(){
                    "NEW" | "PARTIALLY_FILLED" => 1,
                    "CANCELED" | "FILLED" | "REJECTED" | "EXPIRED" => 2,
                    _ => return Err(anyhow!("comession_asset is empty"))
                };
                
                let average_price = 1.0;

                order_info = Some(OrderInfo {
                    order_price: self.order_price,
                    order_quantity: self.order_quantity,
                    order_id: self.order_id,
                    client_order_id: self.client_order_id,
                    order_creation_time: self.order_creation_time,
                    event_time: self.event_time,
                    cumulative_filled_quantity: self.cumulative_filled_quantity,
                    cumulative_transacted_quantity: self.cumulative_transacted_quantity,
                    comession_asset,
                    status,
                    average_price,
                })

            }
            _ => {}
        }

        Ok((trade_info, order_info))
    }


    fn average_price(&self) -> Result<f64>{
        let cumulative_transacted_quantity =
            self.cumulative_transacted_quantity.parse::<f64>()?;
        let cumulative_filled_quantity =
            self.cumulative_filled_quantity.parse::<f64>()?;
        if cumulative_filled_quantity != 0.0 {
            Ok(cumulative_transacted_quantity / cumulative_filled_quantity)
        } else {
            return Err(anyhow::anyhow!("cumulative_filled_quantity is zero"))
        }
    }
}

#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct OrderInfo {
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
    average_price: f64,
}

#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct TradeInfo {
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

#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct OutboundAccountPositionPayload {
    #[serde(rename = "e")]
    event_type: String,
    #[serde(rename = "E")]
    event_time: i64,
    #[serde(rename = "u")]
    account_last_update_time: i64,
    #[serde(rename = "B")]
    balance: Vec<Balance>,
}

#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct Balance {
    #[serde(rename = "a")]
    name: String,
    #[serde(rename = "f")]
    usable_amount: String,
    #[serde(rename = "l")]
    frozen_amount: String,
}

pub struct Hasher {
    pub secret_key: String,
    pub api_key: String,
    pub raw_message: String,
}

impl Hasher {
    pub fn hash(&self) -> String {
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
