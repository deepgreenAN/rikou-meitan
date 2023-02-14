use crate::IS_DARK_MODE;
use dioxus::{events::MouseEvent, prelude::*};
use fermi::use_read;

#[derive(Props)]
pub struct HiddenMenuItem<'a> {
    children: Element<'a>,
    onclick: Option<EventHandler<'a, MouseEvent>>,
}

pub fn HiddenMenuItem<'a>(cx: Scope<'a, HiddenMenuItem<'a>>) -> Element {
    cx.render(rsx! {
        if let Some(onclick) = &cx.props.onclick {
            rsx! {
                div {class: "hidden-menu-item", onclick: move |e| {onclick.call(e)}, &cx.props.children}
            }
        } else {
            rsx! {
                div {class: "hidden-menu-item", &cx.props.children}
            }
        }
        
    })
}

#[derive(Props)]
pub struct HiddenMenuProps<'a> {
    pub children: Element<'a>,
}

pub fn HiddenMenu<'a>(cx: Scope<'a, HiddenMenuProps<'a>>) -> Element {
    let is_dark_mode = use_read(&cx, IS_DARK_MODE);
    let background_class = match *is_dark_mode {
        true => "hidden-menu-background-dark",
        false => "hidden-menu-background-light",
    };

    cx.render(rsx! {
        div { id: "hidden-menu",
            div {id: "hidden-menu-container",
                &cx.props.children
            }
            // 以下はabsolute
            div { id: "hidden-menu-background", class: "{background_class}"}
            div { id: "hidden-menu-footer", "© 2023 dgAN"}
        }
    })
}
