use crate::PLAYING_PLAYER_ID;
use crate::ACTIVE_PLAYER_IDS;
use crate::ACTIVE_PLAYER_NUMBER;
use crate::include_str_from_root;
use domain::movie_clip::SecondRange;

const ORIGIN: &str = "https://rikou-meitan.shuttleapp.rs";

use dioxus::prelude::*;
use fermi::{use_read, use_set, use_atom_state};
use gloo_timers::future::TimeoutFuture;
use gloo_intersection::IntersectionObserverHandler;
use plyr::{
    events::{PlyrStandardEventListener, PlyrStandardEventType},
    Plyr,
    options::*,
};
use wasm_bindgen::UnwrapThrowExt;

use std::cell::Cell;
use std::rc::Rc;

#[derive(Props, PartialEq)]
pub struct PlayerProps {
    #[props(into)]
    id: String,
    #[props(into)]
    video_id: String,
    #[props(!optional)]
    range: Option<SecondRange>,
}

pub fn Player(cx: Scope<PlayerProps>) -> Element {
    let movie_cover_svg_str = include_str_from_root!("images/release/movie_cover.svg");

    let thumbnail_url = use_state(cx, || Option::<String>::None);
    let is_active = use_state(cx, || false);
    let player_state = use_state(cx, || Option::<Plyr>::None);
    let intersecting_handler = cx.use_hook(||{Rc::new(Cell::new(Option::<IntersectionObserverHandler>::None))});

    let (playing_player_id , setter_playing_player_id)= (
        use_read(cx, PLAYING_PLAYER_ID), 
        use_set(cx, PLAYING_PLAYER_ID)
    );

    let (active_player_ids, active_player_ids_state) = (
        use_read(cx, ACTIVE_PLAYER_IDS),
        use_atom_state(cx, ACTIVE_PLAYER_IDS)
    );

    let onplay_event_listener = cx.use_hook(||{Rc::new(Cell::new(Option::<PlyrStandardEventListener>::None))});

    let player_container_id = format!("{}-player-container", &cx.props.id);
    let src_url = format!("https://www.youtube.com/embed/{}?origin={ORIGIN}&iv_load_policy=3&modestbranding=1&playsinline=1&showinfo=0&rel=0&enablejsapi=1", &cx.props.video_id);

    let first_clicked_time = cx.use_hook(||{Rc::new(Cell::new(Option::<chrono::NaiveDateTime>::None))});


    // 初期化
    use_effect(cx, (), {
        to_owned![thumbnail_url, intersecting_handler, player_container_id, intersecting_handler];
        let video_id = cx.props.video_id.clone();
        let player_container_id = player_container_id.clone();
        |_| async move {

            let target_element = gloo_utils::document().get_element_by_id(&player_container_id).unwrap_throw();

            let handler = IntersectionObserverHandler::new({
                to_owned![intersecting_handler];
                move |entries, _|{
                    let first_entry = entries.first().unwrap_throw();
                    if first_entry.is_intersecting() {
                        wasm_bindgen_futures::spawn_local({
                            to_owned![thumbnail_url, video_id, intersecting_handler];
                            async move {
                                TimeoutFuture::new(500).await;
                                // サムネイルのurlをセット
                                thumbnail_url.set(Some(
                                    format!("https://img.youtube.com/vi/{video_id}/sddefault.jpg"),
                                ));

                                // もう不要であるため交差オブザーバーを削除
                                intersecting_handler.set(None);
                            }
                        });
                    }
                }
            }).unwrap_throw();

            // オブザーブ
            handler.observe(&target_element);

            intersecting_handler.set(Some(handler));
        }
    });

    // is_activeに依存させたplayerの初期化
    use_effect(cx, is_active, {
        to_owned![
            player_state, 
            onplay_event_listener, 
            setter_playing_player_id,
            active_player_ids_state,
            intersecting_handler
        ];
        let mut selector = "#".to_string();
        let id = cx.props.id.clone();
        selector.push_str(&id);

        let player_container_id = player_container_id.clone();

        let youtube_options = YoutubeOptions {
            start: cx.props.range.as_ref().map(|range|{range.start().to_u32()}),
            end: cx.props.range.as_ref().map(|range|{range.end().to_u32()}),
            autoplay: Some(1),
            ..Default::default()
        };

        |is_active| async move {
            if *is_active.current() {
                // プレーヤー関連

                let player_options = PlyrOptions::builder()
                .youtube(youtube_options)
                .autoplay(true)
                .build();

                let player = Plyr::new_with_options(&selector, &player_options);
                let onplay_handler = PlyrStandardEventListener::new(
                    &player,
                    PlyrStandardEventType::playing, 
                    {
                        to_owned![id];
                        move |_|{
                            // log::debug!("player:{} is playing", &id);
                            setter_playing_player_id(Some(id.clone()));
                        }
                    });

                player_state.set(Some(player));
                onplay_event_listener.set(Some(onplay_handler));

                active_player_ids_state.with_mut({
                    to_owned![id];
                    move |ids|{
                        ids.push_back(id);
                        // log::debug!("active player list pushed: {:?}", ids);

                        if ids.len() > ACTIVE_PLAYER_NUMBER {
                            let _popped = ids.pop_front();
                            // log::debug!("popped player: {:?}", popped);
                        }
                    }
                });

                // 交差オブザーバー関連
                let target_element = gloo_utils::document().get_element_by_id(&player_container_id).unwrap_throw();

                let handler = IntersectionObserverHandler::new({
                    move |entries, _|{
                        let first_entry = entries.first().unwrap_throw();
                        if !first_entry.is_intersecting() {
                            // ターゲットがビューポートから出たとき
                            if let Some(player) = player_state.current().as_ref(){
                                player.pause();
                            }
                        }
                    }
                }).unwrap_throw();
    
                // オブザーブ
                handler.observe(&target_element);
    
                intersecting_handler.set(Some(handler));
            }
        }
    });

    // アクティプリストに入っていない場合の処理
    use_effect(cx, active_player_ids,{
        to_owned![is_active, player_state, onplay_event_listener, intersecting_handler];
        let player_id = cx.props.id.clone(); 
        |active_player_ids| 
            async move{
                if *is_active.current() && !active_player_ids.iter().any(|id|{id==&player_id}) && player_state.current().is_some(){
                    // プレーヤーの削除
                    is_active.set(false);
                    player_state.set(None);
                    onplay_event_listener.set(None);
                    intersecting_handler.set(None);
                }
        }
    });


    // // 現在再生中のプレーヤーと異なる場合
    use_effect(cx, playing_player_id, {
        let id = cx.props.id.clone();
        to_owned![player_state];

        |playing_player_id| async move {
            // log::debug!("playing_player_id changed!");
            if let Some(playing_player_id) = playing_player_id {
                if playing_player_id != id {
                    if let Some(player) = &*player_state.current() {
                        player.pause();
                        // log::debug!("player: {} is paused.", id);
                    }
                }
            }
        }
    });


    cx.render(rsx! {
        div { class:"player-container", id:"{player_container_id}",
            if let Some(url) = thumbnail_url.get() {
                rsx! {
                    if *is_active.get() {
                        rsx! {
                            div { class: "my-iframe-player",
                                div{ class: "plyr__video-embed", id:"{cx.props.id}",
                                    iframe {
                                        src: "{src_url}",
                                        allowfullscreen: "true",
                                        allow: "autoplay"
                                    }                           
                                }
                                div { class: "player-wrapper",
                                    prevent_default: "touchstart", // 動画のピンチズームを防ぐのに必要 <- 防げなかった
                                    onclick: move |_|{
                                        let now_time = chrono::Local::now().naive_utc();

                                        if let Some(player) = player_state.get() {
                                            match first_clicked_time.get() {
                                                // 既にクリックされている場合
                                                Some(clicked_time) => {
                                                    if (now_time - clicked_time).num_milliseconds() < 800 {
                                                        // クリックした間隔が短い場合
                                                        player.fullscreen().enter();
                                                    } else {
                                                        // クリックした間隔が長い場合
                                                        player.toggle_play();
                                                    }

                                                    first_clicked_time.set(None); // 完全に初期化
                                                },
                                                // クリックされていない・未初期化の場合
                                                None => {
                                                    player.toggle_play();
                                                    first_clicked_time.set(Some(now_time));
    
                                                    wasm_bindgen_futures::spawn_local({
                                                        to_owned![first_clicked_time];
                                                        async move {
                                                            TimeoutFuture::new(800).await;
                                                            first_clicked_time.set(None);  // 間隔が長いと初期化
                                                        }
                                                    });
                                                }
                                            }
                                        }
                                    },
                                    // ondblclick: move |_|{
                                    //     if let Some(player) = player_state.get() {
                                    //         player.fullscreen().enter();
                                    //     }
                                    // },
                                }
                            }
                        }
                    } else {
                        rsx! {
                            div {  class: "my-player-thumbnail", onclick: move |_| {is_active.set(true)},
                                img { src: "{url}"}
                            }
                        }
                    }
                }
            } else {
                rsx! { div { class: "player-cover",dangerous_inner_html: "{movie_cover_svg_str}"}}
            }
        }
    })
}
