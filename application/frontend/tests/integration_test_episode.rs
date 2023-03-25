#![cfg(feature = "integration_test")]

// use common::AppCommonError;
// use domain::episode::{Episode, EpisodeId};
// use domain::Date;
// use frontend::commands::episode_commands;
// use frontend::usecases::episode_usecase;
// use frontend::AppFrontError;

// use fake::{Fake, Faker};
// use rand::seq::SliceRandom;
// use wasm_bindgen_test::*;

// wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

// #[wasm_bindgen_test]
// async fn test_sequence() {
//     // データの保存
//     let mut episodes = {
//         let episodes = vec![Faker.fake::<Episode>(); 5];

//         for episode in episodes.iter() {
//             let cmd = episode_commands::SaveEpisodeCommand::new(episode);
//             episode_usecase::save_episode(cmd).await.unwrap();
//         }
//         episodes
//     };

//     // 全データの取得と確認
//     {
//         let cmd = episode_commands::AllEpisodesCommand;
//         let mut all_episodes = episode_usecase::all_episodes(cmd).await.unwrap();

//         all_episodes.sort_by_key(|episode| episode.id());

//         episodes.sort_by_key(|episode| episode.id());

//         assert_eq!(all_episodes, episodes);
//     }

//     // 一部のデータの編集
//     // 全データの削除
//     {
//         let cmd = episode_commands::AllEpisodesCommand;
//         let all_episodes = episode_usecase::all_episodes(cmd).await.unwrap();

//         for episode in all_episodes.into_iter() {
//             let cmd = episode_commands::RemoveEpisodeCommand::new(episode.id());
//             episode_usecase::remove_episode(cmd).await.unwrap();
//         }
//     }
// }
