#![allow(clippy::derive_partial_eq_without_eq)]
#![allow(non_snake_case)]

mod background;
pub mod components;
mod footer;
mod header;
mod routes;
pub mod utils;

use crate::background::Background;
use crate::footer::Footer;
use crate::header::Header;
// 以下はroutes
use crate::routes::Home;
use crate::routes::NotFound;

use dioxus::{
    prelude::*,
    router::{Route, Router},
};
use fermi::*;

// Flag for Dark/Light mode.
pub static IS_DARK_MODE: Atom<bool> = |_| false;

fn App(cx: Scope) -> Element {
    utils::use_dark_mode(cx);
    cx.render(rsx! {
        Background{
            Router {
                Header{}
                div { id: "contents-container",
                    Route { to: "/", Home{}}
                    Route { to: "", NotFound{}}
                }
                Footer{}
            }

        }
    })
}

fn main() {
    dioxus::web::launch(App);
}
