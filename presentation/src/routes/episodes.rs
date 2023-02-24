mod range_episodes;

use range_episodes::RangeEpisodes;

use domain::Date;

use dioxus::prelude::*;

pub fn EpisodesPage(cx: Scope) -> Element {
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
        }
    })
}
