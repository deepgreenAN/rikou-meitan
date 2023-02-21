mod add_episode;
mod range_episodes;

use crate::components::AddButton;
use domain::Date;
use range_episodes::RangeEpisodes;

use dioxus::prelude::*;

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
