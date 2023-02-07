#![allow(clippy::derive_partial_eq_without_eq)]
#![allow(non_snake_case)]

mod header;

use crate::header::Header;
use dioxus::prelude::*;

fn App(cx: Scope) -> Element {
    cx.render(rsx! {
        Header{}
    })
}

fn main() {
    dioxus::web::launch(App);
}
