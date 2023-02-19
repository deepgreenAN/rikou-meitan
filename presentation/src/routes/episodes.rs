mod add_episode;

use crate::components::{AccordionEpisodes, AddButton};
use domain::{episode::Episode, Date};

use dioxus::{core::to_owned, prelude::*};
use fake::{Fake, Faker};
use gloo_timers::future::TimeoutFuture;
use std::rc::Rc;

pub fn EpisodesPage(cx: Scope) -> Element {
    cx.render(rsx! {
        div {id: "episodes-container",
            RangeEpisodes{
                title: "2018",
                start: Date::from_ymd(2018, 1, 1).expect("Sanity Check"),
                end: Date::from_ymd(2019, 1, 1).expect("Sanity Check"),
                initial_is_open: false,
            }
            RangeEpisodes{
                title: "2019",
                start: Date::from_ymd(2019, 1, 1).expect("Sanity Check"),
                end: Date::from_ymd(2020, 1, 1).expect("Sanity Check"),
                initial_is_open: false,
            }
            RangeEpisodes{
                title: "2020",
                start: Date::from_ymd(2020, 1, 1).expect("Sanity Check"),
                end: Date::from_ymd(2021, 1, 1).expect("Sanity Check"),
                initial_is_open: false,
            }
            RangeEpisodes{
                title: "2021",
                start: Date::from_ymd(2021, 1, 1).expect("Sanity Check"),
                end: Date::from_ymd(2022, 1, 1).expect("Sanity Check"),
                initial_is_open: false,
            }
            RangeEpisodes{
                title: "2022",
                start: Date::from_ymd(2022, 1, 1).expect("Sanity Check"),
                end: Date::from_ymd(2023, 1, 1).expect("Sanity Check"),
                initial_is_open: false,
            }
            RangeEpisodes{
                title: "2023",
                start: Date::from_ymd(2023, 1, 1).expect("Sanity Check"),
                end: Date::from_ymd(2024, 1, 1).expect("Sanity Check"),
                initial_is_open: true,
            }
            div {id: "episodes-add-button", AddButton{}}
        }
    })
}

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
