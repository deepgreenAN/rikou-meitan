use dioxus::prelude::*;
use fermi::use_atom_state;
use gloo_utils::document_element;
use web_sys::HtmlElement;
use wasm_bindgen::{JsCast, UnwrapThrowExt};

pub fn ModeChangeButton(cx: Scope) -> Element {
    let is_dark_mode = use_atom_state(&cx, crate::IS_DARK_MODE);

    let button_svg = match is_dark_mode.get() {
        true => include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/images/release/bottle-mode.svg"
        )),
        false => include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/images/release/moon-mode.svg"
        )),
    };

    cx.render(rsx! {
        div { id: "mode-change-button", 
            onclick: move |_|{
                is_dark_mode.modify(|flag|{
                    let root_element = document_element();
                    let root_style = root_element.unchecked_into::<HtmlElement>().style();
                    
                    match *flag {
                        true => { // ライトモードに変換
                            root_style.set_property("--primary-color", "var(--light-primary-color)").unwrap_throw();
                            root_style.set_property("--primary-bg-color", "var(--light-primary-bg-color)").unwrap_throw();
                            false
                        },
                        false => { // ダークモードに変換
                            root_style.set_property("--primary-color", "var(--dark-primary-color)").unwrap_throw();
                            root_style.set_property("--primary-bg-color", "var(--dark-primary-bg-color)").unwrap_throw();
                            true
                        }
                    }
                });
                
            },
            div { id: "mode-change-cover"}
            div { id: "mode-change-svg", dangerous_inner_html: "{button_svg}" }
        }
    })
}
