use serde::{Deserialize, Serialize};
use serde;

#[derive(Deserialize, Debug)]
pub struct start_verify_response{
    pub request_id:String,
    pub status:String
}