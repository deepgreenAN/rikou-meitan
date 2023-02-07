mod logo;

use logo::TitleLogo;

use dioxus::prelude::*;

pub fn Header(cx: Scope) -> Element {
    cx.render(rsx! {
        TitleLogo{}
    })
}
