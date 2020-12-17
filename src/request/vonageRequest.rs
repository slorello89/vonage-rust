use serde::{Deserialize, Serialize};
use reqwest::header::{HeaderMap, HeaderValue, CONTENT_TYPE};
use std::collections::HashMap;
use serde::de::DeserializeOwned;
use async_trait::async_trait;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct vonage_request{
    pub api_key: String,
    pub api_secret: String
}

impl vonage_request{    

    pub async fn send(&self, body:HashMap<String,String>, url: String){
        let client = reqwest::Client::new();
        match client.post(&url)
        .form(&body)
        .send().await{
            Ok(res) => {
                println!("response: {} ", res.status());
                match res.text().await{
                    Err(e) => println!("Error: {}", e),
                    Ok(y)=> println!("{}",y)
                };
            },
            Err(err) => println!("Error: {}", err)
        };
    }

    pub async fn get<T>(&self, query_params:HashMap<String,String>, url: String)->Option<T> where T:DeserializeOwned{
        let mut request_uri:String = url.clone();
        request_uri.push_str(&String::from("?"));
        for(arg_name,value) in &query_params{
            request_uri.push_str(&String::from(format!("{}={}&",arg_name,value)));
        }
        println!("{}",request_uri);
        match reqwest::get(&request_uri).await{
                Ok(res)=>{
                    let response:T = res.json().await.unwrap();
                    Some(response)
                }
                Err(err) =>{
                    println!("{:?}",err);
                    None
                }
            }
    }
    pub async fn sendReturnGeneric<T>(&self, body:HashMap<String,String>, url: String) ->Option<T> where T:DeserializeOwned{
        let client = reqwest::Client::new();
        match client.post(&url)
            .form(&body)
            .send().await{
                Ok(res) => {
                    let response:T = res.json().await.unwrap();
                    Some(response)
                }
                Err(err) => {
                    None
                }
            }
    }
}