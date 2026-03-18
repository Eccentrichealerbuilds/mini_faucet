use crate::prelude::*;

#[derive(Debug)]
pub enum Responder<T>{
    Success(T),
    Error(T)
}

impl<T> Responder <T> {
    pub fn unwrapper(self) -> T {
        match self {
            Responder::Success(s) => s,
            Responder::Error(e) => e,
        }
    }
}


#[derive(Debug, Deserialize)]
pub struct Resultant {
    pub message: String,
    pub status: String
}
