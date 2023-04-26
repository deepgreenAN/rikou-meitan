// use assert_matches::assert_matches;
// use domain::episode::{Episode, EpisodeId};
// use domain::Date;
// use frontend::commands::episode_commands;
// use frontend::usecases::episode_usecase;
// use frontend::AppCommonError;
// use pretty_assertions::assert_eq;
// use wasm_bindgen_test::*;

// wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

// #[wasm_bindgen_test]
// async fn test_sequence() {
//     let mut episodes = vec![
//         Episode::new((2022, 12, 6), "Episode Content 1".to_string()).unwrap(),
//         Episode::new((2022, 12, 7), "Episode Content 2".to_string()).unwrap(),
//         Episode::new((2022, 12, 8), "Episode Content 3".to_string()).unwrap(),
//         Episode::new((2022, 12, 9), "Episode Content 4".to_string()).unwrap(),
//     ];

//     // save_episode
//     for episode in episodes.iter().cloned() {
//         let cmd = episode_commands::SaveEpisodeCommand::new(episode);
//         let res = episode_usecase::save_episode(cmd).await;
//         assert_matches!(res, Ok(_));
//     }

//     // all_episodes
//     let cmd = episode_commands::AllEpisodeCommand;
//     let mut res_vec = episode_usecase::all_episodes(cmd).await.unwrap();
//     res_vec.sort_by_key(|episode| episode.id());

//     let mut sorted_episodes = episodes.clone();
//     sorted_episodes.sort_by_key(|episode| episode.id());
//     assert_eq!(res_vec, sorted_episodes);

//     // edit_episode
//     let mut edited_episode = episodes[1].clone();
//     edited_episode
//         .edit_content("Another Episode content".to_string())
//         .unwrap();
//     episodes[1] = edited_episode.clone();

//     let cmd = episode_commands::EditEpisodeCommand::new(edited_episode);
//     let res = episode_usecase::edit_episode(cmd).await;
//     assert_matches!(res, Ok(_));

//     let another_episode =
//         Episode::new((2022, 12, 9), "Another Episode Content".to_string()).unwrap();
//     let cmd = episode_commands::EditEpisodeCommand::new(another_episode);
//     let res = episode_usecase::edit_episode(cmd).await;
//     assert_matches!(res, Err(AppCommonError::NoRecordError));

//     // order_by_date_range_episodes
//     let start = Date::from_ymd(2022, 12, 6).unwrap();
//     let end = Date::from_ymd(2022, 12, 8).unwrap();
//     let cmd = episode_commands::OrderByDateRangeEpisodeCommand::new(start, end);
//     let res_vec = episode_usecase::order_by_date_range_episodes(cmd)
//         .await
//         .unwrap();

//     let mut ordered_episodes = episodes.clone();
//     ordered_episodes.sort_by_key(|episode| episode.date());
//     let ordered_episodes = ordered_episodes
//         .into_iter()
//         .filter(|episode| start <= episode.date() && episode.date() < end)
//         .collect::<Vec<_>>();

//     assert_eq!(res_vec, ordered_episodes);

//     // remove_by_id_episode
//     let removed_episode = episodes.remove(1);
//     let cmd = episode_commands::RemoveByIdEpisodeCommand::new(removed_episode.id());
//     let res = episode_usecase::remove_by_id_episode(cmd).await;
//     assert_matches!(res, Ok(_));

//     let cmd = episode_commands::AllEpisodeCommand;
//     let mut res_vec = episode_usecase::all_episodes(cmd).await.unwrap();
//     res_vec.sort_by_key(|episode| episode.id());

//     let mut sorted_episodes = episodes.clone();
//     sorted_episodes.sort_by_key(|episode| episode.id());
//     assert_eq!(res_vec, sorted_episodes);

//     let cmd = episode_commands::RemoveByIdEpisodeCommand::new(EpisodeId::generate());
//     let res = episode_usecase::remove_by_id_episode(cmd).await;
//     assert_matches!(res, Err(AppCommonError::NoRecordError));
// }
