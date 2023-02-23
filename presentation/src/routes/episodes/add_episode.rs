use crate::components::{InputType, ValidationInput};
use domain::{
    episode::{Episode, EpisodeContent},
    Date,
};

use dioxus::prelude::*;

#[derive(Clone, Default)]
struct EpisodeForm {
    date: Option<Date>,
    content: Option<EpisodeContent>,
}

impl TryFrom<EpisodeForm> for Episode {
    type Error = String;
    fn try_from(value: EpisodeForm) -> Result<Self, Self::Error> {
        Ok(Episode::new_with_domains(
            value.date.ok_or("エピソードの日時が無効です".to_string())?,
            value
                .content
                .ok_or("エピソード内容が無効です．".to_string())?,
        ))
    }
}

// -------------------------------------------------------------------------------------------------
// AddEpisode コンポーネント

#[derive(Props)]
pub struct AddEpisodeProps<'a> {
    onsubmit: EventHandler<'a, Episode>,
    oncancel: EventHandler<'a, ()>,
}

pub fn AddEpisode<'a>(cx: Scope<'a, AddEpisodeProps<'a>>) -> Element {
    let is_previewed = use_state(cx, || false);
    let episode_form = use_ref(cx, EpisodeForm::default);

    cx.render(rsx! {
        div { class: "add-episode-container", onmousedown: move |_|{cx.props.oncancel.call(())},
            div { class: "add-episode-ui-container", onmousedown: move |e| {e.stop_propagation();},
                div { class: "add-episode-input-container",
                    div { class: "add-episode-input-caption", "新しいエピソードを追加"}
                    ValidationInput{
                        class:"add-episode-input-date",
                        onchange: move |value: Option<Date>|{episode_form.with_mut(|form|{form.date = value})},
                        error_message: "※有効なDateではありません",
                        label_component: cx.render(rsx!{
                            div { class: "label-container",
                                div { class:"label-main", "エピソードの年月日"}
                                div { class:"label-detail", "アバウトで大丈夫です。後で編集できます。"}
                            }
                        }),
                        required: true,
                        show_error_message: true,
                        input_type:InputType::InputDate
                    }
                    ValidationInput{
                        class:"add-episode-input-content",
                        onchange: move |value: Option<EpisodeContent>|{episode_form.with_mut(|form|{form.content = value})},
                        error_message: "※無効なhtmlが含まれています",
                        label_component: cx.render(rsx!{
                            div { class: "label-container",
                                div { class:"label-main", "エピソード内容"}
                                div { class:"label-detail", "リンク・リスト・太字・斜体などのhtmlも使えます"}
                            }
                        }),
                        required: true,
                        show_error_message: true,
                        input_type:InputType::TextArea
                    }
                    div { class: "add-episode-input-bottom",
                        button { onclick:move |_|{is_previewed.set(true)}, "プレビューを表示"}
                        button { onclick: move |_|{cx.props.oncancel.call(())}, "キャンセル"}
                    }

                }
                is_previewed.get().then(||{
                    rsx!{
                        div { class: "add-episode-preview-container",
                            div { class: "add-episode-preview-caption", "プレビュー"}
                            match TryInto::<Episode>::try_into(episode_form.with(|form|{form.clone()})) {
                                Ok(episode) => {
                                    let content = episode.content().to_string();
                                    rsx! {
                                        ul {
                                            li{
                                                div { class: "preview-content", dangerous_inner_html: "{content}"}
                                            }
                                        }
                                        div { class: "add-episode-preview-bottom", 
                                            button { onclick: move |_|{
                                                let episode: Episode = episode_form
                                                    .with(|form|{form.clone()})
                                                    .try_into()
                                                    .expect("Sanity Check for EpisodeForm to Episode");
                                                log::info!("Add Episode: {episode:?}");
                                                cx.props.onsubmit.call(episode);
                                            }
                                            ,"送信"}
                                        }
                                    }
                                },
                                Err(error_message) => {
                                    let message = format!("プレビューを表示できません: {error_message}");
                                    rsx! {
                                        div { class: "failed-preview-content", "{message}"}
                                        div { class: "add-episode-preview-bottom", button { disabled: "true", "送信"}}
                                    }
                                }
                            }
                        }
                    }
                })
            }
        }
    })
}
