use crate::prelude::*;

// #[tokio::main]
pub async fn claim_with_address(user_address: String) -> Responder<String> {
    let corrected = user_address.trim_start().trim_end();
    let url = format!("https://sepolia-faucet.eccentrichealer.xyz/api/claim/{corrected}");
    let request = Command::new("curl").arg(url).output();
    if let Ok(output) = &request {
        let result = String::from_utf8_lossy(&output.stdout);
        let parser = serde_json::from_str::<serde_json::Value>(&result);
        if let Err(error) = parser {
            let _result = error.to_string();
            return Responder::Error(format!("error connecting, please try again"));
        }
        let parser = parser.unwrap();
        let result = &parser["status"].as_str().unwrap().to_string();
        if result == "success" {
            let body = parser["message"].as_str().unwrap().to_string();
            return  Responder::Success(body);
        }
        else {
            let error_message = parser["message"].as_str().unwrap().to_string();
            return Responder::Error(error_message);
        }
    }
    let failed_result = request.unwrap_err().to_string();
    Responder::Error(failed_result)
}
