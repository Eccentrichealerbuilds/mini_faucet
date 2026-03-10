use crate::blockchain::imports::*;


#[get("/faucet-address")]
pub async fn faucet_addy() -> String {
    dotenv::dotenv().ok();
    match std::env::var("CONTRACT_ADDRESS"){
        Ok(ca) => ca,
        Err(e) => e.to_string(),
    } 
}


#[get("/faucet-balance")]
pub async fn faucet_balance() -> impl Responder{
    let balance = spawn_blocking(||{ get_faucet_balance()}) ;
    match balance {
        result => result.await.unwrap(),
    }
}


#[get("/my-balance/{address}")]
pub async fn address_balance(addy: web::Path<String>) -> impl Responder{
    let address = addy.into_inner();
    let wallet_balance =  get_wallet_balance(address).await;
    wallet_balance
}


#[get("/next-claim-time/{address}")]
pub async fn next_claim(address: web::Path<String>) -> impl Responder {
    let address = address.into_inner();
    let next_time = tokio::task::spawn_blocking(|| nextClaim(address));
    match next_time {
        result => result.await.unwrap()
    }
}