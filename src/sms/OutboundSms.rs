use crate::request::vonageRequest::vonage_request;
use crate::sms::SmsResponse::SmsResponse;
use serde::{Deserialize, Serialize};
use reqwest::header::{HeaderMap, HeaderValue, CONTENT_TYPE};
use std::collections::HashMap;

pub struct OutboundSms{    
    pub args:HashMap<String,String>,
    pub request: vonage_request
}



impl OutboundSms{
    pub async fn send(&self)->Option<SmsResponse>{
        let mut params = self.args.clone();
        params.insert(String::from("api_key"), self.request.api_key.clone());
        params.insert(String::from("api_secret"), self.request.api_secret.clone());
        let response:Option<SmsResponse> = self.request.sendReturnGeneric(params, "https://rest.nexmo.com/sms/json".to_string()).await.unwrap();
        response
    }
}
