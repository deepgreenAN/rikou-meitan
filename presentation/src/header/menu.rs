use dioxus::prelude::*;

pub fn Menu(cx: Scope) -> Element {
    cx.render(rsx! {
        div { id: "header-menu",
            div { class: "header-menu-item", "メニュー１"}
            div { class: "header-menu-item", "メニュー２"}
            // div { class: "header-menu-item", "メニュー３"}
        }
    })
}
