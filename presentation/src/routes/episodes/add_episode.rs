use domain::episode::Episode;

use dioxus::{core::to_owned, prelude::*};

#[derive(Props)]
pub struct AddEpisodeProps<'a> {
    onsubmit: EventHandler<'a, Episode>,
}

pub fn AddEpisode<'a>(cx: Scope<'a, AddEpisodeProps<'a>>) -> Element {
    cx.render(rsx! {
        div { class: "add-episode-container",
            "エビソードを追加"
        }
    })
}
