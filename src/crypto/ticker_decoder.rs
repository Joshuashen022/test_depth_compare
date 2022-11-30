use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct EventTicker {
    #[serde(rename = "e")]
    pub event_type: String,
    #[serde(rename = "E")]
    pub event_time: i64,
    #[serde(rename = "s")]
    pub pair: String,
    #[serde(rename = "t")]
    pub last_update_id: i64,
    #[serde(rename = "p")]
    pub price: String,
    #[serde(rename = "q")]
    pub amount: String,
    #[serde(rename = "b")]
    pub buy_id: i64,
    #[serde(rename = "a")]
    pub sell_id: i64,
    #[serde(rename = "T")]
    pub trade_time: i64,
    /// true => sell
    #[serde(rename = "m")]
    pub direction: bool,
    #[serde(rename = "M")]
    pub other: bool,
}

#[derive(Deserialize, Debug)]
pub struct EventTickers {
    #[serde(rename = "e")]
    pub event_type: String,
    #[serde(rename = "E")]
    pub event_time: i64,
    #[serde(rename = "s")]
    pub pair: String,
    #[serde(rename = "t")]
    pub last_update_id: i64,
    #[serde(rename = "p")]
    pub price: String,
    #[serde(rename = "q")]
    pub amount: String,
    #[serde(rename = "b")]
    pub buy_id: String,
    #[serde(rename = "a")]
    pub sell_id: String,
    #[serde(rename = "T")]
    pub trade_time: i64,
    /// true => sell
    #[serde(rename = "m")]
    pub direction: bool,
    #[serde(rename = "M")]
    pub other: i64,
}
