#![allow(clippy::derive_partial_eq_without_eq)]
#![allow(non_snake_case)]

use dioxus::core::to_owned;
use dioxus::prelude::*;
use plyr::{
    events::{PlyrStandardEventType, PlyrYoutubeEventListener},
    Plyr,
};

#[inline_props]
fn PlyrFrame(cx: Scope, id: String) -> Element {
    let player_state: &UseState<Option<Plyr>> = use_state(&cx, || None);
    let event_listeners_state: &UseState<Option<Vec<PlyrYoutubeEventListener>>> =
        use_state(&cx, || None);

    use_effect(&cx, (), |_| {
        to_owned![player_state, event_listeners_state];
        let mut selector = "#".to_string();
        selector.push_str(id);
        async move {
            let player = Plyr::new(&selector);
            let event_listeners = vec![PlyrYoutubeEventListener::new(
                &player,
                PlyrStandardEventType::ready.into(),
                move |e| {
                    let player = e.detail().plyr();
                    player.play(); // できない
                },
            )
            .unwrap()];

            player_state.set(Some(player));
            event_listeners_state.set(Some(event_listeners));
        }
    });

    cx.render(rsx! {
        div { class: "plyr__video-embed", id:"{id}", width: "400px",
            iframe {
                src: "https://www.youtube.com/embed/bTqVqk7FSmY?origin=https://plyr.io&amp;iv_load_policy=3&amp;modestbranding=1&amp;playsinline=1&amp;showinfo=0&amp;rel=0&amp;enablejsapi=1",
                allowfullscreen: "true",
                allow: "autoplay"
            }
        },
        button {
            onclick: move |_| {player_state.get().as_ref().unwrap().fullscreen().enter();},
            "fullscreen"
        },
        button {
            onclick: move |_| {player_state.get().as_ref().unwrap().play();},
            "play"
        },
        button {
            onclick: move |_| {player_state.get().as_ref().unwrap().pause();},
            "pause"
        }
    })
}

fn App(cx: Scope) -> Element {
    cx.render(rsx! {
        link { rel: "stylesheet", href: "https://cdn.plyr.io/3.7.3/plyr.css"},
        PlyrFrame{id: "player".to_string()},
    })
}

fn main() {
    dioxus::web::launch(App);
}
