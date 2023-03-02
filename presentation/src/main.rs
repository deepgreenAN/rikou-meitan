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
use crate::routes::{Clips, EpisodesPage, Home, NotFound};

use dioxus::prelude::*;
use dioxus_router::{Route, Router};
use fermi::*;
use std::collections::VecDeque;

// ダークモード・ライトモードのフラッグ.
pub static IS_DARK_MODE: Atom<bool> = |_| false;

// 現在再生されているプレーヤーのID．
pub static PLAYING_PLAYER_ID: Atom<Option<String>> = |_| None;

// アクティブなプレーヤーのID
pub const ACTIVE_PLAYER_NUMBER: usize = 3;
pub static ACTIVE_PLAYER_IDS: Atom<VecDeque<String>> =
    |_| VecDeque::with_capacity(ACTIVE_PLAYER_NUMBER);

fn App(cx: Scope) -> Element {
    use_init_atom_root(cx);
    utils::use_dark_mode(cx);
    cx.render(rsx! {
        Background{
            Router {
                Header{}
                div { id: "contents-container",
                    Route { to: "", NotFound{}}
                    Route { to: "/", Home{}}
                    Route { to: "/episodes", EpisodesPage{}}
                    Route { to: "/clips", Clips{}}
                }
                Footer{}
            }

        }
    })
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    dioxus_web::launch(App);
}
