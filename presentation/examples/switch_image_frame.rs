#![allow(clippy::derive_partial_eq_without_eq)]
#![allow(non_snake_case)]

use dioxus::{core::to_owned, prelude::*};
use gloo_timers::future::TimeoutFuture;
use plyr::Plyr;

#[inline_props]
fn SwitchImageFrame(cx: Scope, id: String) -> Element {
    let movie_cover_svg = include_str!("./assets/movie_cover.svg");
    let thumbnail_url: &UseState<Option<String>> = use_state(&cx, || None);
    let is_clicked = use_state(&cx, || false);
    let player_state: &UseState<Option<Plyr>> = use_state(&cx, || None);

    use_effect(&cx, (), {
        to_owned![thumbnail_url];
        |_| async move {
            TimeoutFuture::new(1000).await;
            thumbnail_url.set(Some(
                "https://img.youtube.com/vi/bTqVqk7FSmY/sddefault.jpg".to_string(),
            ))
        }
    });

    use_effect(&cx, is_clicked, {
        to_owned![player_state];
        let mut selector = "#".to_string();
        selector.push_str(id);

        |is_clicked| async move {
            if *is_clicked.get() {
                let player = Plyr::new(&selector);
                player_state.set(Some(player));
            }
        }
    });

    cx.render(rsx! {
        if let Some(url) = thumbnail_url.get() {
            rsx! {
                if *is_clicked.get() {
                    rsx! {
                        div{ class: "plyr__video-embed", id:"{id}",
                            iframe {
                                src: "https://www.youtube.com/embed/bTqVqk7FSmY?origin=https://plyr.io&amp;iv_load_policy=3&amp;modestbranding=1&amp;playsinline=1&amp;showinfo=0&amp;rel=0&amp;enablejsapi=1",
                                allowfullscreen: "true",
                                allow: "autoplay"
                            }                           
                        }
                    }
                } else {
                    rsx! {
                        div { onclick: move |_| {is_clicked.set(true)},
                            img { src: "{url}",width: "400px", height:"225px",object_fit:"cover"}
                        }
                    }
                }
            }
        } else {
            rsx! { div { dangerous_inner_html: "{movie_cover_svg}"} }
        }
    })
}

fn App(cx: Scope) -> Element {
    let style_str = include_str!("./assets/switch_image_frame.css");
    cx.render(rsx! {
        link { rel: "stylesheet", href: "https://cdn.plyr.io/3.7.3/plyr.css"}
        style {"{style_str}"}
        div { id:"movie-container",
            (0..6).map(|i|{
                let id = format!("frame-{i}");
                rsx! {
                    div { width:"400px", height: "225px", key: "{i}",
                        SwitchImageFrame{id: id}
                    }
                }
            })
        }
    })
}

fn main() {
    dioxus::web::launch(App);
}
