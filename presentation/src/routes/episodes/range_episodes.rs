mod edit_episode;

use crate::components::{AccordionEpisodes, AddButton, Quiz};
use crate::utils::use_overlay;
use domain::{episode::Episode, Date};
use edit_episode::EditEpisode;

use dioxus::prelude::*;
use fake::Fake;
use gloo_timers::future::TimeoutFuture;

enum EditEpisodeOpen {
    Modify(Episode),
    Add,
    Close,
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
    // エピソードのデータ
    let episodes_ref = use_ref(cx, || Option::<Vec<Episode>>::None);

    // AddButton関連
    let edit_episode_open = use_state(cx, || EditEpisodeOpen::Close);
    let is_add_button_show = use_state(cx, || cx.props.initial_is_open);
    let overlay_state = use_overlay(cx, 2);

    // 新規追加ボタンを押されたときの処理
    let open_add_episode = move |_| {
        edit_episode_open.set(EditEpisodeOpen::Add);
        overlay_state.activate().expect("Cannot Overlay activate");
    };

    // モーダルを閉じるときの処理
    let close_add_episode = move |_| {
        edit_episode_open.set(EditEpisodeOpen::Close);
        overlay_state.deactivate();
    };

    // 修正ボタンが押されたときの処理
    let on_modify_click = move |episode| {
        edit_episode_open.set(EditEpisodeOpen::Modify(episode));
        overlay_state.activate().expect("Cannot Overlay activate");
    };

    let initial_is_open = cx.props.initial_is_open;
    let start = cx.props.start;
    let end = cx.props.end;

    // initial_is_openがtrueだった時のepisodesの初期化
    use_effect(cx, (), {
        to_owned![episodes_ref];
        |_| async move {
            if initial_is_open {
                let is_episodes_none = { episodes_ref.read().is_none() };
                if is_episodes_none {
                    TimeoutFuture::new(1000).await;
                    let mut episodes = (0..10)
                        .map(|_| (start..end).fake::<Episode>())
                        .collect::<Vec<_>>();
                    episodes.sort_by_key(|episode| episode.date());

                    episodes_ref.set(Some(episodes));
                }
            }
        }
    });

    // アコーディオンを開いたときのコールバック関数
    let on_accordion_open = move |_| {
        cx.spawn({
            to_owned![episodes_ref, is_add_button_show];
            async move {
                let is_episodes_none = { episodes_ref.read().is_none() };
                if is_episodes_none {
                    TimeoutFuture::new(1000).await;
                    let mut episodes = (0..10)
                        .map(|_| (start..end).fake::<Episode>())
                        .collect::<Vec<_>>();
                    episodes.sort_by_key(|episode| episode.date());

                    episodes_ref.set(Some(episodes));
                }
                is_add_button_show.set(true);
            }
        });
    };

    // アコーディオンを閉じたときのコールバック関数
    let on_accordion_close = move |_| {
        is_add_button_show.set(false);
    };

    cx.render(rsx! {
        if cx.props.initial_is_open {
            rsx! {
                AccordionEpisodes{
                    title: cx.props.title.clone(),
                    episodes: episodes_ref.clone(),
                    initial_is_open: true,
                    on_open: on_accordion_open,
                    on_close: on_accordion_close,
                    on_modify_click: on_modify_click,
                }
            }
        } else {
            rsx! {
                AccordionEpisodes{
                    title: cx.props.title.clone(),
                    episodes: episodes_ref.clone(),
                    initial_is_open: false,
                    on_open: on_accordion_open,
                    on_close: on_accordion_close,
                    on_modify_click: on_modify_click
                }
            }
        }
        is_add_button_show.get().then(||{
            rsx!{
                div {id: "episodes-add-button", AddButton{onclick: open_add_episode}}
            }
        })
        match edit_episode_open.get() {
            EditEpisodeOpen::Add => {
                rsx! {
                    EditEpisode{
                        onsubmit: move |new_episode|{
                            close_add_episode(());
                            episodes_ref.with_mut(|episodes|{
                                if let Some(episodes) = episodes.as_mut() {
                                    episodes.push(new_episode);
                                    episodes.sort_by_key(|episode|{episode.date()});
                                }
                            })
                        },
                        oncancel: close_add_episode
                    }
                }
            },
            EditEpisodeOpen::Modify(episode) => {
                rsx!{
                    Quiz{
                        on_cancel: close_add_episode,
                        EditEpisode{
                            onsubmit: move |modified_episode: Episode|{
                                close_add_episode(());
                                episodes_ref.with_mut(|episodes|{
                                    if let Some(episodes) = episodes.as_mut() {
                                        episodes.iter_mut().for_each(|episode|{
                                            if episode.id() == modified_episode.id() {
                                                *episode = modified_episode.clone();
                                            }
                                        })
                                    }
                                })
                            },
                            base_episode: episode.clone(),
                            oncancel: close_add_episode
                        }
                    }
                }
            },
            EditEpisodeOpen::Close => rsx!{Option::<VNode>::None},
        }
    })
}
