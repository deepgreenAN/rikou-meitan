#![allow(clippy::derive_partial_eq_without_eq)]
#![allow(non_snake_case)]

use dioxus::prelude::*;

fn App(cx: Scope) -> Element {
    cx.render(rsx! {
        div { "hello, wasm!" }
    })
}

fn main() {
    dioxus::web::launch(App);
}
