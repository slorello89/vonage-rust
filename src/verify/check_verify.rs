use crate::request::vonageRequest::vonage_request;
use serde::{Deserialize, Serialize};
use reqwest::header::{HeaderMap, HeaderValue, CONTENT_TYPE};
use std::collections::HashMap;

#[derive(Serialize,Debug)]
pub struct check_verify{
    pub args:HashMap<String,String>,
    pub request:vonage_request
}

impl check_verify{
    pub async fn send(&self)->Option<check_verify_response>{
        let mut params = self.args.clone();
        params.insert("api_key".into(),self.request.api_key.to_string());
        params.insert("api_secret".into(), self.request.api_secret.to_string());
        let response:Option<check_verify_response> = self.request.sendReturnGeneric(params,String::from("https://api.nexmo.com/verify/check/json")).await.unwrap();
        response
    }    
}

#[derive(Deserialize,Debug)]
pub struct check_verify_response{
    request_id:String,
    event_id:Option<String>,
    status:Option<String>,
    price:Option<String>,
    currency:Option<String>,
    estimated_price_messages_sent:Option<String>
}