use crate::components::{Player, TooltipMenuButton, TooltipMenuItem};
use domain::{
    movie_clip::{MovieUrl, SecondRange},
    Date,
};

use dioxus::prelude::*;

// -------------------------------------------------------------------------------------------------
// MovieCard

#[derive(Props)]
pub struct MovieCardProps<'a> {
    /// プレーヤーの再生範囲
    // #[props(!optional)]
    range: Option<SecondRange>,
    /// 動画のタイトル
    #[props(into)]
    title: String,
    /// 日時
    date: Option<Date>,
    /// playerのid
    #[props(into)]
    id: String,
    /// youtube url
    movie_url: MovieUrl,
    /// 編集ボタンを押したときの処理
    on_modify: Option<EventHandler<'a>>,
}

pub fn MovieCard<'a>(cx: Scope<'a, MovieCardProps<'a>>) -> Element {
    let like_heart_svg_str = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/images/release/like-heart.svg"
    ));

    cx.render(rsx! {
        div {class: "movie-card-container",
            div { class: "movie-card-player",
                Player{id: &cx.props.id, video_id:cx.props.movie_url.video_id(), range:cx.props.range.clone()}
            }
            div { class: "movie-card-caption",
                div { class: "movie-card-left",
                    div { class: "movie-card-title", "{cx.props.title}"}
                    if let Some(date) = cx.props.date {
                        let (year, month, day) = date.to_ymd();
                        rsx!{
                            div { class: "movie-card-date", format!("{year}/{month}/{day}")}
                        }
                    }
                }
                div { class: "movie-card-right",
                    if let Some(on_modify) = cx.props.on_modify.as_ref() {
                        rsx!{
                            div { class: "movie-card-dot-menu",
                                TooltipMenuButton{
                                    TooltipMenuItem{    
                                        div { onclick:move|_|{on_modify.call(())},"編集"}
                                    }
                                }
                            }
                        }
                    } else {
                        rsx!{div { class: "movie-card-dot-menu"}}
                    }
                    div { class: "movie-card-like-heart", dangerous_inner_html: "{like_heart_svg_str}"}
                }
            }
        }
    })
}

// -------------------------------------------------------------------------------------------------
// MovieContainer

#[derive(Props)]
pub struct MovieContainerProps<'a> {
    children: Element<'a>,
}

pub fn MovieContainer<'a>(cx: Scope<'a, MovieContainerProps<'a>>) -> Element {
    cx.render(rsx! {
        div {class: "movies-container",
            &cx.props.children
        }
    })
}
