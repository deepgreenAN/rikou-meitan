mod hamburger_button;
mod header_menu;
mod hidden_menu;
mod logo;
mod mode_change_button;

pub use hamburger_button::HamburgerButton;
use header_menu::{HeaderMenu, HeaderMenuItem};
pub use hidden_menu::{HiddenMenu, HiddenMenuItem};
use logo::TitleLogo;
pub use mode_change_button::ModeChangeButton;

use crate::utils::{use_overlay, use_scroll_lock};

use dioxus::prelude::*;
use dioxus_router::Link;

pub fn Header(cx: Scope) -> Element {
    // 隠しメニューが開いているかどうか
    let is_hidden_menu_open = use_state(cx, || false);
    let scroll_state = use_scroll_lock(cx);
    let overlay_state = use_overlay(cx, 2);

    let open_hidden_menu = move |_| {
        is_hidden_menu_open.set(true);
        scroll_state.lock();
        overlay_state.activate().expect("Cannot Overlay activate");
        overlay_state
            .add_event_listener("click", {
                to_owned![overlay_state, is_hidden_menu_open, scroll_state];
                move |_| {
                    is_hidden_menu_open.set(false);
                    scroll_state.unlock();
                    overlay_state.deactivate();
                }
            })
            .expect("Overlay cannot added event listener");
    };

    let close_hidden_menu = move |_| {
        is_hidden_menu_open.set(false);
        scroll_state.unlock();
        overlay_state.deactivate();
    };

    cx.render(rsx! {
        div { id:"header-container",
            div { id: "header-left", TitleLogo{} }
            div { id: "header-right",
                ModeChangeButton{}
                HamburgerButton{
                    onclick: open_hidden_menu
                }
            }
            // 以下はabsolute
            div { id: "top-bar"}
            HeaderMenu{
                HeaderMenuItem{Link{ to:"/", "ホーム"}}
                HeaderMenuItem{Link{ to:"/episodes", "エピソード"}}
                HeaderMenuItem{Link{ to:"/clips", "クリップ"}}
                HeaderMenuItem{Link{ to:"/originals", "コラボ配信"}}
                HeaderMenuItem{Link{ to:"/kirinukis", "切り抜き"}}
            }
            is_hidden_menu_open.get().then(||{
                rsx! {
                    HiddenMenu{
                        HiddenMenuItem{
                            Link{ to:"/","ホーム", onclick: close_hidden_menu}
                        }
                        HiddenMenuItem{
                            Link{ to:"/episodes","エピソード", onclick: close_hidden_menu}
                        }
                        HiddenMenuItem{
                            Link{ to:"/clips","クリップ", onclick: close_hidden_menu}
                        }
                        HiddenMenuItem{
                            Link{ to:"/originals", "コラボ配信", onclick: close_hidden_menu}
                        }
                        HiddenMenuItem{
                            Link{ to:"/kirinukis", "切り抜き", onclick: close_hidden_menu}
                        }
                    }
                    div {
                        id: "hidden-menu-hamburger-button",
                        HamburgerButton{
                            onclick: close_hidden_menu,
                        }
                    }
                }
            })
        }
    })
}
