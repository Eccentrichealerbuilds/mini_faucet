pub use dioxus::prelude::*;
pub use serde::{Deserialize, /*Serialize*/};
// pub use reqwest::{get};

pub use crate::components::{next_time::NextTime, home::Home, claim::Claim};
pub use crate::custom::{Request, Route};

// pub const WALLET_SUBMIT_CSS : Asset = asset!("assets/tailwind.css");