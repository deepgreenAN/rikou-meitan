mod more_button;
mod toc;

use crate::components::{AccordionEpisodes, MovieCard, MovieContainer, Player};
use domain::{episode::Episode, movie_clip::MovieClip, Date};
use more_button::MoreButton;
use toc::{Toc, TocContent};

use dioxus::prelude::*;
use fake::{Fake, Faker};
// use gloo_timers::future::TimeoutFuture;

pub fn Home(cx: Scope) -> Element {
    let orikou_desc_str = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/contents/orikou_desc.html"
    ));

    let episodes_ref = use_ref(cx, || Option::<Vec<Episode>>::None);
    let movie_clips_ref = use_ref(cx, || Option::<Vec<MovieClip>>::None);

    let episode_start: Date = (2023, 1, 1).try_into().expect("Date sanity check");
    let episode_end: Date = (2024, 1, 1).try_into().expect("Date sanity check");

    use_effect(cx, (), {
        to_owned![episodes_ref, movie_clips_ref];

        |_| async move {
            // TimeoutFuture::new(3000).await;
            episodes_ref.set(Some(
                (0..10)
                    .map(|_| (episode_start..episode_end).fake::<Episode>())
                    .collect::<Vec<_>>(),
            ));

            movie_clips_ref.set(Some(
                (0..6)
                    .map(|_| Faker.fake::<MovieClip>())
                    .collect::<Vec<_>>(),
            ))
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
                            Player{id:"orikou-desc-movie-player", video_id:"B7OPlsdBuVc", range:None}
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
                    }
                    MoreButton{to:"/episodes"}
                }
                TocContent{
                    id: "clip",
                    title: "クリップ",
                    MovieContainer{
                        movie_clips_ref.read().as_ref().map(|movie_clips|{
                            rsx!{
                                movie_clips.iter().enumerate().map(|(i, movie_clip)|{
                                    rsx!{
                                        MovieCard{
                                            key:"{i}",
                                            date: movie_clip.create_date(),
                                            range: Some(movie_clip.range().clone()),
                                            title: movie_clip.title(),
                                            movie_url: movie_clip.url().clone(),
                                            id: format!("movie-clip-{i}"),
                                        }
                                    }
                                })
                            }
                        })
                    }
                    MoreButton{to:"/clips"}
                }
            }
        }
    })
}
