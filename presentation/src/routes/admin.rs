mod admin_login;
mod json_loader;

use crate::utils::use_overlay;
use admin_login::AdminLogin;
use domain::{
    episode::Episode,
    movie_clip::MovieClip,
    video::{Kirinuki, Original, Video},
};
use json_loader::{JsonLoader, JsonLoaderProps};

use dioxus::prelude::*;
use dioxus_router::Link;
use fake::{Fake, Faker};
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

    // エピソードに関するJsonLoader
    let episode_json_loader_props = JsonLoaderProps {
        title: "エピソードデータ".to_string(),
        id: "episode-json-loader".to_string(),
        on_upload: Rc::new(move |episodes: Vec<Episode>| log::info!("{:?}", episodes)),
        make_json_source: Rc::new(move || {
            (0..10).map(|_| Faker.fake::<Episode>()).collect::<Vec<_>>()
        }),
    };

    let episode_json_loader =
        cx.component(JsonLoader, episode_json_loader_props, "episode_json_loader");

    // クリップに関するJsonLoader
    let clips_json_loader_props = JsonLoaderProps {
        title: "クリップデータ".to_string(),
        id: "clips-json-loader".to_string(),
        on_upload: Rc::new(move |clips: Vec<MovieClip>| log::info!("{:?}", clips)),
        make_json_source: Rc::new(move || {
            (0..10)
                .map(|_| Faker.fake::<MovieClip>())
                .collect::<Vec<_>>()
        }),
    };

    let clips_json_loader = cx.component(JsonLoader, clips_json_loader_props, "clips_json_loader");

    // コラボ配信に関するJsonLoader
    let originals_json_loader_props = JsonLoaderProps {
        title: "コラボ配信データ".to_string(),
        id: "originals-json-loader".to_string(),
        on_upload: Rc::new(move |originals: Vec<Video<Original>>| log::info!("{:?}", originals)),
        make_json_source: Rc::new(move || {
            (0..10)
                .map(|_| Faker.fake::<Video<Original>>())
                .collect::<Vec<_>>()
        }),
    };

    let originals_json_loader = cx.component(
        JsonLoader,
        originals_json_loader_props,
        "originals_json_loader",
    );

    // 切り抜きに関するJsonLoader
    let kirinukis_json_loader_props = JsonLoaderProps {
        title: "切り抜きデータ".to_string(),
        id: "kirinukis-json-loader".to_string(),
        on_upload: Rc::new(move |kirinukis: Vec<Video<Kirinuki>>| log::info!("{:?}", kirinukis)),
        make_json_source: Rc::new(move || {
            (0..10)
                .map(|_| Faker.fake::<Video<Kirinuki>>())
                .collect::<Vec<_>>()
        }),
    };

    let kirinukis_json_loader = cx.component(
        JsonLoader,
        kirinukis_json_loader_props,
        "kirinukis_json_loader",
    );

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
                episode_json_loader,
                clips_json_loader,
                originals_json_loader,
                kirinukis_json_loader
            }
        }
    })
}
