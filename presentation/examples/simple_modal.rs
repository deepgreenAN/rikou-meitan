#![allow(clippy::derive_partial_eq_without_eq)]
#![allow(non_snake_case)]

use dioxus::prelude::*;
use gloo_events::{EventListener, EventListenerOptions};

fn App(cx: Scope) -> Element {
    let is_show_modal = use_state(&cx, || false);
    let show_modal = move |_| is_show_modal.set(true);
    let close_modal = move |_| is_show_modal.set(false);
    let _scroll_lock = use_future(&cx, is_show_modal, |is_show_modal| async move {
        if *is_show_modal {
            let document = gloo_utils::document();
            let options = EventListenerOptions {
                passive: false,
                ..Default::default()
            };
            Some(vec![
                EventListener::new_with_options(&document, "wheel", options, move |e| {
                    e.prevent_default();
                }),
                EventListener::new_with_options(&document, "touchmove", options, move |e| {
                    e.prevent_default();
                }),
            ])
        } else {
            None
        }
    });

    let style_str = include_str!("./assets/simple_modal.css");

    cx.render(rsx! {
        style {"{style_str}"}
        button { onclick: show_modal, "モーダルを表示"}
        is_show_modal.then(||
            rsx! {
                div {
                    id: "modal-overlay", onclick: close_modal,
                    div {
                        id: "modal-content", onclick: move |e| {e.cancel_bubble()},
                        p {"これがモーダルウィンドウです。"}
                        p {button {onclick: close_modal, "キャンセル"}}
                    }
                }
            }
        )
        div {id: "main-content", "メインコンテンツ"}
    })
}

fn main() {
    dioxus::web::launch(App);
}
