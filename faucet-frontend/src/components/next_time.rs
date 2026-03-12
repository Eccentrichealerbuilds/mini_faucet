use crate::imports::*;

#[component]
pub fn NextTime() -> Element {
    let mut status = use_signal(|| "".to_string());
    let mut address = use_signal(|| "".to_string());
    let mut is_loading = use_signal(|| false);
    let mut result = use_signal(|| None);
    let claim = move |_| async move {
        is_loading.set(true);
        let val = address();
        let url = format!("https://sepolia-faucet.eccentrichealer.xyz/api/next-claim-time/{val}");
        let response1 = reqwest::get(url).await;
        if let Err(e) = response1 {
            let connection_error = e.to_string();
            is_loading.set(false);
            status.set("error".to_string());
            return result.set(Some(connection_error));
        }
        let response2 = response1.unwrap().json::<Request>().await;
        if let Err(e) = response2 {
            let json_parse_error = e.to_string();
            is_loading.set(false);
            status.set("error".to_string());
            return result.set(Some(json_parse_error));
        }
        let final_response = response2.unwrap();
        status.set(final_response.status);
        result.set(Some(final_response.message));
        is_loading.set(false);
    };

    rsx! { /*document::Stylesheet { href: WALLET_SUBMIT_CSS},*/
        div {
            h1 { "CHECK YOUR NEXT CLAIM TIME HERE" }br {}
            Link { to: MyRoute::Home, "Homepage"  }
             input { type: "text", oninput: move|e|address.set(e.value()) }
            button {onclick: claim, disabled: is_loading() || address().len() != 42 ,
                if is_loading() {
                    "⏳ Processing..."
                } else {"check claim time"}
            }
            if is_loading() {
                p{"checking....please wait.."}
            }
            if let Some(e) = result() {
                p {
                    if status() == "success" {
                        "Success: {e}"
                    }
                    else if status() == "error" {
                        "Error: {e}"
                    }
                 }
            }
         }
    }
}
