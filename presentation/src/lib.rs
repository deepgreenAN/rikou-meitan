#![allow(clippy::derive_partial_eq_without_eq)]
#![allow(non_snake_case)]

mod background;
mod components;
mod footer;
mod header;
mod routes;
mod utils;

use crate::background::Background;
use crate::footer::Footer;
use crate::header::Header;
// 以下はroutes
use crate::routes::{
    AdminPage, ClipsPage, EpisodesPage, HomePage, NotFoundPage, VideosPage, VideosPageProps,
};
// 以下はcomponents
use crate::components::FlowScript;

use dioxus::prelude::*;
use dioxus_router::{Route, Router};
use fermi::*;
use std::collections::VecDeque;

// グローバルアロケイター
#[cfg(target_arch = "wasm32")]
use lol_alloc::{FreeListAllocator, LockedAllocator};

#[cfg(target_arch = "wasm32")]
#[global_allocator]
static ALLOCATOR: LockedAllocator<FreeListAllocator> =
    LockedAllocator::new(FreeListAllocator::new());

// ダークモード・ライトモードのフラッグ.
pub static IS_DARK_MODE: Atom<bool> = |_| false;

// 現在再生されているプレーヤーのID．
pub static PLAYING_PLAYER_ID: Atom<Option<String>> = |_| None;

// アクティブなプレーヤーのID
pub const ACTIVE_PLAYER_NUMBER: usize = 3;
pub static ACTIVE_PLAYER_IDS: Atom<VecDeque<String>> =
    |_| VecDeque::with_capacity(ACTIVE_PLAYER_NUMBER);

// アドミン用のパスワード
pub struct AdminPassword(pub String);

/// メインのアプリケーションの引数
#[derive(Props, PartialEq, Clone)]
pub struct AppProps {
    pub admin_password: String,
}

/// メインのアプリケーション．
pub fn App(cx: Scope<AppProps>) -> Element {
    use_init_atom_root(cx);
    utils::use_dark_mode(cx);

    use_shared_state_provider(cx, || AdminPassword(cx.props.admin_password.clone()));

    let admin = cfg!(feature = "develop");

    cx.render(rsx! {
        Background{
            Router {
                Header{}
                FlowScript{}
                div { id: "contents-container",
                    Route { to: "", NotFoundPage{}}
                    Route { to: "/", HomePage{}}
                    Route { to: "/episodes", EpisodesPage{admin: admin}}
                    Route { to: "/clips", ClipsPage{admin: admin}}
                    Route { to: "/originals", 
                        VideosPage{..VideosPageProps::<domain::video::Original>::builder().admin(admin).build()}
                    }
                    Route { to: "/kirinukis", 
                        VideosPage{..VideosPageProps::<domain::video::Kirinuki>::builder().admin(admin).build()}
                    }
                    // 以下はadmin関連
                    Route { to: "/admin", AdminPage{}}
                    Route { to: "/admin/episodes", EpisodesPage{admin:true}}
                    Route { to: "/admin/clips", ClipsPage{admin:true}}
                    Route { to: "/admin/originals", 
                        VideosPage{..VideosPageProps::<domain::video::Original>::builder().admin(true).build()}
                    }
                    Route { to: "/admin/kirinukis", 
                        VideosPage{..VideosPageProps::<domain::video::Kirinuki>::builder().admin(true).build()}
                    }
                }
                Footer{}
            }

        }
    })
}
