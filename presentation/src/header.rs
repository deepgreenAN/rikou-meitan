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

use dioxus::prelude::*;
use gloo_events::{EventListener, EventListenerOptions};
use std::cell::Cell;
use std::rc::Rc;

pub fn Header(cx: Scope) -> Element {
    // 隠しメニューが開いているかどうか
    let is_hidden_menu_open = use_state(&cx, || false);

    // スクロールを禁止するための状態
    let scroll_rock_state = cx.use_hook(|_| Rc::new(Cell::new(Option::<Vec<EventListener>>::None)));

    // use_effectは遅延があるためクロージャーとして毎回実行．
    let set_scroll_rock = |is_rocked: bool| {
        if is_rocked {
            let document = gloo_utils::document();
            let options = EventListenerOptions {
                passive: false,
                ..Default::default()
            };
            scroll_rock_state.set(Some(vec![
                EventListener::new_with_options(&document, "wheel", options, move |e| {
                    e.prevent_default();
                }),
                EventListener::new_with_options(&document, "touchmove", options, move |e| {
                    e.prevent_default();
                }),
            ]));
        } else {
            scroll_rock_state.take(); // 破棄
        }
    };

    cx.render(rsx! {
        div { id:"header-container",
            div { id: "header-left", TitleLogo{} }
            div { id: "header-right",
                ModeChangeButton{}
                HamburgerButton{
                    onclick: move |_| {
                        is_hidden_menu_open.set(true);
                        set_scroll_rock(true);
                    }
                }
            }
            // 以下はabsolute
            div { id: "top-bar"}
            HeaderMenu{
                HeaderMenuItem{"メニュー1"}
                HeaderMenuItem{"メニュー2"}
            }
            is_hidden_menu_open.get().then(||{
                rsx! {
                    HiddenMenu{
                        HiddenMenuItem{"メニュー1"}
                        HiddenMenuItem{"メニュー2"}
                        HiddenMenuItem{"メニュー3"}
                    }
                    div { id: "hidden-menu-overlay",
                        onclick: move |_|
                        {
                            is_hidden_menu_open.set(false);
                            set_scroll_rock(false);
                        }
                    }
                    div {
                        id: "hidden-menu-hamburger-button",
                        HamburgerButton{
                            onclick: move |_| {
                                is_hidden_menu_open.set(false);
                                set_scroll_rock(false);
                            },
                        }
                    }
                }
            })
        }
    })
}
