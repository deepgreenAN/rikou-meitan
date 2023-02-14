mod toc;

use dioxus::prelude::*;
use toc::{Toc, TocContent};

pub fn Home(cx: Scope) -> Element {
    let orikou_desc_str = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/contents/orikou_desc.html"
    ));

    cx.render(rsx! {
        div { id: "home-container",
            Toc{},
            div { id: "toc-contents-container",
                TocContent{
                    id: "orikou-desc".to_string(),
                    title: "おりコウとは".to_string(),
                    div { dangerous_inner_html: "{orikou_desc_str}"}
                }
            }
        }
    })
}
