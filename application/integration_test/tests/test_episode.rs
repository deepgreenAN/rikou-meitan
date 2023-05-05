use domain::episode::Episode;
use domain::Date;
use frontend::commands::episode_commands;
use frontend::usecases::episode_usecase;
use frontend::AppFrontError;

use fake::{Fake, Faker};
use pretty_assertions::assert_eq;
use rand::{seq::SliceRandom, thread_rng};
use rstest::{fixture, rstest};
use serial_test::serial;
use std::cmp::Ordering;

async fn all_save(episodes: &[Episode]) -> Result<(), AppFrontError> {
    for episode in episodes.iter() {
        let cmd = episode_commands::SaveEpisodeCommand::new(episode);
        episode_usecase::save_episode(cmd).await?
    }
    Ok(())
}
struct TearDown;

#[allow(unused_must_use)]
impl Drop for TearDown {
    fn drop(&mut self) {
        let rt = tokio::runtime::Runtime::new();

        if let Ok(rt) = rt {
            rt.block_on(async move {
                let all_episodes = {
                    let cmd = episode_commands::AllEpisodesCommand;
                    episode_usecase::all_episodes(cmd).await
                };

                if let Ok(all_episodes) = all_episodes {
                    for episode in all_episodes.into_iter() {
                        let cmd = episode_commands::RemoveEpisodeCommand::new(episode.id());
                        episode_usecase::remove_episode(cmd).await;
                    }
                }
            });
        }
    }
}

/// teardownの関数
#[fixture]
fn teardown() -> TearDown {
    TearDown
}

#[rstest]
#[test]
#[serial("episode")]
fn test_edit_episode_sequence(_teardown: TearDown) {
    let rt = tokio::runtime::Runtime::new().unwrap();

    rt.block_on(async move {
        let mut episodes = (0..100)
            .map(|_| Faker.fake::<Episode>())
            .collect::<Vec<_>>();

        all_save(&episodes).await.unwrap();

        // 変更するエピソードのインデックス
        let mut edit_indices = (0..episodes.len()).collect::<Vec<_>>();
        edit_indices.shuffle(&mut thread_rng());
        let edit_number = 10;

        // episodesの一部を変更．
        for i in edit_indices.into_iter().take(edit_number) {
            let episode = episodes.get_mut(i).unwrap();
            let new_episode = Faker.fake::<Episode>();

            episode.assign(new_episode);

            let cmd = episode_commands::EditEpisodeCommand::new(episode);
            episode_usecase::edit_episode(cmd).await.unwrap();
        }

        // 取得したものと比較
        let mut res = {
            let cmd = episode_commands::AllEpisodesCommand;
            episode_usecase::all_episodes(cmd).await.unwrap()
        };

        res.sort_by_key(|episode| episode.id());
        episodes.sort_by_key(|episode| episode.id());

        assert_eq!(res, episodes);
    });
}

#[rstest]
#[test]
#[serial("episode")]
fn test_order_by_range_episodes_sequence(_teardown: TearDown) {
    let rt = tokio::runtime::Runtime::new().unwrap();

    rt.block_on(async move {
        let mut episodes = (0..100)
            .map(|_| Faker.fake::<Episode>())
            .collect::<Vec<_>>();

        all_save(&episodes).await.unwrap();

        let start = Faker.fake::<Date>();
        let end = Faker.fake::<Date>();

        // 参照元をDateを昇順・IDを昇順にソート・フィルタリング
        episodes.sort_by(|x, y| x.date().cmp(&y.date()).then_with(|| x.id().cmp(&y.id())));
        episodes.retain(|episode| start <= episode.date() && episode.date() < end);

        // リクエストをDateが同じ場合のみIDを昇順にソート
        let mut res = {
            let cmd = episode_commands::OrderByDateRangeEpisodesCommand::new(start, end);
            episode_usecase::order_by_date_range_episodes(cmd)
                .await
                .unwrap()
        };

        res.sort_by(|x, y| {
            if let Ordering::Equal = x.date().cmp(&y.date()) {
                x.id().cmp(&y.id()) // idを昇順にする
            } else {
                Ordering::Equal // 同じDateで無い場合はそのまま
            }
        });

        assert_eq!(res, episodes);
    });
}

#[rstest]
#[test]
#[serial("episode")]
fn test_save_episode_failed(_teardown: TearDown) {
    let rt = tokio::runtime::Runtime::new().unwrap();

    rt.block_on(async move {
        let episode = Faker.fake::<Episode>();

        // 最初は成功する
        let res = {
            let cmd = episode_commands::SaveEpisodeCommand::new(&episode);
            episode_usecase::save_episode(cmd).await
        };

        assert!(matches!(res, Ok(_)), "{:?}", res.unwrap_err());

        // 二度目は失敗する
        let res = {
            let cmd = episode_commands::SaveEpisodeCommand::new(&episode);
            episode_usecase::save_episode(cmd).await
        };

        assert!(
            matches!(
                res,
                Err(AppFrontError::CommonError(
                    frontend::AppCommonError::ConflictError
                ))
            ),
            "{:?}",
            res.unwrap_err()
        );
    });
}

#[rstest]
#[test]
#[serial("episode")]
fn test_edit_episode_failed(_teardown: TearDown) {
    let rt = tokio::runtime::Runtime::new().unwrap();

    rt.block_on(async move {
        let episode = Faker.fake::<Episode>();

        let res = {
            let cmd = episode_commands::SaveEpisodeCommand::new(&episode);
            episode_usecase::save_episode(cmd).await
        };

        assert!(matches!(res, Ok(_)), "{:?}", res.unwrap_err());

        // 最初は成功する
        let res = {
            let cmd = episode_commands::EditEpisodeCommand::new(&episode);
            episode_usecase::edit_episode(cmd).await
        };

        assert!(matches!(res, Ok(_)), "{:?}", res.unwrap_err());

        let res = {
            let cmd = episode_commands::RemoveEpisodeCommand::new(episode.id());
            episode_usecase::remove_episode(cmd).await
        };

        assert!(matches!(res, Ok(_)), "{:?}", res.unwrap_err());

        // 削除した後は失敗する
        let res = {
            let cmd = episode_commands::EditEpisodeCommand::new(&episode);
            episode_usecase::edit_episode(cmd).await
        };

        assert!(
            matches!(
                res,
                Err(AppFrontError::CommonError(
                    frontend::AppCommonError::NoRecordError
                ))
            ),
            "{:?}",
            res.unwrap_err()
        );
    });
}

#[rstest]
#[test]
#[serial("episode")]
fn test_remove_episode_failed(_teardown: TearDown) {
    // ドロップ時に全てを削除
    let _tear_down = TearDown;
    let rt = tokio::runtime::Runtime::new().unwrap();

    rt.block_on(async move {
        let episode = Faker.fake::<Episode>();

        let res = {
            let cmd = episode_commands::SaveEpisodeCommand::new(&episode);
            episode_usecase::save_episode(cmd).await
        };

        assert!(matches!(res, Ok(_)), "{:?}", res.unwrap_err());

        // 最初は成功する
        let res = {
            let cmd = episode_commands::RemoveEpisodeCommand::new(episode.id());
            episode_usecase::remove_episode(cmd).await
        };

        assert!(matches!(res, Ok(_)), "{:?}", res.unwrap_err());

        // 次は失敗する
        let res = {
            let cmd = episode_commands::RemoveEpisodeCommand::new(episode.id());
            episode_usecase::remove_episode(cmd).await
        };

        assert!(
            matches!(
                res,
                Err(AppFrontError::CommonError(
                    frontend::AppCommonError::NoRecordError
                ))
            ),
            "{:?}",
            res.unwrap_err()
        );
    });
}
