use domain::episode::{Episode, EpisodeId};
use domain::Date;
use frontend::commands::episode_commands;
use frontend::usecases::episode_usecase;
use frontend::AppFrontError;

use fake::{Fake, Faker};
use pretty_assertions::assert_eq;
use rand::{seq::SliceRandom, thread_rng, Rng};
use tokio::runtime::Handle;

mod utils;
use utils::use_sync_mutex;

async fn all_save(episodes: &[Episode]) -> Result<(), AppFrontError> {
    for episode in episodes.iter() {
        let cmd = episode_commands::SaveEpisodeCommand::new(episode);
        episode_usecase::save_episode(cmd).await?
    }
    Ok(())
}

async fn all_remove() -> Result<(), AppFrontError> {
    let all_episodes = {
        let cmd = episode_commands::AllEpisodesCommand;
        episode_usecase::all_episodes(cmd).await?
    };

    for episode in all_episodes.into_iter() {
        let cmd = episode_commands::RemoveEpisodeCommand::new(episode.id());
        episode_usecase::remove_episode(cmd).await?
    }
    Ok(())
}

#[test]
fn test_edit_episode_sequence() {
    // let _m = use_sync_mutex();

    futures::executor::block_on(async move {
        let episode = Faker.fake::<Episode>();
        let res = {
            let cmd = episode_commands::SaveEpisodeCommand::new(&episode);
            episode_usecase::save_episode(cmd).await
        };

        // assert!(matches!(res, Ok(_)), "{}", res.unwrap_err());

        // let mut episodes = (0..10).map(|_| Faker.fake::<Episode>()).collect::<Vec<_>>();

        // all_save(&episodes).await;

        // // 変更するエピソードのインデックス
        // let mut edit_indices = (0..episodes.len()).collect::<Vec<_>>();
        // edit_indices.shuffle(&mut thread_rng());
        // let edit_number = 5;

        // // episodesの一部を変更．
        // for i in edit_indices.into_iter().take(edit_number) {
        //     let episode = episodes.get_mut(i).unwrap();
        //     let new_episode = Faker.fake::<Episode>();

        //     episode.assign(new_episode);

        //     let cmd = episode_commands::EditEpisodeCommand::new(episode);
        //     episode_usecase::edit_episode(cmd).await.unwrap();
        // }

        // // 取得したものと比較
        // let mut res = {
        //     let cmd = episode_commands::AllEpisodesCommand;
        //     episode_usecase::all_episodes(cmd).await.unwrap()
        // };

        // res.sort_by_key(|episode| episode.id());
        // episodes.sort_by_key(|episode| episode.id());

        // assert_eq!(res, episodes);

        // all_remove().await.unwrap();
    });
}
