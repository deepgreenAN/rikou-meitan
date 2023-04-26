// use assert_matches::assert_matches;
// use domain::movie_clip::{MovieClip, MovieClipId};
// use domain::Date;
// use frontend::commands::movie_clip_commands;
// use frontend::usecases::movie_clip_usecase;
// use frontend::AppCommonError;
// use pretty_assertions::assert_eq;
// use wasm_bindgen_test::*;

// wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

// #[wasm_bindgen_test]
// async fn test_sequence() {
//     let mut movie_clips = vec![
//         MovieClip::new(
//             "Movie Clip 1".to_string(),
//             "https://www.youtube.com/watch?v=B7OPlsdBuVc".to_string(),
//             200,
//             300,
//             (2022, 12, 6),
//         )
//         .unwrap(),
//         MovieClip::new(
//             "Movie Clip 2".to_string(),
//             "https://www.youtube.com/watch?v=M-I0PkzAJaY".to_string(),
//             400,
//             500,
//             (2022, 12, 5),
//         )
//         .unwrap(),
//         MovieClip::new(
//             "Movie Clip 3".to_string(),
//             "https://www.youtube.com/watch?v=ka7ZsQHN_7Y".to_string(),
//             100,
//             200,
//             (2022, 12, 7),
//         )
//         .unwrap(),
//         MovieClip::new(
//             "Movie Clip 4".to_string(),
//             "https://www.youtube.com/watch?v=GzK-gL-YwXs".to_string(),
//             1000,
//             1100,
//             (2022, 12, 8),
//         )
//         .unwrap(),
//     ];

//     // likeをインクリメント
//     for (i, movie_clip) in movie_clips.iter_mut().enumerate() {
//         for _ in 0..i {
//             movie_clip.like_increment();
//         }
//     }

//     // save_movie_clip
//     for movie_clip in movie_clips.iter().cloned() {
//         let cmd = movie_clip_commands::SaveMovieClipCommand::new(movie_clip);
//         let res = movie_clip_usecase::save_movie_clip(cmd).await;
//         assert_matches!(res, Ok(_));
//     }

//     // all_movie_clips
//     let cmd = movie_clip_commands::AllMovieClipCommand;
//     let mut res_vec = movie_clip_usecase::all_movie_clips(cmd).await.unwrap();
//     res_vec.sort_by_key(|movie_clip| movie_clip.id());

//     let mut sorted_movie_clips = movie_clips.clone();
//     sorted_movie_clips.sort_by_key(|movie_clip| movie_clip.id());

//     assert_eq!(res_vec, sorted_movie_clips);

//     // edit_movie_clips and all_movie_clips
//     let mut edited_movie_clip = movie_clips[1].clone();
//     edited_movie_clip
//         .edit_title("Another Movie Clip".to_string())
//         .unwrap();
//     movie_clips[1] = edited_movie_clip.clone();

//     let cmd = movie_clip_commands::EditMovieClipCommand::new(edited_movie_clip);
//     let res = movie_clip_usecase::edit_movie_clip(cmd).await;
//     assert_matches!(res, Ok(_));

//     let cmd = movie_clip_commands::AllMovieClipCommand;
//     let mut res_vec = movie_clip_usecase::all_movie_clips(cmd).await.unwrap();
//     res_vec.sort_by_key(|movie_clip| movie_clip.id());

//     let mut sorted_movie_clips = movie_clips.clone();
//     sorted_movie_clips.sort_by_key(|movie_clip| movie_clip.id());

//     assert_eq!(res_vec, sorted_movie_clips);

//     let another_movie_clip = MovieClip::new(
//         "Another Movie Clip 1".to_string(),
//         "https://www.youtube.com/watch?v=UEBTtT4JQk8".to_string(),
//         1200,
//         1300,
//         (2022, 12, 8),
//     )
//     .unwrap();

//     let cmd = movie_clip_commands::EditMovieClipCommand::new(another_movie_clip);
//     let res = movie_clip_usecase::edit_movie_clip(cmd).await;
//     assert_matches!(res, Err(AppCommonError::NoRecordError));

//     // order_by_like_limit_movie_clips
//     let length = 3_usize;
//     let cmd = movie_clip_commands::OrderByLikeLimitMovieClipCommand::new(length);
//     let res_vec = movie_clip_usecase::order_movie_clips_by_like_limit(cmd)
//         .await
//         .unwrap();

//     let mut ordered_movie_clips = movie_clips.clone();
//     ordered_movie_clips.sort_by_key(|movie_clip| u32::MAX - movie_clip.like());
//     let ordered_movie_clips = ordered_movie_clips
//         .into_iter()
//         .take(length)
//         .collect::<Vec<_>>();
//     assert_eq!(res_vec, ordered_movie_clips);

//     // order_by_create_date_movie_clips
//     let start = Date::from_ymd(2022, 12, 5).unwrap();
//     let end = Date::from_ymd(2022, 12, 6).unwrap();
//     let cmd = movie_clip_commands::OrderByCreateDateRangeMovieClipCommand::new(start, end);
//     let res_vec = movie_clip_usecase::order_movie_clips_by_create_date_range(cmd)
//         .await
//         .unwrap();

//     let mut ordered_movie_clips = movie_clips.clone();
//     ordered_movie_clips.sort_by_key(|movie_clip| movie_clip.create_date());
//     let ordered_movie_clips = ordered_movie_clips
//         .into_iter()
//         .filter(|movie_clip| start <= movie_clip.create_date() && movie_clip.create_date() < end)
//         .collect::<Vec<_>>();

//     assert_eq!(res_vec, ordered_movie_clips);

//     // remove_by_id_movie_clip
//     let removed_movie_clip = movie_clips.remove(1);
//     let cmd = movie_clip_commands::RemoveMovieClipCommand::new(removed_movie_clip.id());
//     let res = movie_clip_usecase::remove_movie_clip(cmd).await;
//     assert_matches!(res, Ok(_));

//     let cmd = movie_clip_commands::AllMovieClipCommand;
//     let mut res_vec = movie_clip_usecase::all_movie_clips(cmd).await.unwrap();
//     res_vec.sort_by_key(|movie_clip| movie_clip.id());

//     let mut sorted_movie_clips = movie_clips.clone();
//     sorted_movie_clips.sort_by_key(|movie_clip| movie_clip.id());

//     assert_eq!(res_vec, sorted_movie_clips);

//     let cmd = movie_clip_commands::RemoveMovieClipCommand::new(MovieClipId::generate());
//     let res = movie_clip_usecase::remove_movie_clip(cmd).await;
//     assert_matches!(res, Err(AppCommonError::NoRecordError));
// }
