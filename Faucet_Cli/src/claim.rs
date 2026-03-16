use crate::prelude::*;

#[tokio::main]
pub async fn claim() -> Responder<String, String>{
    let mut user_address = String::new();
    io::stdin().read_line(&mut user_address).expect("can't read input");
    let corrected = user_address.trim_start().trim_end();
    let url = format!("https://sepolia-faucet.eccentrichealer.xyz/api/claim/{corrected}");
    let get_claim = get(url).await;
    if let Err(error) = get_claim {
        let result = error.to_string();
        return Responder::Error(result)
    }
    let connect_success = get_claim.unwrap().json::<Resultant>().await;
    if let Err(error) = connect_success {
        let result = error.to_string();
        return Responder::Error(result);
    }
    let result = connect_success.unwrap();
    if result.status == "success" {
        Responder::Success(result.message)
    }
    else {
        Responder::Error(result.message)
    }

}
