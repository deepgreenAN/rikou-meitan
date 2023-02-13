use dioxus::prelude::*;

pub fn Footer(cx: Scope) -> Element {
    cx.render(rsx! {
        div { id: "footer-container",
            div { id: "footer-center",
                span {id:"copy-right", "© 2023 dgAN"}
                span {id:"maintenance-repo",
                    "このサイトは"
                    a {href: "https://github.com/deepgreenAN/rikou-meitan", "こちら"}
                    "でメンテナンスされています．"
                }
            }
        }
    })
}
