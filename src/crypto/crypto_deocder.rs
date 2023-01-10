use serde::Serialize;
use std::time::{SystemTime, UNIX_EPOCH};
use hex::encode;
use hmac::{Hmac, Mac};
use sha2::Sha256;
type HmacSha256 = Hmac<Sha256>;

pub trait CryptoDecode{
    fn into_string(self) -> String;
}
#[derive(Clone, Debug, Serialize)]
pub struct GetAccountSummary{
    pub currency: String,
}

impl CryptoDecode for GetAccountSummary{
    fn into_string(self) -> String {
        let mut params = Vec::new();
        params.push(("currency".into(), self.currency));
        CryptoParams(params).into_string()
    }
}

#[derive(Clone, Debug, Serialize)]
pub struct CreateOrder{
    pub instrument_name: String,
    pub side: String,
    pub r#type:String,
    pub price: String,
    pub quantity: String,
    pub notional: Option<String>,
    pub client_oid: String,
    pub time_in_force: Option<String>,
    pub exec_inst: String,
    pub trigger_price: Option<String>
}

impl CreateOrder{
    pub fn new(
        instrument_name: &str,
        is_buy: bool,
        price: &str,
        amount: &str,
        client_oid: &str,
        is_maker: bool,
    ) -> Self{
        CreateOrder{
            instrument_name: instrument_name.to_string(),
            side: if is_buy {"BUY".into()} else{"SELL".into()},
            r#type:"LIMIT".into(),
            price: price.into(),
            quantity: amount.into(),
            notional: None,
            client_oid: client_oid.into(),
            time_in_force: None,
            exec_inst: if is_maker{"POST_ONLY".into()}else{"".into()},
            trigger_price: None
        }
    }
}

impl CryptoDecode for CreateOrder{
    fn into_string(self) -> String {
        let mut params = Vec::new();
        params.push(("instrument_name".into(), self.instrument_name));
        params.push(("side".into(), self.side));
        params.push(("type".into(), self.r#type));
        params.push(("price".into(), self.price));
        params.push(("quantity".into(), self.quantity));
        params.push(("client_oid".into(), self.client_oid));
        params.push(("exec_inst".into(), self.exec_inst));
        CryptoParams(params).into_string()
    }
}

pub struct CryptoParams(Vec<(String,String)>);

impl CryptoParams{
    fn into_string(self) -> String{
        let mut result = String::new();
        let mut params = self.0;
        params.sort();
        for (key, value) in params.iter(){
            result += key;
            result += value;
        }
        result
    }
}
#[derive(Clone, Serialize, Debug)]
pub struct CryptoRequest<Params:Clone>{
    id: i64,
    method: String,
    /// (key, value)
    params: Params,
    // api_key: String,
    // sig: String,
    nonce: i64,
}

impl<Params:CryptoDecode + Clone> CryptoRequest<Params>{
    pub fn new(method: &str, id: i64, params: Params) -> Self{
        let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
        CryptoRequest { 
            method: method.to_string(),
            // api_key: String::new(),
            id, 
            params, 
            nonce: now.as_millis() as i64,
            // sig: String::new(),
        }
    }

    pub fn sign(&mut self, api_key: &str, secret_key: &str){
        let _sig = self.sign_request(api_key, secret_key);
        // self.api_key = api_key.to_string();
        // self.sig = sig;
    }

    fn sign_request(&self, api_key: &str, secret_key: &str) -> String {
        let params = self.params.clone().into_string();
        let id = self.id;
        let nonce = self.nonce;
        let method = self.method.clone();
        let sig_payload = format!("{}{}{}{}{}", method, id, api_key, params, nonce);
    
        let hash = Hasher{
            secret_key: secret_key.to_string(),
            api_key: api_key.to_string(),
            raw_message: sig_payload,
        };
        hash.hash()
    }
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

