use crate::request::vonageRequest::vonage_request;
use crate::verify::start_verify_response::start_verify_response;
use serde::{Deserialize, Serialize};
use reqwest::header::{HeaderMap, HeaderValue, CONTENT_TYPE};
use std::collections::HashMap;

pub struct start_verify{
    pub args:HashMap<String,String>,
    pub request:vonage_request
}
impl start_verify{
    pub async fn send(&self)->Option<start_verify_response>{
        let mut params = self.args.clone();
        params.insert("api_key".into(),self.request.api_key.to_string());
        params.insert("api_secret".into(), self.request.api_secret.to_string());
        let response:Option<start_verify_response> = self.request.sendReturnGeneric(params,String::from("https://api.nexmo.com/verify/json")).await.unwrap();
        response
    }
}
