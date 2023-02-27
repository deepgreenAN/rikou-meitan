mod edit_clip;

use crate::components::{AddButton, MovieCard, MovieContainer};
use domain::movie_clip::MovieClip;

use dioxus::prelude::*;
use fake::{Fake, Faker};
use strum::IntoEnumIterator;
use strum_macros::{Display, EnumIter as EnumIterMacro, EnumString};

#[derive(Display, EnumIterMacro, EnumString, Debug, PartialEq, Eq, Clone, Default)]
enum SortType {
    #[default]
    #[strum(serialize = "作成日時")]
    CreateDate,
    #[strum(serialize = "Like")]
    Like,
}

pub fn Clips(cx: Scope) -> Element {
    let movie_clips_ref = use_ref(cx, || Option::<Vec<MovieClip>>::None);

    use_effect(cx, (), {
        to_owned![movie_clips_ref];
        |_| async move {
            movie_clips_ref.set(Some(
                (0..20)
                    .map(|_| Faker.fake::<MovieClip>())
                    .collect::<Vec<_>>(),
            ))
        }
    });

    cx.render(rsx! {
        div { id: "clips-container",
            div {id: "clips-title-container",
                h2 {id: "clips-title", "クリップ"}
            }
            div { id: "clips-menu",
                div { id: "clips-sort-select-container",
                    select {
                        SortType::iter().map(|sort_type|{
                            rsx!{
                                option { class: "clips-sort-option", value: "{sort_type.to_string()}", selected: sort_type == SortType::default(),
                                    sort_type.to_string()
                                }
                            }
                        })
                    }
                }
                div { id: "clips-add-button",
                    AddButton {onclick: move |_|{}}
                }
            }
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
                                    on_modify: move |_|{}
                                }
                            }
                        })
                    }
                })
            }
        }
    })
}
