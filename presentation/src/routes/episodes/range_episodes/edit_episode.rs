use crate::components::{EditModal, InputType, ValidationInput};
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

    // フォーム入力部分
    let input_element = rsx! {
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
                    div { class:"label-detail", "リンク・リスト・太字・斜体などのhtmlも使えます。a要素はrel=\"noopener noreferrer\"を付けてください。"}
                }
            }),
            required: true,
            input_type:InputType::TextArea,
            initial_value: cx.props.base_episode.as_ref().map(|episode|{episode.content().clone()}),
        }
    };

    // プレビュー部分
    let preview_element = rsx! {
        match TryInto::<Episode>::try_into(episode_form.with(|form|{form.clone()})) {
            Ok(episode) => {
                let content = episode.content().to_string();
                let (year, month, day) = episode.date().to_ymd();
                rsx! {
                    ul {
                        li{
                            span { class: "preview-date", format!("{year}/{month}/{day}")}
                            span { class: "preview-content", dangerous_inner_html: "{content}"}

                        }
                    }
                    div { class: "edit-preview-bottom",
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
                    div { class: "failed-preview", "{message}"}
                    div { class: "edit-preview-bottom", button { disabled: "true", "送信"}}
                }
            }
        }
    };

    cx.render(rsx! {
        if let Some(on_remove) = cx.props.on_remove.as_ref() {
            let base_episode = cx.props.base_episode.as_ref().expect("Set base episode");
            rsx!{
                EditModal{
                    caption: input_caption.to_string(),
                    on_cancel: move |_| {cx.props.on_cancel.call(())},
                    on_remove: move |_| {on_remove.call(base_episode.clone())},
                    input: cx.render(input_element),
                    preview: cx.render(preview_element)
                }
            }
        } else {
            rsx!{
                EditModal{
                    caption: input_caption.to_string(),
                    on_cancel: move |_| {cx.props.on_cancel.call(())},
                    input: cx.render(input_element),
                    preview: cx.render(preview_element)
                }
            }
        }
    })
}
