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
        params.push(("currency".to_string(), self.currency));
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
    id: i32,
    method: String,
    api_key: String,
    /// (key, value)
    params: Params,
    nonce: i64,
    sig: String,
}

impl<Params:CryptoDecode + Clone> CryptoRequest<Params>{
    pub fn new(method: &str, id: i32, params: Params) -> Self{
        let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
        CryptoRequest { 
            method: method.to_string(),
            api_key: String::new(),
            id, 
            params, 
            nonce: now.as_millis() as i64,
            sig: String::new(),
        }
    }

    pub fn sign(&mut self, api_key: &str, secret_key: &str){
        let sig = self.sign_request(api_key, secret_key);
        self.api_key = api_key.to_string();
        self.sig = sig;
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

