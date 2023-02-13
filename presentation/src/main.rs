#![allow(clippy::derive_partial_eq_without_eq)]
#![allow(non_snake_case)]

mod background;
mod footer;
mod header;
// mod toc;
pub mod utils;

use crate::background::Background;
use crate::footer::Footer;
use crate::header::Header;
use dioxus::prelude::*;
use fermi::*;

// Flag for Dark/Light mode.
pub static IS_DARK_MODE: Atom<bool> = |_| false;

fn App(cx: Scope) -> Element {
    utils::use_dark_mode(cx);
    cx.render(rsx! {
        Background{
            Header{}
            div{id: "contents-container"}
            Footer{}
        }

    })
}

fn main() {
    dioxus::web::launch(App);
}
