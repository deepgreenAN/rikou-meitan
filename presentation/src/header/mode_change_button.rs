use dioxus::prelude::*;
use fermi::use_atom_state;

pub fn ModeChangeButton(cx: Scope) -> Element {
    let is_dark_mode = use_atom_state(&cx, crate::IS_DARK_MODE);

    let button_svg = match is_dark_mode.get() {
        true => include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/images/release/moon-blackedge.svg"
        )),
        false => include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/images/release/bottle-blackedge-mode.svg"
        )),
    };

    cx.render(rsx! {
        div { id: "mode-change-button", onclick: move |_|{is_dark_mode.modify(|flag|{!flag})},
            dangerous_inner_html: "{button_svg}"
        }
    })
}
