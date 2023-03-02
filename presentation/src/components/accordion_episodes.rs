use crate::components::Spinner;
use domain::episode::Episode;

use dioxus::prelude::*;

#[derive(Props)]
pub struct AccordionEpisodesProps<'a> {
    /// タイトル
    #[props(into)]
    title: String,
    /// エピソードのリスト
    episodes: UseRef<Option<Vec<Episode>>>,
    /// コンポーネント作成時にパネルを開くかどうか．
    initial_is_open: bool,
    /// アコーディオン機能を無効にするかどうか
    #[props(default = false)]
    fixed: bool,
    /// アコーディオンを開いたときの処理
    on_open: Option<EventHandler<'a>>,
    /// アコーディオンを閉じたときの処理
    on_close: Option<EventHandler<'a>>,
    /// 編集ボタンが押されたときの処理
    on_modify_click: Option<EventHandler<'a, Episode>>,
}

pub fn AccordionEpisodes<'a>(cx: Scope<'a, AccordionEpisodesProps<'a>>) -> Element {
    let is_accordion_open = use_state(cx, || cx.props.initial_is_open);

    let accordion_button_str = match is_accordion_open.get() {
        true => include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/images/release/minus.svg"
        )),
        false => include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/images/release/plus.svg"
        )),
    };

    
    let episodes_is_some = { // Refの再帰を防ぐため
        cx.props.episodes.read().is_some()
    };
    

    cx.render(rsx! {div { class: "accordion-episodes-container",
        div { class: "accordion-bar-container",
            (!cx.props.fixed).then(||{
                rsx! {
                    div { class: "plus-svg",
                        dangerous_inner_html: "{accordion_button_str}",
                        onclick: move |_|{
                            is_accordion_open.modify(|flag|{
                                if !flag {
                                    if let Some(on_open) = cx.props.on_open.as_ref() {
                                        on_open.call(());
                                    }
                                } else if let Some(on_close) = cx.props.on_close.as_ref() {
                                    on_close.call(());
                                }
                                !flag
                            })
                        }
                    }
                }
            })
            div { class: "accordion-title", "{cx.props.title}"}
        }
        is_accordion_open.get().then(||{
            if episodes_is_some { // データが与えられ多場合
                rsx! {
                    div { class: "accordion-episode-list",
                        ul {
                            cx.props.episodes.read().as_ref().map(|episodes|{
                                rsx!{
                                    episodes.iter().enumerate().map(|(i,episode)|{
                                        let (year, month, day) = episode.date().to_ymd();
                                        let content = episode.content();
                                        let episode = episode.clone();
                                        rsx! {
                                            li {key: "{i}",
                                                div { class: "episode-item-container",
                                                    div {
                                                        span { class: "episode-date", format!("{year}/{month}/{day}")}
                                                        span { class: "episode-content", dangerous_inner_html: "{content}"}
                                                    }
                                                    if let Some(on_modify_click) = cx.props.on_modify_click.as_ref() {
                                                        rsx!{
                                                            div { class: "episode-item-right",
                                                                button {class: "episode-modify-button",
                                                                    onclick: move |_|{
                                                                        on_modify_click.call(episode.clone());
                                                                    }, 
                                                                    "編集"
                                                                }
                                                            }
                                                        }
                                                    } 
                                                }
                                            }
                                        }
                                    })
                                }
                                
                            })
                        }
                    }
                }
            } else { // まだデータが与えられていない場合
                rsx! {
                    div { class: "accordion-spinner-container",
                        div { class: "accordion-spinner", Spinner{}}
                    }
                    
                }
            }

        })

    }})
}
