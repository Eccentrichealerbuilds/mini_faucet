mod imports;
mod components;
mod custom;

use crate::imports::*;

fn main() {
    dioxus::launch(next_claim_input);
}