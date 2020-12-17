use crate::request::vonageRequest::vonage_request;
use serde::{Deserialize, Serialize};
use reqwest::header::{HeaderMap, HeaderValue, CONTENT_TYPE};
use std::collections::HashMap;

#[derive(Serialize, Debug)]
pub struct insight_request{
    pub args:HashMap<String,String>,
    pub request:vonage_request
}

impl insight_request{
    pub async fn get_basic(&self)->Option<basic_insight_response>{
        let mut params = self.args.clone();
        params.insert(String::from("api_secret"),self.request.api_secret.clone());
        params.insert(String::from("api_key"),self.request.api_key.clone());
        self.request.get(params, String::from("https://api.nexmo.com/ni/basic/json")).await
    }
    pub async fn get_standard(&self)->Option<standard_insight_response>{
        let mut params = self.args.clone();
        params.insert(String::from("api_secret"),self.request.api_secret.clone());
        params.insert(String::from("api_key"),self.request.api_key.clone());
        self.request.get(params, String::from("https://api.nexmo.com/ni/standard/json")).await
    }
    pub async fn get_advanced_sync(&self)->Option<advanced_insight_response>{
        let mut params = self.args.clone();
        params.insert(String::from("api_secret"),self.request.api_secret.clone());
        params.insert(String::from("api_key"),self.request.api_key.clone());
        self.request.get(params, String::from("https://api.nexmo.com/ni/advanced/json")).await
    }
}

#[derive(Deserialize,Debug)]
pub struct basic_insight_response{
    pub status:Option<u8>,
    pub status_message:Option<String>,
    pub request_id:Option<String>,
    pub international_format_number:Option<String>,
    pub national_format_number:Option<String>,
    pub country_code:Option<String>,
    pub country_code_iso3:Option<String>,
    pub country_name:Option<String>,
    pub country_prefix:Option<String>
}

#[derive(Deserialize, Debug)]
pub struct carrier{
    pub network_code:Option<String>,
    pub name:Option<String>,
    pub country:Option<String>,
    pub network_type:Option<String>
}

#[derive(Deserialize, Debug)]
pub struct roaming{
    pub status:Option<String>,
    pub roaming_country_code:Option<String>,
    pub roaming_network_code:Option<String>,
    pub roaming_network_name:Option<String>

}
#[derive(Deserialize, Debug)]
pub struct caller_id{
    pub caller_type:Option<String>,
    pub caller_name:Option<String>,
    pub first_name:Option<String>,
    pub last_name:Option<String>
}


#[derive(Deserialize, Debug)]
pub struct standard_insight_response{
    pub status:Option<u8>,
    pub status_message:Option<String>,
    pub request_id:Option<String>,
    pub international_format_number:Option<String>,
    pub national_format_number:Option<String>,
    pub country_code:Option<String>,
    pub country_code_iso3: Option<String>,
    pub country_name:Option<String>,
    pub request_price:Option<String>,
    pub refund_price:Option<String>,
    pub remaining_balance:String,
    pub current_carrier:Option<carrier>,
    pub original_carrier:Option<carrier>,
    pub caller_id:Option<caller_id>,
    pub caller_name:Option<String>,
    pub last_name:Option<String>,
    pub first_name:Option<String>,
    pub caller_type:Option<String>
}

#[derive(Deserialize, Debug)]
pub struct advanced_insight_response{
    pub status:Option<u8>,
    pub status_message:Option<String>,
    pub request_id:Option<String>,
    pub international_format_number:Option<String>,
    pub national_format_number:Option<String>,
    pub country_code:Option<String>,
    pub country_code_iso3: Option<String>,
    pub country_name:Option<String>,
    pub request_price:Option<String>,
    pub refund_price:Option<String>,
    pub remaining_balance:String,
    pub current_carrier:Option<carrier>,
    pub original_carrier:Option<carrier>,
    pub caller_id:Option<caller_id>,
    pub caller_name:Option<String>,
    pub last_name:Option<String>,
    pub first_name:Option<String>,
    pub caller_type:Option<String>,
    pub ported:Option<String>,
    pub lookup_outcome:Option<u8>,
    pub lookup_outcome_message:Option<String>,
    pub valid_number:Option<String>,
    pub reachable:Option<String>
}
