mod edit_video;

use crate::components::{IntersectionBottom, MovieCard, MovieContainer, Quiz, VideoPageMenu};
use crate::utils::use_overlay;
use domain::video::{Video, VideoType};
use edit_video::EditVideo;
use frontend::{commands::video_commands, usecases::video_usecase};

use dioxus::prelude::*;
use gloo_intersection::IntersectionObserverHandler;
use std::cell::Cell;
use std::rc::Rc;
use strum_macros::{Display, EnumIter, EnumString};

enum EditVideoOpen<T: VideoType> {
    Modify(Video<T>),
    Add,
    Close,
}

#[derive(Display, EnumIter, EnumString, Debug, PartialEq, Eq, Clone, Copy, Default)]
enum SortType {
    #[default]
    #[strum(serialize = "投稿日")]
    Date,
    #[strum(serialize = "Like")]
    Like,
}

#[derive(Props, PartialEq)]
pub struct VideosPageProps<T> {
    /// 管理者かどうか
    #[props(default = false)]
    pub admin: bool,
    /// 型パラメーター用の引数
    #[props(default)]
    _video_type: std::marker::PhantomData<T>,
}

pub fn VideosPage<T>(cx: Scope<VideosPageProps<T>>) -> Element
where
    T: VideoType + crate::utils::Caption + 'static,
{
    let videos_ref = use_ref(cx, || Option::<Vec<Video<T>>>::None);
    let is_load_continue = cx.use_hook(|| Rc::new(Cell::new(true)));
    let sort_type_state = use_state(cx, SortType::default);

    // EditVideo関連
    let edit_video_open = use_state(cx, || EditVideoOpen::Close);
    let overlay_state = use_overlay(cx, 2);

    // 新規追加モーダルを開く処理
    let open_add_video = move |_| {
        edit_video_open.set(EditVideoOpen::Add);
        overlay_state.activate().expect("Cannot overlay activate");
    };

    // モーダルを閉じる処理
    let close_edit_video = move |_| {
        edit_video_open.set(EditVideoOpen::Close);
        overlay_state.deactivate();
    };

    // 状態の初期化
    use_effect(cx, sort_type_state, {
        to_owned![videos_ref, is_load_continue];
        |sort_type| async move {
            // ロードを許可
            is_load_continue.set(true);

            // データをフェッチ
            let res = match *sort_type.current() {
                SortType::Date => {
                    let cmd = video_commands::OrderByDateVideosCommand::new(20);
                    video_usecase::order_by_date_videos(cmd).await
                }
                SortType::Like => {
                    let cmd = video_commands::OrderByLikeVideosCommand::new(20);
                    video_usecase::order_by_like_videos(cmd).await
                }
            };

            match res {
                Ok(new_videos) => {
                    // データが一つも取得できない場合は今後のロードを拒否
                    if new_videos.is_empty() {
                        is_load_continue.set(false);
                    }

                    videos_ref.set(Some(new_videos));
                }
                Err(e) => {
                    log::error!("{}", e);
                }
            }
        }
    });

    // 底が交差するときのオブザーバー
    let intersection_handler = cx.use_hook(|| {
        let handler = IntersectionObserverHandler::new({
            to_owned![videos_ref, is_load_continue, sort_type_state];
            move |entries, _| {
                let target_entry = entries.into_iter().next().expect("Observe sanity check");
                if target_entry.is_intersecting() {
                    to_owned![videos_ref, is_load_continue, sort_type_state];
                    wasm_bindgen_futures::spawn_local(async move {
                        // 最後の値を取得
                        let last_video = videos_ref.with(|videos_opt| {
                            if let Some(videos) = videos_opt.as_ref() {
                                videos.last().cloned()
                            } else {
                                None
                            }
                        });

                        if let Some(last_video) = last_video {
                            // データをフェッチ
                            let res = match *sort_type_state.current() {
                                SortType::Date => {
                                    let cmd = video_commands::OrderByDateLaterVideosCommand::new(
                                        &last_video,
                                        20,
                                    );
                                    video_usecase::order_by_date_later_videos(cmd).await
                                }
                                SortType::Like => {
                                    let cmd = video_commands::OrderByLikeLaterVideosCommand::new(
                                        &last_video,
                                        20,
                                    );
                                    video_usecase::order_by_like_later_videos(cmd).await
                                }
                            };

                            match res {
                                Ok(new_videos) => {
                                    // データが一つも取得できない場合以降のロードを拒否
                                    if new_videos.is_empty() {
                                        is_load_continue.set(false);
                                    }
                                    videos_ref.with_mut(|videos_opt| {
                                        if let Some(videos) = videos_opt.as_mut() {
                                            // 重複を防ぎながら挿入
                                            for new_video in new_videos.into_iter() {
                                                let is_not_contain = videos
                                                    .iter()
                                                    .all(|video| video.id() != new_video.id());

                                                if is_not_contain {
                                                    videos.push(new_video);
                                                }
                                            }
                                        }
                                    });
                                }
                                Err(e) => {
                                    log::error!("{}", e);
                                }
                            }
                        }
                    });
                }
            }
        })
        .expect("Intersection Handler Error");
        Rc::new(handler)
    });

    // 新規追加の時の処理
    let add_video = move |new_video: Video<T>| {
        close_edit_video(());

        // new_videoを末尾に挿入
        {
            let new_video = new_video.clone();
            videos_ref.with_mut(|videos| {
                if let Some(videos) = videos.as_mut() {
                    videos.push(new_video);
                }
            });
        }

        // API
        cx.spawn({
            to_owned![videos_ref];
            async move {
                let res = {
                    let cmd = video_commands::SaveVideoCommand::new(&new_video);
                    video_usecase::save_video(cmd).await
                };

                if let Err(e) = res {
                    log::error!("{} Removed video: {:?}", e, new_video);

                    // new_videoを削除
                    videos_ref.with_mut(|videos_opt| {
                        if let Some(videos) = videos_opt.as_mut() {
                            videos.retain(|video| video.id() != new_video.id());
                        }
                    });
                }
            }
        })
    };

    // 編集の時の処理
    let modify_video = move |modified_video: Video<T>| {
        close_edit_video(());
        let mut old_video = Option::<Video<T>>::None;

        // modified_videoに更新
        {
            let modified_video = modified_video.clone();
            videos_ref.with_mut(|videos| {
                if let Some(videos) = videos.as_mut() {
                    let found_video = videos
                        .iter_mut()
                        .find(|video| video.id() == modified_video.id())
                        .expect("Modify for not exists video");
                    // 古いデータを新しいデータに更新。
                    old_video = Some(std::mem::replace(found_video, modified_video));
                }
            });
        }

        // API
        cx.spawn({
            to_owned![videos_ref];
            async move {
                let res = {
                    let cmd = video_commands::EditVideoCommand::new(&modified_video);
                    video_usecase::edit_video(cmd).await
                };

                // レスポンスがエラーの場合
                if let Err(e) = res {
                    log::error!("{} Roll backed video: {:?}", e, modified_video);

                    if let Some(old_video) = old_video {
                        videos_ref.with_mut(|videos_opt| {
                            if let Some(videos) = videos_opt.as_mut() {
                                let found_video = videos
                                    .iter_mut()
                                    .find(|video| video.id() == old_video.id())
                                    .expect("Cannot found video.");

                                // 更新したエラーをロールバック
                                *found_video = old_video;
                            }
                        });
                    }
                }
            }
        });
    };

    // 削除の時の処理
    let remove_video = move |video_for_remove: Video<T>| {
        close_edit_video(());

        {
            videos_ref.with_mut(|videos| {
                if let Some(videos) = videos.as_mut() {
                    videos.retain(|video| video.id() != video_for_remove.id());
                }
            });
        }

        // API
        cx.spawn({
            to_owned![videos_ref, sort_type_state];

            async move {
                let res = {
                    let cmd = video_commands::RemoveVideoCommand::new(video_for_remove.id());
                    video_usecase::remove_video::<T>(cmd).await
                };

                // レスポンスがエラーの場合
                if let Err(e) = res {
                    log::error!("{} Re-pushed video: {:?}", e, video_for_remove);

                    // 削除した要素を再び挿入してソート
                    videos_ref.with_mut(|videos_opt| {
                        if let Some(videos) = videos_opt.as_mut() {
                            videos.push(video_for_remove);

                            match *sort_type_state.current() {
                                SortType::Date => {
                                    // dateを降順・idを昇順にソート
                                    videos.sort_by(|x, y| {
                                        y.date().cmp(&x.date()).then_with(|| x.id().cmp(&y.id()))
                                    });
                                }
                                SortType::Like => {
                                    // likeを降順・idを昇順にソート
                                    videos.sort_by(|x, y| {
                                        y.like().cmp(&x.like()).then_with(|| x.id().cmp(&y.id()))
                                    });
                                }
                            }
                        }
                    })
                }
            }
        });
    };

    cx.render(rsx! {
        div { class: "videos-container",
            div { class: "videos-title-container",
                h2 { class: "videos-title",
                    match cx.props.admin {
                        true => format!("{}(管理者モード)", T::caption()),
                        false => T::caption()
                    }
                }
            }
            div { class: "videos-caption",
                format!("Youtubeの{}動画をまとめたページです。Youtube動画をiframeで表示しています。", T::caption())
            }
            VideoPageMenu{
                on_click_add_button: open_add_video,
                on_change_sort_select: move |sort_type: SortType|{sort_type_state.set(sort_type)},
            }
            match edit_video_open.get() {
                EditVideoOpen::Add => rsx!{
                    EditVideo{
                        on_submit: add_video,
                        on_cancel: close_edit_video
                        base_video: Option::<Video<T>>::None
                    }
                },
                EditVideoOpen::Modify(modified_video) => rsx!{
                    Quiz{
                        on_cancel: close_edit_video,
                        admin: cx.props.admin,
                        match cx.props.admin {
                            true => rsx!{ // 管理者の時
                                EditVideo {
                                    on_submit: modify_video,
                                    on_cancel: close_edit_video,
                                    base_video: Some(modified_video.clone()),
                                    on_remove: remove_video
                                }
                            },
                            false => rsx!{ // 管理者でない時
                                EditVideo {
                                    on_submit: modify_video,
                                    on_cancel: close_edit_video,
                                    base_video: Some(modified_video.clone()),
                                }
                            }
                        }
                    }
                },
                EditVideoOpen::Close => rsx!{Option::<VNode>::None}
            }


            MovieContainer{
                videos_ref.read().as_ref().map(|videos|{
                    rsx!{
                        videos.iter().map(|video|{
                            let video = video.clone();
                            let id = video.id();
                            rsx!{
                                MovieCard{
                                    key: "{id}",
                                    date: video.date(),
                                    title: video.title(),
                                    movie_url: video.url().clone(),
                                    author: video.author(),
                                    id: format!("video-{id}"),
                                    on_modify: move |_|{
                                        edit_video_open.set(EditVideoOpen::Modify(video.clone()));
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
