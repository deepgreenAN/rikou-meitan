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
    /// 編集を可能にするかどうか
    #[props(default = true)]
    editable: bool,
    /// アコーディオンを開いたときの処理
    onopen: Option<EventHandler<'a>>
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
                                    if let Some(onopen) = cx.props.onopen.as_ref() {
                                        onopen.call(());
                                    }
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
                                        rsx! {
                                            li {key: "{i}",
                                                div { class: "episode-item-container",
                                                    div {
                                                        span { class: "episode-date", format!("{year}/{month}/{day}")}
                                                        span { class: "episode-content", "{content}"}
                                                    }
                                                    cx.props.editable.then(||
                                                        rsx!{
                                                            div { class: "episode-item-right",
                                                                button {class: "episode-modify-button", "編集"}
                                                            }
                                                        }
                                                    ) 
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
