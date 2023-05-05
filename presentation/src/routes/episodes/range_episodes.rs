mod edit_episode;

use crate::components::{AccordionEpisodes, AddButton, Quiz};
use crate::utils::use_overlay;
use domain::{episode::Episode, Date};
use edit_episode::EditEpisode;
use frontend::{
    commands::episode_commands, usecases::episode_usecase, AppCommonError, AppFrontError,
};

use dioxus::prelude::*;

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
    /// 管理者
    #[props(default = false)]
    admin: bool,
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
            // initial_is_openかtrueの場合にデータをフェッチ
            if initial_is_open {
                let res = {
                    let cmd = episode_commands::OrderByDateRangeEpisodesCommand::new(start, end);
                    episode_usecase::order_by_date_range_episodes(cmd).await
                };

                match res {
                    Ok(episodes) => {
                        episodes_ref.set(Some(episodes));
                    }
                    Err(e) => {
                        log::error!("{}", e)
                    }
                };
            }
        }
    });

    // アコーディオンを開いたときのコールバック関数
    let on_accordion_open = move |_| {
        cx.spawn({
            to_owned![episodes_ref, is_add_button_show];

            async move {
                // episodes_refがNoneの場合にデータをフェッチ
                if episodes_ref.with(|episodes_opt| episodes_opt.is_none()) {
                    let res = {
                        let cmd =
                            episode_commands::OrderByDateRangeEpisodesCommand::new(start, end);
                        episode_usecase::order_by_date_range_episodes(cmd).await
                    };

                    match res {
                        Ok(episodes) => {
                            episodes_ref.set(Some(episodes));
                        }
                        Err(e) => {
                            log::error!("{}", e)
                        }
                    }
                }
                is_add_button_show.set(true);
            }
        });
    };

    // アコーディオンを閉じたときのコールバック関数
    let on_accordion_close = move |_| {
        is_add_button_show.set(false);
    };

    // 新しく追加するときの処理
    let add_submitted_episode = move |new_episode: Episode| {
        close_add_episode(());
        // データを新しく追加してソート
        {
            let new_episode = new_episode.clone();
            episodes_ref.with_mut(|episodes| {
                if let Some(episodes) = episodes.as_mut() {
                    log::info!("Add episode: {:?}", new_episode);
                    episodes.push(new_episode);
                    episodes.sort_by_key(|episode| episode.date());
                }
            });
        }

        // API
        cx.spawn({
            to_owned![episodes_ref];
            async move {
                let res = {
                    let cmd = episode_commands::SaveEpisodeCommand::new(&new_episode);
                    episode_usecase::save_episode(cmd).await
                };

                if let Err(e) = res {
                    log::error!("{}", e);

                    // new_movie_clipを削除
                    episodes_ref.with_mut(|episodes_opt| {
                        if let Some(episodes) = episodes_opt.as_mut() {
                            log::info!("Remove episode: {:?}", new_episode);
                            episodes.retain(|episode| episode.id() != new_episode.id());
                        }
                    })
                }
            }
        })
    };

    // 編集するときの処理
    let modify_submitted_episode = move |modified_episode: Episode| {
        close_add_episode(());
        let mut old_episode = Option::<Episode>::None;

        // modified_episodeを編集する
        {
            let modified_episode = modified_episode.clone();
            episodes_ref.with_mut(|episodes| {
                if let Some(episodes) = episodes.as_mut() {
                    let found_episode = episodes
                        .iter_mut()
                        .find(|episode| episode.id() == modified_episode.id())
                        .expect("Cannot find modified episode");

                    log::info!(
                        "Episode: {:?}, modify to: {:?}",
                        found_episode,
                        modified_episode
                    );
                    old_episode = Some(std::mem::replace(found_episode, modified_episode));
                }
            });
        }

        // API
        cx.spawn({
            to_owned![episodes_ref];
            async move {
                let res = {
                    let cmd = episode_commands::EditEpisodeCommand::new(&modified_episode);
                    episode_usecase::edit_episode(cmd).await
                };

                // レスポンスがエラーの場合
                if let Err(e) = res {
                    log::error!("{}", e);

                    // 更新したデータをロールバック(NoRecordエラーの場合は削除)
                    if matches!(e, AppFrontError::CommonError(AppCommonError::NoRecordError)) {
                        // NoRecordエラーの場合に削除
                        episodes_ref.with_mut(|episodes_opt| {
                            if let Some(episodes) = episodes_opt.as_mut() {
                                log::info!("Remove episode: {:?}", modified_episode);
                                episodes.retain(|episode| episode.id() != modified_episode.id());
                            }
                        })
                    } else {
                        // その他のエラーの場合にデータをロールバック

                        if let Some(old_episode) = old_episode {
                            episodes_ref.with_mut(|episodes_opt| {
                                if let Some(episodes) = episodes_opt.as_mut() {
                                    let found_episode = episodes
                                        .iter_mut()
                                        .find(|episode| episode.id() == old_episode.id())
                                        .expect("Cannot find old episode");

                                    log::info!("Roll back episode: {:?}", old_episode);
                                    *found_episode = old_episode;
                                }
                            });
                        }
                    }
                }
            }
        })
    };

    // 削除するときの処理
    let remove_episode = move |episode_for_remove: Episode| {
        close_add_episode(());
        {
            episodes_ref.with_mut(|episodes| {
                if let Some(episodes) = episodes.as_mut() {
                    log::info!("Remove episode: {:?}", episode_for_remove);
                    episodes.retain(|episode| episode.id() != episode_for_remove.id());
                }
            })
        }

        // API
        cx.spawn({
            to_owned![episodes_ref];
            async move {
                let res = {
                    let cmd = episode_commands::RemoveEpisodeCommand::new(episode_for_remove.id());
                    episode_usecase::remove_episode(cmd).await
                };

                // レスポンスがエラーの場合
                if let Err(e) = res {
                    log::error!("{}", e);

                    // 削除したデータを再び挿入・ソート(NoRecordエラーでない場合)
                    if !matches!(e, AppFrontError::CommonError(AppCommonError::NoRecordError)) {
                        episodes_ref.with_mut(|episodes_opt| {
                            if let Some(episodes) = episodes_opt {
                                log::info!("Re-pushed episode: {:?}", episode_for_remove);
                                episodes.push(episode_for_remove);

                                episodes.sort_by(|x, y| {
                                    x.date().cmp(&y.date()).then_with(|| x.id().cmp(&y.id()))
                                });
                            }
                        });
                    }
                }
            }
        });
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
                        on_submit: add_submitted_episode,
                        on_cancel: close_add_episode
                    }
                }
            },
            EditEpisodeOpen::Modify(episode) => {
                rsx!{
                    Quiz{
                        on_cancel: close_add_episode,
                        admin: cx.props.admin,
                        match cx.props.admin {
                            true => rsx!{ // 管理者の時
                                EditEpisode{
                                    on_submit: modify_submitted_episode,
                                    base_episode: episode.clone(),
                                    on_cancel: close_add_episode,
                                    on_remove: remove_episode
                                }
                            },
                            false => rsx!{ // 管理者でない時
                                EditEpisode{
                                    on_submit: modify_submitted_episode,
                                    base_episode: episode.clone(),
                                    on_cancel: close_add_episode,
                                }
                            }
                        }



                    }
                }
            },
            EditEpisodeOpen::Close => rsx!{Option::<VNode>::None},
        }
    })
}
