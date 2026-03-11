use crate::imports::*;

#[component]
pub fn next_claim_input() -> Element  {
    
    let mut address = use_signal(||format!(""));
    let mut is_loading = use_signal(||false);
    let mut result = use_signal(||None);
    let claim = move |_| async move {

        is_loading.set(true);
        let val = address();
        let url = format!("https://sepolia-faucet.eccentrichealer.xyz/api/next-claim-time/{val}");
        let response = reqwest::get(url)
            .await
            .unwrap()
            .json::<Request>()
            .await
            .unwrap();

            result.set(Some(response.message));
            is_loading.set(false);
    };

    rsx!{ /*document::Stylesheet { href: WALLET_SUBMIT_CSS},*/
        div { 
            h1 { "Welcome to faucet" }br {}
             input { type: "text", oninput: move|e|address.set(e.value()) }
            button {onclick: claim, disabled: is_loading() || address() == "" ,
                if is_loading() {
                    "⏳ Processing..."
                } else {"check claim time"}    
            }
            if let Some(e) = result() {
                p { "{e}" }
            }
            else if is_loading() {
                p{"loading"}
            }
         }
    }
}