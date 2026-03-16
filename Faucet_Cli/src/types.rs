use crate::prelude::*;

#[derive(Debug)]
pub enum Responder<T,F>{
    Success(T),
    Error(F)
}

#[derive(Debug, Deserialize)]
pub struct Resultant {
    pub message: String,
    pub status: String
}
