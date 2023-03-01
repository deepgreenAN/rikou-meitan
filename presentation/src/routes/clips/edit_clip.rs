use crate::components::{InputType, RequiredString, ValidationInput};
use domain::{
    movie_clip::{MovieClip, MovieUrl, Second},
    Date,
};

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
// AddMovieClipコンポーネント

#[derive(Props)]
pub struct AddMovieClipProps<'a> {
    base_movie_clip: Option<MovieClip>,
    on_submit: EventHandler<'a, MovieClip>,
    on_cancel: EventHandler<'a, ()>,
}

pub fn AddMovieClip<'a>(cx: Scope<'a, AddMovieClipProps<'a>>) -> Element {
    let is_previewed = use_state(cx, || false);
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

    cx.render(rsx! {
        div { class:"edit-clip-container",
            onclick: move |_|{cx.props.on_cancel.call(())},
            div { class: "edit-clip-ui-container",
                onclick: move |e| {e.stop_propagation();},
                div { class: "edit-clip-input-container",
                    div { class: "edit-clip-input-caption", "新しいクリップを追加"}
                }
            }
        }
    })
}
