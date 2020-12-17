#![allow(warnings)]
#[macro_use]
extern crate dotenv_codegen;
mod number_insights{
    pub mod insights;
}
mod sms{
    pub mod OutboundSms;
    pub mod SmsResponse;
}
mod request{
    pub mod vonageRequest;
}
mod verify{
    pub mod start_verify;
    pub mod start_verify_response;
    pub mod check_verify;
}
use crate::request::vonageRequest::vonage_request;
use crate::sms::OutboundSms::OutboundSms;
use crate::verify::start_verify::start_verify;
use crate::verify::check_verify::check_verify;
use crate::number_insights::insights::insight_request;
use clap::{App, Arg};
use dotenv::dotenv;
use std::env;
use std::collections::HashMap;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let req = vonage_request{api_key:String::from(dotenv!("VONAGE_API_KEY")), api_secret:String::from(dotenv!("VONAGE_API_SECRET")).to_string()};
    let mut params:HashMap<String,String> = HashMap::new();
    let matches = App::new("Vonage Rust SDK")
        .version("0.1.0")
        .author("Steve Lorello <steve.lorello@vonage.com>")
        .arg(Arg::with_name("snippet")
            .short("s")
            .long("snippet")
            .takes_value(true)
            .help("The snippet you want to run. Options: sendSms, startVerify, checkVerify, basicInsight, standardInsight, advancedInsight"))
        .arg(Arg::with_name("code")
            .short("c")
            .long("code")
            .takes_value(true)
            .help("The code for a verify request - if you are checking the request"))
        .arg(Arg::with_name("request_id")
            .short("id")
            .long("request_id")
            .takes_value(true)
            .help("This is the request id for the verify request you're verifying"))
        .arg(Arg::with_name("text")
            .short("x")
            .long("text")
            .takes_value(true)
            .help("The text to send in the SMS"))
        .arg(Arg::with_name("to_number")
            .short("n")
            .long("to_number")
            .takes_value(true)
            .help("The number to send the SMS to"))
        .arg(Arg::with_name("insight_number")            
            .long("insight_number")
            .takes_value(true)
            .help("The number to get insights on"))
        .get_matches();
    
    match matches.value_of("snippet"){
        None => println!("ERROR: snippet argument is required"),
        Some(s) => {
            match s{
                "sendSms"=>{
                    match matches.value_of("to_number"){
                        None => println!("ERROR: missing to_number parameter"),
                        Some(to)=>{
                            match matches.value_of("text"){
                                None =>println!("ERROR: missing text parameter"),
                                Some(text)=>{
                                    params.insert(String::from("to"), String::from(to));
                                    params.insert(String::from("from"), String::from(dotenv!("FROM_NUMBER")));
                                    params.insert(String::from("text"), String::from(text));    
                                    let sms = OutboundSms{args:params.clone(), request:req.clone()};
                                    let response = sms.send().await.unwrap();
                                    println!("{:?}",response);
                                }
                            };
                        }
                    };
                },
                "startVerify"=>{
                    match matches.value_of("to_number"){
                        None=>println!("ERROR: Missing parameter to_number"),
                        Some(to_number)=>{
                            params.insert("number".into(), to_number.into());
                            params.insert("brand".into(), "Steve's pizza".into());
                            params.insert(String::from("workflow_id"), String::from("6"));
                            let verify = start_verify{args:params.clone(), request: req.clone()};
                            let response = verify.send().await.unwrap();
                            println!("{:?}", response);                            
                        }
                    }
                },
                "checkVerify"=>{
                    match matches.value_of("request_id"){
                        None=>println!("ERROR: Missing Parameter request_id"),
                        Some(request_id)=>{
                            match matches.value_of("code"){
                                None=>println!("Error: Missing Parameter code"),
                                Some(code)=>{
                                    params.clear();
                                    params.insert(String::from("request_id"), String::from(request_id));
                                    params.insert(String::from("code"), String::from(code));
                                    let check = check_verify{args:params.clone(),request:req.clone()};
                                    let response = check.send().await;
                                    println!("{:?}",response);
                                }
                            };
                        }
                    };
                },
                "basicInsight"=>{
                    match matches.value_of("insight_number"){
                        None => println!("ERROR: Missing parameter insight_number"),
                        Some(number)=>{
                            params.clear();
                            params.insert(String::from("number"),String::from(number));
                            let insight = insight_request{args:params.clone(), request:req.clone()};
                            let response = insight.get_basic().await;
                            println!("{:?}", response);
                        }
                    }
                },
                "standardInsight"=>{
                    match matches.value_of("insight_number"){
                        None => println!("ERROR: Missing parameter insight_number"),
                        Some(number)=>{
                            params.clear();
                            params.insert(String::from("number"),String::from(number));
                            let insight = insight_request{args:params.clone(), request:req.clone()};
                            let response = insight.get_standard().await;
                            println!("{:?}", response);
                        }
                    }
                }
                "advancedInsight"=>{
                    match matches.value_of("insight_number"){
                        None => println!("ERROR: Missing parameter insight_number"),
                        Some(number)=>{
                            params.clear();
                            params.insert(String::from("number"),String::from(number));
                            let insight = insight_request{args:params.clone(), request:req.clone()};
                            let response = insight.get_advanced_sync().await;
                            println!("{:?}", response);
                        }
                    }
                }
                _=> println!("ERROR: UNRECOGNIZED snippet")
            };
        }
    };
}
