mod more_button;
mod toc;

use crate::components::{AccordionEpisodes, Player};
use domain::episode::Episode;
use more_button::MoreButton;
use toc::{Toc, TocContent};

use dioxus::prelude::*;
use fake::{Fake, Faker};
use gloo_timers::future::TimeoutFuture;

pub fn Home(cx: Scope) -> Element {
    let orikou_desc_str = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/contents/orikou_desc.html"
    ));

    let episodes_ref = use_ref(cx, || Option::<Vec<Episode>>::None);

    use_effect(cx, (), {
        to_owned![episodes_ref];

        |_| async move {
            TimeoutFuture::new(3000).await;
            episodes_ref.set(Some(
                (0..10).map(|_| Faker.fake::<Episode>()).collect::<Vec<_>>(),
            ));
        }
    });

    cx.render(rsx! {
        div { id: "home-container",
            Toc{},
            div { id: "toc-contents-container",
                TocContent{
                    id: "orikou-desc",
                    title: "おりコウとは",
                    div { dangerous_inner_html: "{orikou_desc_str}"}
                    div { id: "orikou-desc-movie-container-outer",
                        div { id: "orikou-desc-movie-container-inner",
                            Player{id:"orikou-desc-movie-player", video_id:"B7OPlsdBuVc"}
                        }
                    }
                }
                TocContent{
                    id: "episode",
                    title: "エピソード",
                    AccordionEpisodes{
                        title: "2023",
                        episodes: episodes_ref.clone(),
                        initial_is_open: true,
                        fixed: true,
                        editable: false
                    }
                    MoreButton{to:"/episodes"}
                }
            }
        }
    })
}
