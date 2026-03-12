use crate::imports::*;

#[derive(Deserialize)]
pub struct Request {
    pub message: String,
    pub status: String,
}

#[derive(Clone, Routable, PartialEq)]
pub enum MyRoute {
    #[route("/")]
    Home,
    #[route("/next-claim-time")]
    NextTime,
    #[route("/claim")]
    Claim,
    #[route("/my-balance")]
    MyBalance,
    #[route("/faucet-balance")]
    FaucetBalance,
    #[route("/contribute")]
    Contribute,
}
