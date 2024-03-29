#![allow(clippy::derive_partial_eq_without_eq)]
#![allow(non_snake_case)]

mod domain;
mod domain_error;
mod domain_form;
mod form_component;
mod validation_input_component;

use dioxus::prelude::*;
use form_component::ValidationForm;

fn App(cx: Scope) -> Element {
    let style_str = include_str!("../assets/validation_input.css");

    cx.render(rsx! {
        style {"{style_str}"}
        ValidationForm {}
    })
}

fn main() {
    use wasm_bindgen::UnwrapThrowExt;
    console_log::init_with_level(log::Level::Info).unwrap_throw();
    dioxus_web::launch(App);
}
