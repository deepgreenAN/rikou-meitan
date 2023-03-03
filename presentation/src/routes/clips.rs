mod edit_clip;

use crate::components::{AddButton, MovieCard, MovieContainer, IntersectionBottom};
use crate::utils::use_overlay;
use domain::{movie_clip::MovieClip, Date};
use edit_clip::EditMovieClip;

use dioxus::prelude::*;
use fake::Fake;
use gloo_intersection::IntersectionObserverHandler;
use strum::IntoEnumIterator;
use strum_macros::{Display, EnumIter as EnumIterMacro, EnumString};
use std::rc::Rc;

enum EditMovieClipOpen {
    Modify(MovieClip),
    Add,
    Close,
}

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

    // AddMovieClip関連
    let edit_movie_clip_open = use_state(cx, || EditMovieClipOpen::Close);
    let overlay_state = use_overlay(cx, 2);

    // 新規追加モーダルを開いたときの処理
    let open_edit_movie_clip = move |_| {
        edit_movie_clip_open.set(EditMovieClipOpen::Add);
        overlay_state.activate().expect("Cannot overlay activate");
    };

    // モーダルを閉じたときの処理
    let close_edit_movie_clip = move |_| {
        edit_movie_clip_open.set(EditMovieClipOpen::Close);
        overlay_state.deactivate();
    };

    // 状態の初期化
    use_effect(cx, (), {
        to_owned![movie_clips_ref];
        |_| async move {
            let start = Date::from_ymd(2018, 12, 7).expect("Date sanity check");
            let end = Date::from_ymd(2023, 3, 3).expect("Date sanity check");
            let mut movie_clips = (0..20)
                .map(|_| (start..end).fake::<MovieClip>())
                .collect::<Vec<_>>();

            movie_clips.sort_by_key(|movie_clip| movie_clip.create_date());
            movie_clips_ref.set(Some(movie_clips));
        }
    });

    // 底が交差するときのオブザーバー
    let intersection_handler = cx.use_hook(||{
        let handler = IntersectionObserverHandler::new({
            to_owned![movie_clips_ref];
            move |entries, _| {
                let target_entry = entries.into_iter().next().expect("Observe sanity check");
                if target_entry.is_intersecting() {
                    let start = Date::from_ymd(2023, 3, 3).expect("Date sanity check");
                    let end = Date::from_ymd(2024, 1, 1).expect("Date sanity check");
                    let mut new_movie_clips = (0..20)
                        .map(|_| (start..end).fake::<MovieClip>())
                        .collect::<Vec<_>>();

                    new_movie_clips.sort_by_key(|movie_clip| movie_clip.create_date());
                    movie_clips_ref.with_mut(|movie_clips| {
                        if let Some(movie_clips) = movie_clips.as_mut() {
                            movie_clips.append(&mut new_movie_clips);
                        }
                    });
                }
            }
        })
        .expect("Intersection Handler Error");
        Rc::new(handler)
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
                    AddButton {onclick: open_edit_movie_clip}
                }

                match edit_movie_clip_open.get() {
                    EditMovieClipOpen::Add => rsx!{
                        EditMovieClip{
                            on_submit: move |new_movie_clip|{
                                close_edit_movie_clip(());
                                movie_clips_ref.with_mut(|movie_clips|{
                                    if let Some(movie_clips) = movie_clips.as_mut() {
                                        movie_clips.push(new_movie_clip);
                                        movie_clips.sort_by_key(|movie_clip|{movie_clip.create_date()});
                                    }
                                });
                            },
                            on_cancel: close_edit_movie_clip
                        }
                    },
                    EditMovieClipOpen::Modify(movie_clip) => rsx!{
                        EditMovieClip{
                            base_movie_clip: movie_clip.clone(),
                            on_submit: move |modified_movie_clip: MovieClip|{
                                close_edit_movie_clip(());
                                movie_clips_ref.with_mut(|movie_clips|{
                                    if let Some(movie_clips) = movie_clips.as_mut() {
                                        movie_clips.iter_mut().for_each(|movie_clip|{
                                            if movie_clip.id() == modified_movie_clip.id() {
                                                *movie_clip = modified_movie_clip.clone();
                                            }
                                        })
                                    }
                                });
                            },
                            on_cancel: close_edit_movie_clip
                        }
                    },
                    EditMovieClipOpen::Close => rsx!{Option::<VNode>::None}
                }
            }
            MovieContainer{
                movie_clips_ref.read().as_ref().map(|movie_clips|{
                    rsx!{
                        movie_clips.iter().enumerate().map(|(i, movie_clip)|{
                            let movie_clip = movie_clip.clone();
                            rsx!{
                                MovieCard{
                                    key:"{i}",
                                    date: movie_clip.create_date(),
                                    range: movie_clip.range().clone(),
                                    title: movie_clip.title(),
                                    movie_url: movie_clip.url().clone(),
                                    id: format!("movie-clip-{i}"),
                                    on_modify: move |_|{
                                        edit_movie_clip_open.set(EditMovieClipOpen::Modify(movie_clip.clone()));
                                        overlay_state.activate().expect("Cannot Overlay activate.");
                                    }
                                }
                            }
                        })
                    }
                })
            }
            
            IntersectionBottom{intersection_handler: intersection_handler.clone()}
        }
    })
}
