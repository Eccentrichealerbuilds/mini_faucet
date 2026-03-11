mod imports;
mod components;
mod custom;

use crate::imports::*;

fn main() {
    dioxus::launch(app);
}

fn app() -> Element {
    rsx!{
        Router::<Route>{}
    }
}