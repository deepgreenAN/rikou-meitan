mod edit_video;

use crate::components::{IntersectionBottom, MovieCard, MovieContainer, Quiz, VideoPageMenu};
use crate::utils::use_overlay;
use domain::video::{Original, Video};

use dioxus::prelude::*;
use fake::{Fake, Faker};
use gloo_intersection::IntersectionObserverHandler;
use std::rc::Rc;
use strum_macros::{Display, EnumIter, EnumString};

enum EditVideoOpen {
    Modify(Video<Original>),
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
pub struct VideosPageProps {
    #[props(default = false)]
    admin: bool,
}

pub fn VideosPage(cx: Scope<VideosPageProps>) -> Element {
    let videos_ref = use_ref(cx, || Option::<Vec<Video<Original>>>::None);
    // 状態の初期化
    use_effect(cx, (), {
        to_owned![videos_ref];
        |_| async move {
            let mut videos = (0..20)
                .map(|_| Faker.fake::<Video<Original>>())
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
                        .map(|_| Faker.fake::<Video<Original>>())
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

    cx.render(rsx! {
        div { class: "videos-container",
            div { class: "videos-title-container",
                h2 { class: "videos-title",
                    match cx.props.admin {
                        true => "コラボ配信(管理者モード)",
                        false => "コラボ配信"
                    }
                }
            }
            VideoPageMenu{
                _enum_type: SortType::default()
                on_click_add_button: move |_|{},
                on_change_sort_select: move |e: FormEvent|{log::info!("{}", e.value)},
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
                                    id: format!("video-{id}"),
                                    on_modify: move |_|{}
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
