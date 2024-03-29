use dioxus::prelude::*;
use wasm_bindgen::{JsCast, UnwrapThrowExt};
use web_sys::HtmlElement;

pub fn set_dark_mode(is_dark_mode: bool) {
    let root_element = gloo_utils::document_element();
    let root_style = root_element.unchecked_into::<HtmlElement>().style();

    match is_dark_mode {
        true => {
            // ダークモードに設定
            root_style
                .set_property("--primary-color", "var(--dark-primary-color)")
                .unwrap_throw();
            root_style
                .set_property("--primary-color-rgb", "var(--dark-primary-color-rgb)")
                .unwrap_throw();
            root_style
                .set_property("--primary-bg-color", "var(--dark-primary-bg-color)")
                .unwrap_throw();
            root_style
                .set_property("--primary-bg-color-rgb", "var(--dark-primary-bg-color-rgb)")
                .unwrap_throw();
        }
        false => {
            // ライトモードに設定
            root_style
                .set_property("--primary-color", "var(--light-primary-color)")
                .unwrap_throw();
            root_style
                .set_property("--primary-color-rgb", "var(--light-primary-color-rgb)")
                .unwrap_throw();
            root_style
                .set_property("--primary-bg-color", "var(--light-primary-bg-color)")
                .unwrap_throw();
            root_style
                .set_property(
                    "--primary-bg-color-rgb",
                    "var(--light-primary-bg-color-rgb)",
                )
                .unwrap_throw();
        }
    }
}

pub fn use_dark_mode<T>(cx: Scope<'_, T>) {
    let setter_dark_mode = fermi::use_set(cx, crate::IS_DARK_MODE);

    use_effect(cx, (), {
        to_owned![setter_dark_mode];
        |_| async move {
            let media_query_list = gloo_utils::window()
                .match_media("(prefers-color-scheme: dark)")
                .unwrap_throw()
                .unwrap_throw();
            let browser_dark_mode = media_query_list.matches();
            set_dark_mode(browser_dark_mode);
            setter_dark_mode(browser_dark_mode);
        }
    });
}
