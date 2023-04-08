mod range_episodes;

use range_episodes::RangeEpisodes;

use domain::Date;

use dioxus::prelude::*;

#[derive(Props, PartialEq)]
pub struct EpisodesPageProps {
    #[props(default = false)]
    admin: bool
}


pub fn EpisodesPage(cx: Scope<EpisodesPageProps>) -> Element {
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
            div { id: "episodes-title-container",
                h2 {id: "episodes-title", 
                    match cx.props.admin {
                        true => "エピソード(管理者用モード)",
                        false => "エピソード"
                    }
                }
            } 
            div {
                id: "episodes-caption",
                "エピソードを年ごとにまとめたページです。Youtube動画をiframeで表示しています。"
            }
            titles.iter().zip(ranges.iter()).zip(initial_is_opens.iter()).map(|((title, range), initial_is_open)|{
                rsx! {
                    RangeEpisodes{
                        key:"{title}",
                        title: "{title}",
                        start: range.0,
                        end: range.1,
                        initial_is_open: *initial_is_open,
                        admin: cx.props.admin
                    }
                }
            })
        }
    })
}
