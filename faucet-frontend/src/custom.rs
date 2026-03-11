use crate::imports::*;

#[derive(Deserialize)]
pub struct Request {
    pub message : String, pub status : String, 
}