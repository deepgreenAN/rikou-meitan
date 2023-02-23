mod add_episode;
mod range_episodes;

use crate::components::AddButton;
use crate::utils::use_overlay;
use add_episode::AddEpisode;
use range_episodes::RangeEpisodes;

use domain::Date;

use dioxus::prelude::*;

pub fn EpisodesPage(cx: Scope) -> Element {
    let is_add_episode_show = use_state(cx, || false);
    let overlay_state = use_overlay(cx, 2);

    let open_add_episode = move |_| {
        is_add_episode_show.set(true);
        overlay_state.activate().expect("Cannot Overlay activate");
    };

    let close_add_episode = move |_| {
        is_add_episode_show.set(false);
        overlay_state.deactivate();
    };

    let titles = cx.use_hook(|| vec!["2018", "2019", "2020", "2021", "2022", "2023"]);

    let ranges = cx.use_hook(|| {
        vec![
            (
                Date::from_ymd(2018, 1, 1).expect("Sanity Check"),
                Date::from_ymd(2019, 1, 1).expect("Sanity Check"),
            ),
            (
                Date::from_ymd(2019, 1, 1).expect("Sanity Check"),
                Date::from_ymd(2020, 1, 1).expect("Sanity Check"),
            ),
            (
                Date::from_ymd(2020, 1, 1).expect("Sanity Check"),
                Date::from_ymd(2021, 1, 1).expect("Sanity Check"),
            ),
            (
                Date::from_ymd(2021, 1, 1).expect("Sanity Check"),
                Date::from_ymd(2022, 1, 1).expect("Sanity Check"),
            ),
            (
                Date::from_ymd(2022, 1, 1).expect("Sanity Check"),
                Date::from_ymd(2023, 1, 1).expect("Sanity Check"),
            ),
            (
                Date::from_ymd(2023, 1, 1).expect("Sanity Check"),
                Date::from_ymd(2024, 1, 1).expect("Sanity Check"),
            ),
        ]
    });

    let initial_is_opens = cx.use_hook(|| vec![false, false, false, false, false, true]);

    cx.render(rsx! {
        div {id: "episodes-container",
            titles.iter().zip(ranges.iter()).zip(initial_is_opens.iter()).map(|((title, range), initial_is_open)|{
                rsx! {
                    RangeEpisodes{
                        key:"{title}",
                        title: "{title}",
                        start: range.0,
                        end: range.1,
                        initial_is_open: *initial_is_open,
                    }
                }
            })
            div {id: "episodes-add-button", AddButton{onclick: open_add_episode}}
            is_add_episode_show.get().then(||{
                rsx! {
                    AddEpisode{
                        onsubmit: move |_|{close_add_episode(());},
                        oncancel: close_add_episode
                    }
                }
            })
        }
    })
}
