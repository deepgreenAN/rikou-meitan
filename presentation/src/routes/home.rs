mod toc;

use crate::components::Player;
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
                    div { id: "orikou-desc-movie-container-outer",
                        div { id: "orikou-desc-movie-container-inner",
                            Player{id:"orikou-desc-movie-player".to_string(), video_id:"B7OPlsdBuVc".to_string()}
                        }
                    }
                }
                TocContent{
                    id: "episode".to_string(),
                    title: "エピソード".to_string(),
                    "何らかのエピソード"
                }
            }
        }
    })
}
