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
pub struct EditEpisodeProps<'a> {
    /// 編集のベースとなるエピソード
    base_episode: Option<Episode>,
    /// 送信時の処理
    on_submit: EventHandler<'a, Episode>,
    /// キャンセル時の処理
    on_cancel: EventHandler<'a, ()>,
    // 削除時の処理
    on_remove: Option<EventHandler<'a, Episode>>,
}

pub fn EditEpisode<'a>(cx: Scope<'a, EditEpisodeProps<'a>>) -> Element {
    let is_previewed = use_state(cx, || false);
    let episode_form = use_ref(cx, || {
        if let Some(base_episode) = cx.props.base_episode.as_ref() {
            EpisodeForm {
                date: Some(base_episode.date()),
                content: Some(base_episode.content().clone()),
            }
        } else {
            EpisodeForm::default()
        }
    });

    let input_caption = match cx.props.base_episode.is_some() {
        true => "エピソードを編集",
        false => "新しいエピソードを追加",
    };

    cx.render(rsx! {
        div { class: "edit-episode-container", 
            onclick: move |_|{cx.props.on_cancel.call(())}, //なぜかonmousedownのstop_propagationが効かない
            div { class: "edit-episode-ui-container", 
                onclick: move |e| {e.stop_propagation();},
                div { class: "edit-episode-input-container",
                    div { class: "edit-episode-input-caption", "{input_caption}"}
                    ValidationInput{
                        class:"edit-episode-input-date",
                        on_input: move |value: Option<Date>|{episode_form.with_mut(|form|{form.date = value})},
                        error_message: "※有効なDateではありません",
                        label_component: cx.render(rsx!{
                            div { class: "label-container",
                                div { class:"label-main", "エピソードの年月日"}
                                div { class:"label-detail", "アバウトで大丈夫です。後で編集できます。"}
                            }
                        }),
                        required: true,
                        input_type:InputType::InputDate,
                        initial_value: cx.props.base_episode.as_ref().map(|episode|{episode.date()}),
                    }
                    ValidationInput{
                        class:"edit-episode-input-content",
                        on_input: move |value: Option<EpisodeContent>|{episode_form.with_mut(|form|{form.content = value})},
                        error_message: "※無効なhtmlが含まれています",
                        label_component: cx.render(rsx!{
                            div { class: "label-container",
                                div { class:"label-main", "エピソード内容"}
                                div { class:"label-detail", "リンク・リスト・太字・斜体などのhtmlも使えます"}
                            }
                        }),
                        required: true,
                        input_type:InputType::TextArea,
                        initial_value: cx.props.base_episode.as_ref().map(|episode|{episode.content().clone()}),
                    }
                    div { class: "edit-episode-input-bottom",
                        button { onclick:move |_|{is_previewed.set(true)}, "プレビューを表示"}
                        button { onclick: move |_|{cx.props.on_cancel.call(())}, "キャンセル"}
                        // 削除ボタン
                        cx.props.base_episode.as_ref().map(|episode|{
                            rsx!{
                                cx.props.on_remove.as_ref().map(|on_remove|{
                                    rsx!{
                                        button {onclick: move |_|{on_remove.call(episode.clone())}, "削除"}
                                    }
                                })
                            }
                        })
                    }

                }
                is_previewed.get().then(||{
                    rsx!{
                        div { class: "edit-episode-preview-container",
                            div { class: "edit-episode-preview-caption", "プレビュー"}
                            match TryInto::<Episode>::try_into(episode_form.with(|form|{form.clone()})) {
                                Ok(episode) => {
                                    let content = episode.content().to_string();
                                    let (year, month, day) = episode.date().to_ymd();
                                    rsx! {
                                        ul {
                                            li{
                                                div { class:"preview-item-container",
                                                    span { class: "preview-date", format!("{year}/{month}/{day}")}
                                                    span { class: "preview-content", dangerous_inner_html: "{content}"}
                                                }
                                            }
                                        }
                                        div { class: "edit-episode-preview-bottom", 
                                            button { onclick: move |_|{
                                                if let Some(base_episode) = cx.props.base_episode.as_ref() {
                                                    let mut base_episode = base_episode.clone();
                                                    base_episode.assign(episode.clone());
                                                    cx.props.on_submit.call(base_episode);
                                                } else {
                                                    cx.props.on_submit.call(episode.clone());
                                                }
                                            }
                                            ,"送信"}
                                        }
                                    }
                                },
                                Err(error_message) => {
                                    let message = format!("プレビューを表示できません: {error_message}");
                                    rsx! {
                                        div { class: "failed-preview-content", "{message}"}
                                        div { class: "edit-episode-preview-bottom", button { disabled: "true", "送信"}}
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
