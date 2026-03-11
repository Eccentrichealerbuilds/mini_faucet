use crate::imports::*;

#[component]
pub fn Home() -> Element {
    rsx!{
        div { 
            h1 { 
                "WELCOME TO SEPOLIA FAUCET"
             }br {}
             Link {to: Route::Claim, "Claim page"} br {}
             button { 
                onclick: move |_|{ use_navigator().push(Route::NextTime);}, "check next claim time"
              }

         }
    }
}