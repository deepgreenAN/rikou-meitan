use crate::components::AccordionEpisodes;
use domain::{episode::Episode, Date};

use dioxus::{core::to_owned, prelude::*};
use fake::{Fake, Faker};
use gloo_timers::future::TimeoutFuture;
use std::rc::Rc;

#[derive(Props, PartialEq)]
pub struct RangeEpisodesProps {
    /// アコーディオンパネルに渡すタイトル
    #[props(into)]
    title: String,
    /// エピソードデータの日時の下限
    start: Date,
    /// エピソードデータの日時の上限
    end: Date,
    /// 初期値としてアコーディオンパネルを開いておくかどうか
    initial_is_open: bool,
}

pub fn RangeEpisodes(cx: Scope<RangeEpisodesProps>) -> Element {
    let episodes_ref = use_ref(&cx, || Option::<Vec<Episode>>::None);

    use_effect(&cx, (), {
        to_owned![episodes_ref];
        let initial_is_open = Rc::new(cx.props.initial_is_open);

        |_| async move {
            if *initial_is_open {
                TimeoutFuture::new(1000).await;
                episodes_ref.set(Some(
                    (0..10).map(|_| Faker.fake::<Episode>()).collect::<Vec<_>>(),
                ));
            }
        }
    });

    let onopen = move |_| {
        cx.spawn({
            to_owned![episodes_ref];
            async move {
                TimeoutFuture::new(1000).await;
                episodes_ref.set(Some(
                    (0..10).map(|_| Faker.fake::<Episode>()).collect::<Vec<_>>(),
                ));
            }
        });
    };

    cx.render(rsx! {
        if cx.props.initial_is_open {
            rsx! {
                AccordionEpisodes{
                    title: cx.props.title.clone(),
                    episodes: episodes_ref.clone(),
                    initial_is_open: true
                }
            }
        } else {
            rsx! {
                AccordionEpisodes{
                    title: cx.props.title.clone(),
                    episodes: episodes_ref.clone(),
                    initial_is_open: false,
                    onopen: onopen
                }
            }
        }
    })
}
