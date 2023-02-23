#![allow(clippy::derive_partial_eq_without_eq)]
#![allow(non_snake_case)]

use dioxus::prelude::*;

fn App(cx: Scope) -> Element {
    let style_str = include_str!("./assets/hello_world.css");

    cx.render(rsx! {
        style {"{style_str}"}
        div { class: "hello", "hello, world!" }
    })
}

fn main() {
    dioxus_web::launch(App);
}
