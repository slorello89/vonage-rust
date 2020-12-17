use serde::{Deserialize, Serialize};
use serde;

#[derive(Deserialize, Debug)]
pub struct SmsResponse{
    #[serde(alias="message-count")]
    pub message_count:String,
    pub messages:Vec<message>
}

#[derive(Deserialize,Debug)]
pub struct message{
    pub to:String,
    #[serde(alias="message-id")]
    pub message_id:String,
    pub status:String,
    #[serde(alias="remaining-balance")]
    pub remaining_balance:String,
    #[serde(alias="message-price")]
    pub message_price:String,
    pub network:String
}