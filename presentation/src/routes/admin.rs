mod admin_login;
mod json_loader;

use crate::utils::use_overlay;
use admin_login::AdminLogin;
use domain::{
    episode::Episode,
    movie_clip::MovieClip,
    video::{Kirinuki, Original, Video},
};
use frontend::{commands, usecases};
use json_loader::{JsonLoader, JsonLoaderProps};

use dioxus::prelude::*;
use dioxus_router::Link;
use std::rc::Rc;

pub fn AdminPage(cx: Scope) -> Element {
    let is_admin_login_open = use_state(cx, || true);
    let overlay_state = use_overlay(cx, 2);

    use_effect(cx, (), {
        to_owned![overlay_state];
        |_| async move {
            overlay_state.activate().expect("Cannot overlay activate");
        }
    });

    // 管理者ログインモーダルを閉じた処理
    let close_admin_login = move |_| {
        is_admin_login_open.set(false);
        overlay_state.deactivate();
    };

    // エピソードに関するJsonLoaderの引数
    let episode_json_loader_props = JsonLoaderProps {
        title: "エピソードデータ".to_string(),
        id: "episode-json-loader".to_string(),
        on_upload: Rc::new(move |episodes: Vec<Episode>| {
            wasm_bindgen_futures::spawn_local(async move {
                for episode in episodes.into_iter() {
                    let cmd = commands::episode_commands::SaveEpisodeCommand::new(&episode);
                    let res = usecases::episode_usecase::save_episode(cmd).await;
                    if let Err(e) = res {
                        log::error!("{} Cannot save episode:{:?}", e, episode);
                    }
                }
            });
        }),
        make_json_source: Rc::new(|| async move {
            let cmd = commands::episode_commands::AllEpisodesCommand;
            usecases::episode_usecase::all_episodes(cmd)
                .await
                .expect("All episodes fetch failed.")
        }),
    };

    // クリップに関するJsonLoaderの引数
    let clips_json_loader_props = JsonLoaderProps {
        title: "クリップデータ".to_string(),
        id: "clips-json-loader".to_string(),
        on_upload: Rc::new(move |clips: Vec<MovieClip>| {
            wasm_bindgen_futures::spawn_local(async move {
                for clip in clips.into_iter() {
                    let cmd = commands::movie_clip_commands::SaveMovieClipCommand::new(&clip);
                    let res = usecases::movie_clip_usecase::save_movie_clip(cmd).await;
                    if let Err(e) = res {
                        log::error!("{} Cannot save movie_clip: {:?}", e, clip);
                    }
                }
            });
        }),
        make_json_source: Rc::new(|| async move {
            let cmd = commands::movie_clip_commands::AllMovieClipsCommand;
            usecases::movie_clip_usecase::all_movie_clips(cmd)
                .await
                .expect("All movie_clips fetch failed.")
        }),
    };

    // コラボ配信に関するJsonLoaderの引数
    let originals_json_loader_props = JsonLoaderProps {
        title: "コラボ配信データ".to_string(),
        id: "originals-json-loader".to_string(),
        on_upload: Rc::new(move |originals: Vec<Video<Original>>| {
            wasm_bindgen_futures::spawn_local(async move {
                for original in originals.into_iter() {
                    let cmd = commands::video_commands::SaveVideoCommand::new(&original);
                    let res = usecases::video_usecase::save_video(cmd).await;
                    if let Err(e) = res {
                        log::error!("{} Cannot save original: {:?}", e, original);
                    }
                }
            });
        }),
        make_json_source: Rc::new(|| async move {
            let cmd = commands::video_commands::AllVideosCommand;
            usecases::video_usecase::all_videos::<Original>(cmd)
                .await
                .expect("All originals fetch failed.")
        }),
    };

    // 切り抜きに関するJsonLoaderの引数
    let kirinukis_json_loader_props = JsonLoaderProps {
        title: "切り抜きデータ".to_string(),
        id: "kirinukis-json-loader".to_string(),
        on_upload: Rc::new(move |kirinukis: Vec<Video<Kirinuki>>| {
            wasm_bindgen_futures::spawn_local(async move {
                for kirinuki in kirinukis.into_iter() {
                    let cmd = commands::video_commands::SaveVideoCommand::new(&kirinuki);
                    let res = usecases::video_usecase::save_video(cmd).await;
                    if let Err(e) = res {
                        log::error!("{} Cannot save kirinuki: {:?}", e, kirinuki);
                    }
                }
            });
        }),
        make_json_source: Rc::new(|| async move {
            let cmd = commands::video_commands::AllVideosCommand;
            usecases::video_usecase::all_videos::<Kirinuki>(cmd)
                .await
                .expect("All kirinukis fetch failed.")
        }),
    };

    cx.render(rsx! {
        is_admin_login_open.get().then(||{
            rsx!{
                AdminLogin{on_cancel: close_admin_login}
            }
        })

        div {id: "admin-container",
            div {id: "admin-menu-caption", "メニュー"}
            Link{ to: "/admin/episodes", "エピソード"}
            Link{ to: "/admin/clips", "クリップ"}
            Link{ to: "/admin/originals", "コラボ配信"}
            Link{ to: "/admin/kirinukis", "切り抜き"}

            div {id: "admin-json-loader-container",
                JsonLoader{..episode_json_loader_props},
                JsonLoader{..clips_json_loader_props},
                JsonLoader{..originals_json_loader_props},
                JsonLoader{..kirinukis_json_loader_props}
            }
        }
    })
}
