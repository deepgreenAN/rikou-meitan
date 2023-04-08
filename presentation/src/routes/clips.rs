mod edit_clip;

use crate::components::{MovieCard, MovieContainer, IntersectionBottom, Quiz, VideoPageMenu};
use crate::utils::use_overlay;
use domain::movie_clip::MovieClip;
use edit_clip::EditMovieClip;

use frontend::{commands::movie_clip_commands, usecases::movie_clip_usecase};

use dioxus::prelude::*;
use gloo_intersection::IntersectionObserverHandler;
use strum_macros::{Display, EnumIter, EnumString};
use std::rc::Rc;
use std::cell::Cell;

enum EditMovieClipOpen {
    Modify(MovieClip),
    Add,
    Close,
}

#[derive(Display, EnumIter, EnumString, Debug, PartialEq, Eq, Clone, Copy, Default)]
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
    let is_load_continue = cx.use_hook(|| Rc::new(Cell::new(true)));
    let sort_type_state = use_state(cx, SortType::default);

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
    use_effect(cx, sort_type_state, {
        to_owned![movie_clips_ref, is_load_continue];
        |sort_type| async move {
            // ロードを許可
            is_load_continue.set(true);

            // データをフェッチ
            let res = match *sort_type.current() {
                SortType::CreateDate => {
                    let cmd = movie_clip_commands::OrderByCreateDateMovieClipsCommand::new(20);
                    movie_clip_usecase::order_by_create_date_movie_clips(cmd).await
                },
                SortType::Like => {
                    let cmd = movie_clip_commands::OrderByLikeMovieClipsCommand::new(20);
                    movie_clip_usecase::order_by_like_movie_clips(cmd).await
                }
            };

            match res {
                Ok(new_movie_clips) => {
                    // データが一つも取得できない場合以降のデータのロードを拒否
                    if new_movie_clips.is_empty() {
                        is_load_continue.set(false);
                    }

                    movie_clips_ref.set(Some(new_movie_clips));
                },
                Err(e) => log::error!("{}", e)
            }
        }
    });

    // 底が交差するときのオブザーバー
    let intersection_handler = cx.use_hook(||{
        let handler = IntersectionObserverHandler::new({
            to_owned![movie_clips_ref, is_load_continue, sort_type_state];
            move |entries, _| {
                let target_entry = entries.into_iter().next().expect("Observe sanity check");
                if is_load_continue.get() && target_entry.is_intersecting() {
                    {
                        to_owned![movie_clips_ref, is_load_continue, sort_type_state];
                        wasm_bindgen_futures::spawn_local(async move {
                            // 最後の値を取得
                            let last_movie_clip = movie_clips_ref.with(
                                |movie_clips_opt|{
                                    if let Some(movie_clips) = movie_clips_opt.as_ref() {
                                        movie_clips.last().cloned()
                                    } else {
                                        None
                                    }
                                }
                            );

                            if let Some(last_movie_clip) = last_movie_clip {
                                // データをフェッチ
                                let res = match *sort_type_state.current() {
                                    SortType::CreateDate => {
                                        let cmd = movie_clip_commands::OrderByCreateDateLaterMovieClipsCommand::new(&last_movie_clip,20);
                                        movie_clip_usecase::order_by_create_date_later_movie_clips(cmd).await
                                    },
                                    SortType::Like => {
                                        let cmd = movie_clip_commands::OrderByLikeLaterMovieClipsCommand::new(&last_movie_clip,20);
                                        movie_clip_usecase::order_by_like_later_movie_clips(cmd).await
                                    }
                                };
                                match res {
                                    Ok(new_movie_clips) => {
                                        // データが一つも取得できない場合以降のデータのロードを拒否
                                        if new_movie_clips.is_empty(){
                                            is_load_continue.set(false);
                                        }

                                        movie_clips_ref.with_mut(|movie_clips_vec|{
                                            if let Some(movie_clips_vec) = movie_clips_vec.as_mut() {
                                                // 重複を確認しながら挿入
                                                for new_movie_clip in new_movie_clips.into_iter() {
                                                    let is_not_contain = movie_clips_vec.iter().all(|clip|{clip.id() != new_movie_clip.id()});

                                                    if is_not_contain {
                                                        movie_clips_vec.push(new_movie_clip);
                                                    }
                                                }
                                            }
                                        })
                                    },
                                    Err(e) => log::error!("{}", e)
                                }
                            }
                        });
                    }
                }
            }
        })
        .expect("Intersection Handler Error");
        Rc::new(handler)
    });

    // 新しく追加するときの処理
    let add_submitted_clip = move |new_movie_clip: MovieClip|{
        close_edit_movie_clip(());

        // new_movie_clipデータを末尾に挿入
        {
            let new_movie_clip = new_movie_clip.clone();
            movie_clips_ref.with_mut(|movie_clips|{
                if let Some(movie_clips) = movie_clips.as_mut() {
                    movie_clips.push(new_movie_clip);
                }
            });
        }

        // API
        cx.spawn({
            to_owned![movie_clips_ref];
            async move {          
                let res = {
                    let cmd = movie_clip_commands::SaveMovieClipCommand::new(&new_movie_clip);
                    movie_clip_usecase::save_movie_clip(cmd).await
                };

                // レスポンスがエラーの場合
                if let Err(e) = res {
                    log::error!("{} Removed movie_clip: {:?}", e, new_movie_clip);

                    // new_movie_clipを削除
                    movie_clips_ref.with_mut(|movie_clips|{
                        if let Some(movie_clips) = movie_clips.as_mut() {
                            movie_clips.retain(|clip|{clip.id() != new_movie_clip.id()});
                        }
                    })
                }
            }
        });
            
    };

    // 編集のときの処理
    let modify_submitted_clip = move |modified_movie_clip: MovieClip|{
        close_edit_movie_clip(());
        let mut old_movie_clip = Option::<MovieClip>::None; // 古いデータ

        // modified_movie_clipに更新
        {
            let modified_movie_clip = modified_movie_clip.clone();
            movie_clips_ref.with_mut(|movie_clips_opt|{
                if let Some(movie_clips) = movie_clips_opt.as_mut() {
                    let found_movie_clip = movie_clips.iter_mut().find(|movie_clip|{movie_clip.id() == modified_movie_clip.id()}).expect("Cannot find modified_movie_clip");
                    // 古いデータを新しいデータに更新
                    old_movie_clip = Some(std::mem::replace(found_movie_clip, modified_movie_clip));
                }
            });
        }

        // API
        cx.spawn({
            to_owned![movie_clips_ref];
            async move {
                let res = {
                    let cmd = movie_clip_commands::EditMovieClipCommand::new(&modified_movie_clip);
                    movie_clip_usecase::edit_movie_clip(cmd).await
                };

                // レスポンスがエラーの場合
                if let Err(e) = res {
                    log::error!("{}. Roll backed movie_clip: {:?}", e, old_movie_clip);

                    if let Some(old_movie_clip) = old_movie_clip {
                        movie_clips_ref.with_mut(|movie_clips_opt|{
                            if let Some(movie_clips) = movie_clips_opt {
                                let found_movie_clip = movie_clips.iter_mut().find(|movie_clip|{movie_clip.id() == old_movie_clip.id()}).expect("Cannot find old_movie_clip");

                                // 更新したデータをロールバック
                                *found_movie_clip = old_movie_clip;
                            }
                        });
                    }
                }
            }
        });

    };

    // 削除のときの処理
    let remove_clip = move |clip_for_remove: MovieClip|{
        close_edit_movie_clip(());

        // clip_for_removeの削除
        {
            movie_clips_ref.with_mut(|movie_clips|{
                if let Some(movie_clips) = movie_clips.as_mut() {
                    movie_clips.retain(|movie_clip|{
                        movie_clip.id() != clip_for_remove.id()
                    });
                }
            });
        }

        // API
        cx.spawn({
            to_owned![movie_clips_ref, sort_type_state];
            async move {
                let res = {
                    let cmd = movie_clip_commands::RemoveMovieClipCommand::new(clip_for_remove.id());
                    movie_clip_usecase::remove_movie_clip(cmd).await
                };

                // レスポンスがエラーの場合
                if let Err(e) = res {
                    log::error!("{}. Re-pushed movie_clip: {:?}", e, clip_for_remove);

                    // 削除した要素を再び挿入してソート
                    movie_clips_ref.with_mut(|movie_clips_opt|{
                        if let Some(movie_clips) = movie_clips_opt.as_mut() {
                            movie_clips.push(clip_for_remove);

                            match *sort_type_state.current() {
                                SortType::CreateDate => {
                                    // create_dateを降順・idを昇順にソート
                                    movie_clips.sort_by(|x, y|{
                                        y.create_date().cmp(&x.create_date()).then_with(||{
                                            x.id().cmp(&y.id())
                                        })
                                    });
                                },
                                SortType::Like => {
                                    // likeを降順・idを昇順にソート
                                    movie_clips.sort_by(|x, y|{
                                        y.like().cmp(&x.like()).then_with(||{
                                            x.id().cmp(&y.id())
                                        })
                                    });
                                }
                            }
                            
                        }
                    });
                }
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
            div {id: "clips-caption",
                "Youtube動画のクリップをまとめたページです。Youtube動画をiframeで表示しています。"
            }
            VideoPageMenu{
                on_click_add_button: open_edit_movie_clip,
                on_change_sort_select: move |sort_type: SortType|{
                    sort_type_state.set(sort_type);
                },
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
