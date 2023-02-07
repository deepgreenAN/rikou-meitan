mod logo;
mod mode_change_button;

use logo::TitleLogo;
use mode_change_button::ModeChangeButton;

use dioxus::prelude::*;

pub fn Header(cx: Scope) -> Element {
    cx.render(rsx! {
        TitleLogo{}
        ModeChangeButton{}
    })
}
