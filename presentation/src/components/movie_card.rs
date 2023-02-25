use crate::components::Player;
use domain::{movie_clip::Second, Date};

use dioxus::prelude::*;

#[derive(Props)]
pub struct MovieCardProps {
    start: Option,
}
