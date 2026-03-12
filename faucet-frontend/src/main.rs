mod components;
mod custom;
mod imports;

use crate::imports::*;

fn main() {
    dioxus::launch(app);
}

fn app() -> Element {
    rsx! {
        Router::<MyRoute>{}
    }
}
