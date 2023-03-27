#![cfg(feature = "integration_test")]

// use common::AppCommonError;
// use domain::episode::{Episode, EpisodeId};
// use domain::Date;
// use frontend::commands::episode_commands;
// use frontend::usecases::episode_usecase;
// use frontend::AppFrontError;

// use fake::{Fake, Faker};
// use pretty_assertions::assert_eq;
// use rand::seq::SliceRandom;
// use wasm_bindgen_test::*;
// use wasm_rs_async_executor::single_threaded::block_on;

// wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

// fn save_episodes() -> Vec<Episode> {
//     let episodes = (0..100)
//         .map(|_| Faker.fake::<Episode>())
//         .collect::<Vec<_>>();

//     block_on(async move {
//         for episode in episodes.iter() {
//             let cmd = episode_commands::SaveEpisodeCommand::new(episode);
//             episode_usecase::save_episode(cmd).await.unwrap()
//         }
//         episodes
//     })
// }

// fn clear_all() {
//     block_on(async move {
//         let all_episodes = {
//             let cmd = episode_commands::AllEpisodesCommand;
//             episode_usecase::all_episodes(cmd).await.unwrap()
//         };

//         for episode in all_episodes.iter() {
//             let cmd = episode_commands::RemoveEpisodeCommand::new(episode.id());
//             episode_usecase::remove_episode(cmd).await.unwrap();
//         }
//     })
// }

// #[wasm_bindgen_test]
// fn test_save_and_all() {
//     let mut episodes = save_episodes();

//     episodes.sort_by_key(|episode| episode.id());

//     let mut all_episodes = block_on(async move {
//         let cmd = episode_commands::AllEpisodesCommand;
//         episode_usecase::all_episodes(cmd).await.unwrap()
//     });

//     all_episodes.sort_by_key(|episode| episode.id());

//     assert_eq!(episodes, all_episodes);

//     clear_all();
// }
