use crate::imports::*;

#[derive(Deserialize)]
pub struct Request {
    pub message : String, pub status : String, 
}

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[route("/")]
    Home,
    #[route("/next-claim-time")]
    NextTime,
    #[route("/claim")]
    Claim,
}