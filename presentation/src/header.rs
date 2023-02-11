mod logo;
mod menu;
mod mode_change_button;

use logo::TitleLogo;
use menu::Menu;
use mode_change_button::ModeChangeButton;

use dioxus::prelude::*;

pub fn Header(cx: Scope) -> Element {
    cx.render(rsx! {
        div { id:"header-container",
            div { id: "header-left", TitleLogo{} }
            div { id: "header-right", ModeChangeButton{}}
            // 以下はabsolute
            Menu{}
            div { id: "top-bar"}
        }
    })
}
