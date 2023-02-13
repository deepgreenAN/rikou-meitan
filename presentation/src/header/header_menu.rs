use dioxus::prelude::*;

#[derive(Props)]
pub struct HeaderMenuItemProps<'a> {
    children: Element<'a>,
}

pub fn HeaderMenuItem<'a>(cx: Scope<'a, HeaderMenuItemProps<'a>>) -> Element {
    cx.render(rsx! {
        div {class: "header-menu-item", &cx.props.children}
    })
}

#[derive(Props)]
pub struct HeaderMenuProps<'a> {
    children: Element<'a>,
}

pub fn HeaderMenu<'a>(cx: Scope<'a, HeaderMenuProps<'a>>) -> Element {
    cx.render(rsx! {
        div { id: "header-menu",
            &cx.props.children
        }
    })
}
