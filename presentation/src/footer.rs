use dioxus::prelude::*;
use dioxus_router::Link;

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
                Link{ id:"admin-link", to:"/admin", "管理者用ページ"}
            }
        }
    })
}
