mod more_button;
mod toc;

use crate::components::{AccordionEpisodes, MovieCard, MovieContainer, Player};
use domain::{
    episode::Episode,
    movie_clip::MovieClip,
    video::{Kirinuki, Original, Video},
    Date,
};
use more_button::MoreButton;
use toc::{Toc, TocContent};

use frontend::{
    commands::{episode_commands, movie_clip_commands, video_commands},
    usecases::{episode_usecase, movie_clip_usecase, video_usecase},
};

use dioxus::prelude::*;

pub fn HomePage(cx: Scope) -> Element {
    let orikou_desc_str = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/contents/orikou_desc.html"
    ));

    let episodes_ref = use_ref(cx, || Option::<Vec<Episode>>::None);
    let movie_clips_ref = use_ref(cx, || Option::<Vec<MovieClip>>::None);
    let originals_ref = use_ref(cx, || Option::<Vec<Video<Original>>>::None);
    let kirinukis_ref = use_ref(cx, || Option::<Vec<Video<Kirinuki>>>::None);

    let episode_start: Date = (2023, 1, 1).try_into().expect("Date sanity check");
    let episode_end: Date = (2024, 1, 1).try_into().expect("Date sanity check");

    // 初期化
    use_effect(cx, (), {
        to_owned![episodes_ref, movie_clips_ref, originals_ref, kirinukis_ref];

        |_| async move {
            // episodesの初期化
            {
                let cmd = episode_commands::OrderByDateRangeEpisodesCommand::new(
                    episode_start,
                    episode_end,
                );
                let res = episode_usecase::order_by_date_range_episodes(cmd).await;
                match res {
                    Ok(episodes) => episodes_ref.set(Some(episodes)),
                    Err(e) => log::error!("{}", e),
                }
            }
            // movie_clipsの初期化
            {
                let cmd = movie_clip_commands::OrderByLikeMovieClipsCommand::new(6);
                let res = movie_clip_usecase::order_by_like_movie_clips(cmd).await;
                match res {
                    Ok(movie_clips) => movie_clips_ref.set(Some(movie_clips)),
                    Err(e) => log::error!("{}", e),
                }
            }

            // originalsの初期化
            {
                let cmd = video_commands::OrderByLikeVideosCommand::new(6);
                let res = video_usecase::order_by_like_videos(cmd).await;
                match res {
                    Ok(originals) => originals_ref.set(Some(originals)),
                    Err(e) => log::error!("{}", e),
                }
            }

            // kirinukisの初期化
            {
                let cmd = video_commands::OrderByLikeVideosCommand::new(6);
                let res = video_usecase::order_by_like_videos(cmd).await;
                match res {
                    Ok(kirinukis) => kirinukis_ref.set(Some(kirinukis)),
                    Err(e) => log::error!("{}", e),
                }
            }
        }
    });

    cx.render(rsx! {
        div { id: "home-container",
            Toc{},
            div { id: "toc-contents-container",
                TocContent{
                    id: "orikou-desc",
                    title: "おりコウとは",
                    div { dangerous_inner_html: "{orikou_desc_str}"}
                    div { id: "orikou-desc-movie-container-outer",
                        div { id: "orikou-desc-movie-container-inner",
                            Player{id:"orikou-desc-movie-player", video_id:"B7OPlsdBuVc", range:None}
                        }
                    }
                }
                TocContent{
                    id: "episodes",
                    title: "エピソード",
                    caption: "おりコウについてのエピソード",
                    AccordionEpisodes{
                        title: "2023",
                        episodes: episodes_ref.clone(),
                        initial_is_open: true,
                        fixed: true,
                    }
                    MoreButton{to:"/episodes"}
                }
                TocContent{
                    id: "clips",
                    title: "クリップ",
                    caption: "おりコウの配信動画のクリップ(Youtube)",
                    MovieContainer{
                        movie_clips_ref.read().as_ref().map(|movie_clips|{
                            rsx!{
                                movie_clips.iter().map(|movie_clip|{
                                    let id = movie_clip.id();
                                    rsx!{
                                        MovieCard{
                                            key:"{id}",
                                            date: movie_clip.create_date(),
                                            range: movie_clip.range().clone(),
                                            title: movie_clip.title(),
                                            movie_url: movie_clip.url().clone(),
                                            id: format!("movie-clip-{id}"),
                                        }
                                    }
                                })
                            }
                        })
                    }
                    MoreButton{to:"/clips"}
                }
                TocContent{
                    id: "originals",
                    title: "コラボ配信",
                    caption: "おりコウのコラボ配信(Youtube)",
                    MovieContainer{
                        originals_ref.read().as_ref().map(|originals|{
                            rsx!{
                                originals.iter().map(|original|{
                                    let id = original.id();
                                    rsx!{
                                        MovieCard{
                                            key: "{id}",
                                            date: original.date(),
                                            title: original.title(),
                                            author: original.author(),
                                            movie_url: original.url().clone(),
                                            id: format!("original-{id}")
                                        }
                                    }
                                })
                            }
                        })
                    }
                    MoreButton{to:"/originals"}
                }
                TocContent{
                    id: "kirinukis",
                    title: "切り抜き",
                    caption: "おりコウの切り抜き(Youtube)",
                    MovieContainer{
                        kirinukis_ref.read().as_ref().map(|kirinukis|{
                            rsx!{
                                kirinukis.iter().map(|kirinuki|{
                                    let id = kirinuki.id();
                                    rsx!{
                                        MovieCard{
                                            key: "{id}",
                                            date: kirinuki.date(),
                                            title: kirinuki.title(),
                                            author: kirinuki.author(),
                                            movie_url: kirinuki.url().clone(),
                                            id: format!("original-{id}")
                                        }
                                    }
                                })
                            }
                        })
                    }
                    MoreButton{to:"/kirinukis"}
                }
            }
        }
    })
}
