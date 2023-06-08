use crate::include_str_from_root;
use crate::utils::set_dark_mode;

use dioxus::prelude::*;
use fermi::use_atom_state;

pub fn ModeChangeButton(cx: Scope) -> Element {
    let is_dark_mode = use_atom_state(cx, crate::IS_DARK_MODE);

    let button_svg = match is_dark_mode.get() {
        true => include_str_from_root!("images/release/bottle-mode.svg"),
        false => include_str_from_root!("images/release/moon-mode.svg"),
    };

    cx.render(rsx! {
        div { id: "mode-change-button",
            onclick: move |_|{
                is_dark_mode.modify(|flag|{
                    set_dark_mode(!*flag);
                    !flag
                });

            },
            div { id: "mode-change-cover",
                div { id: "mode-change-svg", dangerous_inner_html: "{button_svg}" }
            }
        }
    })
}
