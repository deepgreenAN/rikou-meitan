use dioxus::{prelude::*, router::Link};

pub fn NotFound(cx: Scope) -> Element {
    cx.render(rsx! {
        div { id: "not-found-container",
            div { id: "not-found-text",
                "404 Page Not Found."
            }
            div { id: "not-found-desc",
                "URLにミスがないかご確認ください"
            }
            Link {to:"/", "ホームページへ戻る"}
        }
    })
}
