use crate::components::{InputType, RequiredString, ValidationInput};
use domain::{
    movie_clip::{MovieClip, MovieUrl, Second},
    Date,
};

use dioxus::prelude::*;

#[derive(Clone, Default)]
struct MovieClipForm {
    title: Option<String>,
    url: Option<MovieUrl>,
    start: Option<Second>,
    end: Option<Second>,
}
