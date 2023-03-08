mod edit_clip;

use crate::components::{MovieCard, MovieContainer, IntersectionBottom, Quiz, VideoPageMenu};
use crate::utils::use_overlay;
use domain::movie_clip::MovieClip;
use edit_clip::EditMovieClip;

use dioxus::prelude::*;
use fake::{Fake, Faker};
use gloo_intersection::IntersectionObserverHandler;
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
    #[strum(serialize = "作成日")]
    CreateDate,
    #[strum(serialize = "Like")]
    Like,
}

#[derive(Props, PartialEq)]
pub struct ClipsPageProps {
    #[props(default = false)]
    admin: bool
}

pub fn ClipsPage(cx: Scope<ClipsPageProps>) -> Element {
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
            let mut movie_clips = (0..20)
                .map(|_| Faker.fake::<MovieClip>())
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
                    let mut new_movie_clips = (0..20)
                        .map(|_| Faker.fake::<MovieClip>())
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

    // 新しく追加するときの処理
    let add_submitted_clip = move |new_movie_clip|{
        close_edit_movie_clip(());
        movie_clips_ref.with_mut(|movie_clips|{
            if let Some(movie_clips) = movie_clips.as_mut() {
                movie_clips.push(new_movie_clip);
                movie_clips.sort_by_key(|movie_clip|{movie_clip.create_date()});
            }
        });
    };

    // 編集のときの処理
    let modify_submitted_clip = move |modified_movie_clip: MovieClip|{
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
    };

    // 削除のときの処理
    let remove_clip = move |clip_for_remove: MovieClip|{
        close_edit_movie_clip(());
        movie_clips_ref.with_mut(|movie_clips|{
            if let Some(movie_clips) = movie_clips.as_mut() {
                movie_clips.retain(|movie_clip|{
                    movie_clip.id() != clip_for_remove.id()
                });
            }
        });
    };


    cx.render(rsx! {
        div { id: "clips-container",
            div {id: "clips-title-container",
                h2 {id: "clips-title",
                    match cx.props.admin {
                        true => "クリップ(管理者モード)",
                        false => "クリップ"
                    }
                }
            }
            VideoPageMenu{
                _enum_type: SortType::default()
                on_click_add_button: open_edit_movie_clip,
                on_change_sort_select: move |e: FormEvent|{log::info!("{}", e.value)},
            }
            match edit_movie_clip_open.get() {
                EditMovieClipOpen::Add => rsx!{
                    EditMovieClip{
                        on_submit: add_submitted_clip,
                        on_cancel: close_edit_movie_clip,
                    }
                },
                EditMovieClipOpen::Modify(movie_clip) => rsx!{
                    Quiz{
                        on_cancel: close_edit_movie_clip,
                        admin: cx.props.admin,
                        match cx.props.admin {
                            true => rsx!{ // 管理者の時
                                EditMovieClip{
                                    base_movie_clip: movie_clip.clone(),
                                    on_submit: modify_submitted_clip,
                                    on_cancel: close_edit_movie_clip,
                                    on_remove: remove_clip
                                }
                            },
                            false => rsx!{ // 管理者でない時
                                EditMovieClip{
                                    base_movie_clip: movie_clip.clone(),
                                    on_submit: modify_submitted_clip,
                                    on_cancel: close_edit_movie_clip,
                                }
                            }
                        }
                    }

                },
                EditMovieClipOpen::Close => rsx!{Option::<VNode>::None}
            }
            
            MovieContainer{
                movie_clips_ref.read().as_ref().map(|movie_clips|{
                    rsx!{
                        movie_clips.iter().map(|movie_clip|{
                            let movie_clip = movie_clip.clone();
                            let id = movie_clip.id();
                            rsx!{
                                MovieCard{
                                    key:"{id}",
                                    date: movie_clip.create_date(),
                                    range: movie_clip.range().clone(),
                                    title: movie_clip.title(),
                                    movie_url: movie_clip.url().clone(),
                                    id: format!("movie-clip-{id}"),
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
