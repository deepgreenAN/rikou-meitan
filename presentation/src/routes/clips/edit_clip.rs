use crate::components::{EditModal, InputType, MovieCard, RequiredString, ValidationInput};
use domain::movie_clip::{MovieClip, MovieUrl, Second};

use chrono::Local;
use dioxus::prelude::*;
use wasm_bindgen::UnwrapThrowExt;

#[derive(Clone, Default)]
struct MovieClipForm {
    title: Option<String>,
    url: Option<MovieUrl>,
    start: Option<Second>,
    end: Option<Second>,
}

impl TryFrom<MovieClipForm> for MovieClip {
    type Error = String;
    fn try_from(value: MovieClipForm) -> Result<Self, Self::Error> {
        let (start, end): (Second, Second) = (
            value.start.ok_or("開始時間が無効です".to_string())?,
            value.end.ok_or("終了時間が無効です".to_string())?,
        );

        let create_date = Local::now().naive_utc().date();

        Ok(MovieClip::new_with_domains(
            value.title.ok_or("タイトルが無効です")?,
            value.url.ok_or("Urlが無効です．".to_string())?,
            (start..end)
                .try_into()
                .map_err(|_| "再生範囲が無効です".to_string())?,
            create_date.try_into().unwrap_throw(),
        ))
    }
}

// -------------------------------------------------------------------------------------------------
// EditMovieClipコンポーネント

#[derive(Props)]
pub struct EditMovieClipProps<'a> {
    // 編集のベースとなるクリッブ．Someである場合に編集モードとなる
    base_movie_clip: Option<MovieClip>,
    // 送信時の処理
    on_submit: EventHandler<'a, MovieClip>,
    // キャンセル時の処理
    on_cancel: EventHandler<'a, ()>,
    // 削除時の処理
    on_remove: Option<EventHandler<'a, MovieClip>>,
}

pub fn EditMovieClip<'a>(cx: Scope<'a, EditMovieClipProps<'a>>) -> Element {
    let movie_clip_form = use_ref(cx, || {
        if let Some(base_movie_clip) = cx.props.base_movie_clip.as_ref() {
            MovieClipForm {
                title: Some(base_movie_clip.title().to_string()),
                start: Some(base_movie_clip.range().start()),
                end: Some(base_movie_clip.range().end()),
                url: Some(base_movie_clip.url().clone()),
            }
        } else {
            MovieClipForm::default()
        }
    });

    let input_caption = match cx.props.base_movie_clip.is_some() {
        false => "新しいクリップを追加",
        true => "クリップを編集",
    };

    // フォーム入力部分
    let input_element = rsx! {
        ValidationInput{
            class: "edit-clip-input-title",
            on_input: move |title: Option<RequiredString>|{
                movie_clip_form.with_mut(|form|{form.title = title.map(|title|{title.to_string()})
            })},
            error_message: "※無効なタイトルです",
            label_component: cx.render(rsx!{
                div { class: "label-container",
                    div { class:"label-main", "クリップのタイトル"}
                }
            }),
            required: true,
            input_type: InputType::TextArea,
            initial_value: cx.props.base_movie_clip.as_ref().map(|clip|{clip.title().to_string().try_into().expect("Required Sanity Check")})
        }
        ValidationInput{
            class: "edit-clip-input-url",
            on_input: move |url: Option<MovieUrl>|{
                movie_clip_form.with_mut(|form|{form.url = url})
            },
            error_message: "※無効なurlです",
            label_component: cx.render(rsx!{
                div { class: "label-container",
                    div { class:"label-main", "クリップの動画のurl"}
                    div { class:"label-detail", "youtubeのみ可能です"}
                }
            }),
            required: true,
            input_type: InputType::InputUrl,
            initial_value: cx.props.base_movie_clip.as_ref().map(|clip|{clip.url().clone()})
        }
        ValidationInput{
            class: "edit-clip-input-start",
            on_input: move |start: Option<Second>|{
                movie_clip_form.with_mut(|form|{form.start = start})
            },
            error_message: "※無効な開始時間です",
            label_component: cx.render(rsx!{
                div { class: "label-container",
                    div { class:"label-main", "クリップの開始時間"}
                    div { class:"label-detail", "秒数で指定してください．"}
                }
            }),
            required: true,
            input_type: InputType::InputNum,
            initial_value: cx.props.base_movie_clip.as_ref().map(|clip|{clip.range().start()})
        }
        ValidationInput{
            class: "edit-clip-input-end",
            on_input: move |end: Option<Second>|{
                movie_clip_form.with_mut(|form|{form.end = end})
            },
            error_message: "※無効な終了時間です",
            label_component: cx.render(rsx!{
                div { class: "label-container",
                    div { class:"label-main", "クリップの終了時間"}
                    div { class:"label-detail", "秒数で指定してください．"}
                }
            }),
            required: true,
            input_type: InputType::InputNum,
            initial_value: cx.props.base_movie_clip.as_ref().map(|clip|{clip.range().end()})
        }
    };

    // プレビュー部分
    let preview_element = rsx! {
        match TryInto::<MovieClip>::try_into(movie_clip_form.with(|form|{form.clone()})) {
            Ok(movie_clip) => rsx!{
                div { class: "edit-clip-preview-player-container",
                    MovieCard{
                        range: movie_clip.range().clone(),
                        title: movie_clip.title(),
                        date: movie_clip.create_date(),
                        id: "movie-clip-preview-player",
                        movie_url: movie_clip.url().clone(),
                    }
                }

                div { class: "edit-preview-bottom",
                    button {
                        onclick: move |_|{
                            if let Some(base_movie_clip) = cx.props.base_movie_clip.as_ref() {
                                let mut base_movie_clip = base_movie_clip.clone();
                                base_movie_clip.assign(movie_clip.clone());
                                cx.props.on_submit.call(base_movie_clip);
                            } else {
                                cx.props.on_submit.call(movie_clip.clone());
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
            let base_movie_clip = cx.props.base_movie_clip.as_ref().expect("Set base movie clip");
            rsx!{
                EditModal{
                    caption: input_caption.to_string(),
                    on_cancel: move |_| {cx.props.on_cancel.call(())},
                    on_remove: move |_| {on_remove.call(base_movie_clip.clone())},
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
