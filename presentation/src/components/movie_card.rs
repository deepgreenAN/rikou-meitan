use crate::components::{Player, TooltipMenuButton, TooltipMenuItem};
use crate::include_str_from_root;
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
    /// 投稿者
    #[props(into)]
    author: Option<String>,
    /// Likeボタンを押したときの処理
    on_like: Option<EventHandler<'a>>,
    /// is_likedの初期値
    #[props(default = false)]
    is_liked: bool
}

pub fn MovieCard<'a>(cx: Scope<'a, MovieCardProps<'a>>) -> Element {
    let is_liked = use_state(cx, ||{false}); // 最初はfalseと仮定

    let like_heart_svg_str = include_str_from_root!("images/release/like-heart.svg");

    // ライクされたときのクラス名
    let liked_class = match *is_liked.get() || cx.props.is_liked {
        true => " liked",
        false => ""
    };

    // ライクが押されたときの処理
    let on_like = move |_| {
        if !is_liked.get() && !cx.props.is_liked {
            if let Some(on_like) = cx.props.on_like.as_ref() {
                on_like.call(());
            }
        }

        is_liked.set(true);
    };

    cx.render(rsx! {
        div {class: "movie-card-container",
            div { class: "movie-card-player",
                Player{id: &cx.props.id, video_id:cx.props.movie_url.video_id(), range:cx.props.range.clone()}
            }
            div { class: "movie-card-caption",
                div { class: "movie-card-left",
                    div { class: "movie-card-title", "{cx.props.title}"}
                    div { class: "movie-card-left-bottom",
                        cx.props.date.map(|date|{
                            let (year, month, day) = date.to_ymd();
                            rsx!{
                                div { format!("{year}/{month}/{day}")}
                            } 
                        })
                        cx.props.author.as_ref().map(|author|{
                            rsx!{
                                div {"{author}"}
                            }
                        })
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
                    div { class: format_args!("movie-card-like-heart{liked_class}"), 
                        onclick: on_like,
                        dangerous_inner_html: "{like_heart_svg_str}"
                    }
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

