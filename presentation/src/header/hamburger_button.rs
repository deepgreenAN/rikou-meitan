use crate::include_str_from_root;

use dioxus::prelude::*;

#[derive(Props)]
pub struct HiddenMenuButtonProps<'a> {
    pub onclick: EventHandler<'a, MouseEvent>,
}

pub fn HamburgerButton<'a>(cx: Scope<'a, HiddenMenuButtonProps<'a>>) -> Element {
    let button_svg = include_str_from_root!("images/release/hamburger-menu.svg");

    cx.render(rsx! {
        div {
            class: "hamburger-menu-button",
            onclick: move |e|{cx.props.onclick.call(e)},
            dangerous_inner_html: "{button_svg}"
        }
    })
}
