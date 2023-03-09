mod edit_video;

use crate::components::{IntersectionBottom, MovieCard, MovieContainer, Quiz, VideoPageMenu};
use crate::utils::use_overlay;
use domain::video::Video;
use edit_video::EditVideo;

use dioxus::prelude::*;
use fake::{Fake, Faker};
use gloo_intersection::IntersectionObserverHandler;
use std::rc::Rc;
use strum_macros::{Display, EnumIter, EnumString};

enum EditVideoOpen<T> {
    Modify(Video<T>),
    Add,
    Close,
}

#[derive(Display, EnumIter, EnumString, Debug, PartialEq, Eq, Clone, Default)]
enum SortType {
    #[default]
    #[strum(serialize = "投稿日")]
    CreateDate,
    #[strum(serialize = "Like")]
    Like,
}

#[derive(Props, PartialEq)]
pub struct VideosPageProps<T> {
    /// 管理者かどうか
    #[props(default = false)]
    admin: bool,
    /// 型パラメーター用の引数
    _video_type: T,
}

pub fn VideosPage<T>(cx: Scope<VideosPageProps<T>>) -> Element
where
    T: Default + Clone + crate::utils::Caption + 'static,
{
    let videos_ref = use_ref(cx, || Option::<Vec<Video<T>>>::None);

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
    use_effect(cx, (), {
        to_owned![videos_ref];
        |_| async move {
            let mut videos = (0..20)
                .map(|_| Faker.fake::<Video<T>>())
                .collect::<Vec<_>>();

            videos.sort_by_key(|video| video.date());
            videos_ref.set(Some(videos));
        }
    });

    // 底が交差するときのオブザーバー
    let intersection_handler = cx.use_hook(|| {
        let handler = IntersectionObserverHandler::new({
            to_owned![videos_ref];
            move |entries, _| {
                let target_entry = entries.into_iter().next().expect("Observe sanity check");
                if target_entry.is_intersecting() {
                    let mut new_videos = (0..20)
                        .map(|_| Faker.fake::<Video<T>>())
                        .collect::<Vec<_>>();

                    new_videos.sort_by_key(|video| video.date());
                    videos_ref.with_mut(|videos| {
                        if let Some(videos) = videos.as_mut() {
                            videos.append(&mut new_videos);
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
        videos_ref.with_mut(|videos| {
            if let Some(videos) = videos.as_mut() {
                videos.push(new_video);
                videos.sort_by_key(|video| video.date());
            }
        });
    };

    // 編集の時の処理
    let modify_video = move |modified_video: Video<T>| {
        close_edit_video(());
        videos_ref.with_mut(|videos| {
            if let Some(videos) = videos.as_mut() {
                let video_for_modify = videos
                    .iter_mut()
                    .find(|video| video.id() == modified_video.id())
                    .expect("Modify for not exists video");
                *video_for_modify = modified_video;
            }
        });
    };

    // 削除の時の処理
    let remove_video = move |video_for_remove: Video<T>| {
        close_edit_video(());
        videos_ref.with_mut(|videos| {
            if let Some(videos) = videos.as_mut() {
                videos.retain(|video| video.id() != video_for_remove.id());
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
            VideoPageMenu{
                _enum_type: SortType::default()
                on_click_add_button: open_add_video,
                on_change_sort_select: move |e: FormEvent|{log::info!("{}", e.value)},
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
