use dioxus::prelude::*;

#[derive(Props)]
pub struct HiddenMenuButtonProps<'a> {
    pub onclick: EventHandler<'a>,
}

pub fn HamburgerButton<'a>(cx: Scope<'a, HiddenMenuButtonProps<'a>>) -> Element {
    let button_svg = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/images/release/hamburger-menu.svg"
    ));

    cx.render(rsx! {
        div {
            class: "hamburger-menu-button",
            onclick: move |_|{cx.props.onclick.call(());},
            dangerous_inner_html: "{button_svg}"
        }
    })
}
