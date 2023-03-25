use crate::components::{EditModal, InputType, MovieCard, RequiredString, ValidationInput};
use domain::{
    video::{Video, VideoType},
    Date, MovieUrl,
};

use dioxus::prelude::*;

#[derive(Clone, Default)]
struct VideoForm {
    title: Option<String>,
    url: Option<MovieUrl>,
    date: Option<Date>,
    author: Option<String>,
}

impl<T: VideoType> TryFrom<VideoForm> for Video<T> {
    type Error = String;
    fn try_from(value: VideoForm) -> Result<Self, Self::Error> {
        Ok(Video::<T>::new_with_domains(
            value.title.ok_or("タイトルが無効です".to_string())?,
            value.url.ok_or("Urlが無効です".to_string())?,
            value.date.ok_or("投稿日が無効です".to_string())?,
            value.author.ok_or("投稿者が無効です".to_string())?,
        ))
    }
}

// -------------------------------------------------------------------------------------------------
// EditVideoコンポーネント

#[derive(Props)]
pub struct EditVideoProps<'a, T: VideoType> {
    /// 編集のベースとなるVideo．Someである場合に編集モードとなる．
    #[props(!optional)]
    base_video: Option<Video<T>>,
    /// 送信時の処理
    on_submit: EventHandler<'a, Video<T>>,
    /// キャンセル時の処理
    on_cancel: EventHandler<'a, ()>,
    /// 削除時の処理
    on_remove: Option<EventHandler<'a, Video<T>>>,
}

pub fn EditVideo<'a, T>(cx: Scope<'a, EditVideoProps<'a, T>>) -> Element
where
    T: VideoType + crate::utils::Caption,
{
    let video_form = use_ref(cx, || {
        if let Some(base_video) = cx.props.base_video.as_ref() {
            VideoForm {
                title: Some(base_video.title().to_string()),
                url: Some(base_video.url().clone()),
                date: Some(base_video.date()),
                author: Some(base_video.author().to_string()),
            }
        } else {
            VideoForm::default()
        }
    });

    let caption_name = &T::caption();

    let input_caption = match cx.props.base_video.is_some() {
        false => format!("新しい{caption_name}を追加"),
        true => format!("{caption_name}を編集"),
    };

    // フォーム入力部分
    let input_element = rsx! {
        ValidationInput{
            class: "edit-video-title",
            on_input: move |title: Option<RequiredString>|{
                video_form.with_mut(|video_form|{video_form.title = title.map(|title|{title.to_string()})})
            },
            error_message: "※無効なタイトルです",
            label_component: cx.render(rsx!{
                div { class: "label-container",
                    div { class:"label-main", "{caption_name}のタイトル"}
                }
            }),
            required: true,
            input_type: InputType::TextArea,
            initial_value: cx.props.base_video.as_ref().map(|video|{video.title().to_string().try_into().expect("Required sanity check")})
        }
        ValidationInput{
            class: "edit-video-url",
            on_input: move |url: Option<MovieUrl>|{
                video_form.with_mut(|video_form|{video_form.url = url})
            },
            error_message: "※無効なURLです",
            label_component: cx.render(rsx!{
                div { class: "label-container",
                    div { class:"label-main", "{caption_name}のUrl"}
                }
            }),
            required: true,
            input_type: InputType::InputUrl,
            initial_value: cx.props.base_video.as_ref().map(|video|{video.url().clone()})
        }
        ValidationInput{
            class: "edit-video-date",
            on_input: move |date: Option<Date>|{
                video_form.with_mut(|video_form|{video_form.date = date})
            },
            error_message: "※無効な投稿日です",
            label_component: cx.render(rsx!{
                div { class: "label-container",
                    div { class:"label-main", "{caption_name}の投稿日"}
                }
            }),
            required: true,
            input_type: InputType::InputDate,
            initial_value: cx.props.base_video.as_ref().map(|video|{video.date()})
        }
        ValidationInput{
            class: "edit-video-author",
            on_input: move |author: Option<RequiredString>|{
                video_form.with_mut(|video_form|{video_form.author = author.map(|author|{author.to_string()})})
            },
            error_message: "※無効な投稿者です",
            label_component: cx.render(rsx!{
                div { class: "label-container",
                    div { class:"label-main", "{caption_name}の投稿者"}
                }
            }),
            required: true,
            input_type: InputType::InputText,
            initial_value: cx.props.base_video.as_ref().map(|video|{video.author().to_string().try_into().expect("Required sanity check")})
        }
    };

    // プレビュー部分
    let preview_element = rsx! {
        match TryInto::<Video<T>>::try_into(video_form.with(|form|{form.clone()})) {
            Ok(video) => rsx!{
                div{ class: "edit-video-preview-player-container",
                    MovieCard{
                        date: video.date(),
                        title: video.title(),
                        movie_url: video.url().clone(),
                        author: video.author(),
                        id: "video-preview-player",
                    }
                }
                div { class: "edit-preview-bottom",
                    button {
                        onclick: move |_|{
                            if let Some(base_video) = cx.props.base_video.as_ref() {
                                let mut base_video = base_video.clone();
                                base_video.assign(video.clone());
                                cx.props.on_submit.call(base_video);
                            } else {
                                cx.props.on_submit.call(video.clone());
                            }
                        },
                        "送信"
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
            let base_video = cx.props.base_video.as_ref().expect("Set base video");
            rsx!{
                EditModal{
                    caption: input_caption.to_string(),
                    on_cancel: move |_| {cx.props.on_cancel.call(())},
                    on_remove: move |_| {on_remove.call(base_video.clone())},
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
