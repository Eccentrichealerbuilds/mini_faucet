pub use dioxus::prelude::*;
pub use serde::{Deserialize, /*Serialize*/};
pub use reqwest::{get};

pub use crate::components::claim_input::next_claim_input;
pub use crate::custom::Request;

pub const WALLET_SUBMIT_CSS : Asset = asset!("assets/tailwind.css");