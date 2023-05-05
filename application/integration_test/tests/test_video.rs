use domain::video::{Original, Video};
use frontend::{commands::video_commands, usecases::video_usecase, AppCommonError, AppFrontError};

use fake::{Fake, Faker};
use pretty_assertions::assert_eq;
use rand::Rng;
use rand::{seq::SliceRandom, thread_rng};
use rstest::{fixture, rstest};
use serial_test::serial;
use std::cmp::Ordering;

async fn all_save(videos: &[Video<Original>]) -> Result<(), AppFrontError> {
    for video in videos.iter() {
        let cmd = video_commands::SaveVideoCommand::new(video);
        video_usecase::save_video(cmd).await?
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
                let all_videos = {
                    let cmd = video_commands::AllVideosCommand;
                    video_usecase::all_videos::<Original>(cmd).await
                };

                if let Ok(all_videos) = all_videos {
                    for video in all_videos.into_iter() {
                        let cmd = video_commands::RemoveVideoCommand::new(video.id());
                        video_usecase::remove_video::<Original>(cmd).await;
                    }
                }
            });
        }
    }
}

#[fixture]
fn teardown() -> TearDown {
    TearDown
}

#[rstest]
#[test]
#[serial("video")]
fn test_edit_video_sequence(_teardown: TearDown) {
    let rt = tokio::runtime::Runtime::new().unwrap();

    rt.block_on(async move {
        let mut videos = (0..100)
            .map(|_| Faker.fake::<Video<Original>>())
            .collect::<Vec<_>>();

        all_save(&videos).await.unwrap();

        // 変更するVideo<Original>sのインデックス
        let mut edit_indices = (0..videos.len()).collect::<Vec<_>>();
        edit_indices.shuffle(&mut thread_rng());
        let edit_number = 10;

        // videosの一部を変更．
        for i in edit_indices.into_iter().take(edit_number) {
            let video = videos.get_mut(i).unwrap();
            let new_video = Faker.fake::<Video<Original>>();

            video.assign(new_video);

            let cmd = video_commands::EditVideoCommand::new(video);
            video_usecase::edit_video(cmd).await.unwrap();
        }

        // 取得したものと比較
        let mut res = {
            let cmd = video_commands::AllVideosCommand;
            video_usecase::all_videos::<Original>(cmd).await.unwrap()
        };

        res.sort_by_key(|video| video.id());
        videos.sort_by_key(|video| video.id());

        assert_eq!(res, videos);
    });
}

#[rstest]
#[test]
#[serial("video")]
fn test_increment_like_video_sequence(_teardown: TearDown) {
    let rt = tokio::runtime::Runtime::new().unwrap();

    rt.block_on(async move {
        let mut videos = (0..10)
            .map(|_| Faker.fake::<Video<Original>>())
            .collect::<Vec<_>>();

        all_save(&videos).await.unwrap();

        let like_lim = 10_usize;
        let like_numbers = (0..videos.len())
            .map(|_| thread_rng().gen_range(0..like_lim))
            .collect::<Vec<_>>();

        for (video, like_number) in videos.iter_mut().zip(like_numbers.iter()) {
            for _ in 0..*like_number {
                video.increment_like();

                let cmd = video_commands::IncrementLikeVideoCommand::new(video.id());
                video_usecase::increment_like_video::<Original>(cmd)
                    .await
                    .unwrap()
            }
        }

        // 取得したものと比較
        let mut res = {
            let cmd = video_commands::AllVideosCommand;
            video_usecase::all_videos::<Original>(cmd).await.unwrap()
        };

        res.sort_by_key(|video| video.id());
        videos.sort_by_key(|video| video.id());

        assert_eq!(res, videos);
    });
}

#[rstest]
#[test]
#[serial("video")]
fn test_order_by_like_sequence(_teardown: TearDown) {
    let rt = tokio::runtime::Runtime::new().unwrap();

    rt.block_on(async move {
        let mut videos = (0..100)
            .map(|_| Faker.fake::<Video<Original>>())
            .collect::<Vec<_>>();

        all_save(&videos).await.unwrap();

        let length = 5_usize;

        let mut res = {
            let cmd = video_commands::OrderByLikeVideosCommand::new(length);
            video_usecase::order_by_like_videos(cmd).await.unwrap()
        };

        // Likeが同じ場合はidで昇順．
        res.sort_by(|x, y| {
            if let Ordering::Equal = x.like().cmp(&y.like()) {
                x.id().cmp(&y.id())
            } else {
                Ordering::Equal
            }
        });

        // Likeで降順・idで昇順にソート
        videos.sort_by(|x, y| y.like().cmp(&x.like()).then(x.id().cmp(&y.id())));
        let videos = videos.into_iter().take(length).collect::<Vec<_>>();

        assert_eq!(res, videos);
    });
}

#[rstest]
#[test]
#[serial("video")]
fn test_order_by_like_later_sequence(_teardown: TearDown) {
    let rt = tokio::runtime::Runtime::new().unwrap();

    rt.block_on(async move {
        let mut videos = (0..100)
            .map(|_| Faker.fake::<Video<Original>>())
            .collect::<Vec<_>>();

        all_save(&videos).await.unwrap();

        let length = 20_usize;
        let reference = videos.choose(&mut thread_rng()).unwrap().clone();

        let mut res = {
            let cmd = video_commands::OrderByLikeLaterVideosCommand::new(&reference, length);
            video_usecase::order_by_like_later_videos(cmd)
                .await
                .unwrap()
        };

        // Likeが同じ場合はidで昇順．
        res.sort_by(|x, y| {
            if let Ordering::Equal = x.like().cmp(&y.like()) {
                x.id().cmp(&y.id())
            } else {
                Ordering::Equal
            }
        });

        // Likeで降順・idで昇順にソート・フィルタリング
        videos.sort_by(|x, y| y.like().cmp(&x.like()).then(x.id().cmp(&y.id())));
        let videos = videos
            .into_iter()
            .filter(|video| {
                reference.like() > video.like()
                    || (reference.like() == video.like() && reference.id() < video.id())
            })
            .take(length)
            .collect::<Vec<_>>();

        assert_eq!(res, videos);
    });
}

#[rstest]
#[test]
#[serial("video")]
fn test_order_by_date_sequence(_teardown: TearDown) {
    let rt = tokio::runtime::Runtime::new().unwrap();

    rt.block_on(async move {
        let mut videos = (0..100)
            .map(|_| Faker.fake::<Video<Original>>())
            .collect::<Vec<_>>();

        all_save(&videos).await.unwrap();

        let length = 20_usize;

        let mut res = {
            let cmd = video_commands::OrderByDateVideosCommand::new(length);
            video_usecase::order_by_date_videos(cmd).await.unwrap()
        };

        // 作成日時が同じ場合はidで昇順．
        res.sort_by(|x, y| {
            if let Ordering::Equal = x.date().cmp(&y.date()) {
                x.id().cmp(&y.id())
            } else {
                Ordering::Equal
            }
        });

        // 日時で降順・idで昇順にソート
        videos.sort_by(|x, y| y.date().cmp(&x.date()).then(x.id().cmp(&y.id())));
        let videos = videos.into_iter().take(length).collect::<Vec<_>>();

        assert_eq!(res, videos);
    });
}

#[rstest]
#[test]
#[serial("video")]
fn test_order_by_date_later_sequence(_teardown: TearDown) {
    let rt = tokio::runtime::Runtime::new().unwrap();

    rt.block_on(async move {
        let mut videos = (0..100)
            .map(|_| Faker.fake::<Video<Original>>())
            .collect::<Vec<_>>();

        all_save(&videos).await.unwrap();

        let length = 20_usize;
        let reference = videos.choose(&mut thread_rng()).unwrap().clone();

        let mut res = {
            let cmd = video_commands::OrderByDateLaterVideosCommand::new(&reference, length);
            video_usecase::order_by_date_later_videos(cmd)
                .await
                .unwrap()
        };

        // 日時が同じ場合はidで昇順．
        res.sort_by(|x, y| {
            if let Ordering::Equal = x.date().cmp(&y.date()) {
                x.id().cmp(&y.id())
            } else {
                Ordering::Equal
            }
        });

        // 日時で降順・idで昇順にソート・フィルタリング
        videos.sort_by(|x, y| y.date().cmp(&x.date()).then(x.id().cmp(&y.id())));
        let videos = videos
            .into_iter()
            .filter(|video| {
                reference.date() > video.date()
                    || (reference.date() == video.date() && reference.id() < video.id())
            })
            .take(length)
            .collect::<Vec<_>>();

        assert_eq!(res, videos);
    });
}

#[rstest]
#[test]
#[serial("video")]
fn test_save_video_failed(_teardown: TearDown) {
    let rt = tokio::runtime::Runtime::new().unwrap();

    rt.block_on(async move {
        let video = Faker.fake::<Video<Original>>();

        // 最初は成功する
        let res = {
            let cmd = video_commands::SaveVideoCommand::new(&video);
            video_usecase::save_video(cmd).await
        };

        assert!(matches!(res, Ok(_)), "{:?}", res.unwrap_err());

        // 二度目は失敗する
        let res = {
            let cmd = video_commands::SaveVideoCommand::new(&video);
            video_usecase::save_video(cmd).await
        };

        assert!(
            matches!(
                res,
                Err(AppFrontError::CommonError(AppCommonError::ConflictError))
            ),
            "{:?}",
            res.unwrap_err()
        );
    });
}

#[rstest]
#[test]
#[serial("video")]
fn test_edit_video_failed(_teardown: TearDown) {
    let rt = tokio::runtime::Runtime::new().unwrap();

    rt.block_on(async move {
        let video = Faker.fake::<Video<Original>>();

        let res = {
            let cmd = video_commands::SaveVideoCommand::new(&video);
            video_usecase::save_video(cmd).await
        };

        assert!(matches!(res, Ok(_)), "{:?}", res.unwrap_err());

        // 最初は成功する
        let res = {
            let cmd = video_commands::EditVideoCommand::new(&video);
            video_usecase::edit_video(cmd).await
        };

        assert!(matches!(res, Ok(_)), "{:?}", res.unwrap_err());

        let res = {
            let cmd = video_commands::RemoveVideoCommand::new(video.id());
            video_usecase::remove_video::<Original>(cmd).await
        };

        assert!(matches!(res, Ok(_)), "{:?}", res.unwrap_err());

        // 削除した後は失敗する
        let res = {
            let cmd = video_commands::EditVideoCommand::new(&video);
            video_usecase::edit_video(cmd).await
        };

        assert!(
            matches!(
                res,
                Err(AppFrontError::CommonError(AppCommonError::NoRecordError))
            ),
            "{:?}",
            res.unwrap_err()
        );
    });
}

#[rstest]
#[test]
#[serial("video")]
fn test_remove_video_failed(_teardown: TearDown) {
    // ドロップ時に全てを削除
    let _tear_down = TearDown;
    let rt = tokio::runtime::Runtime::new().unwrap();

    rt.block_on(async move {
        let video = Faker.fake::<Video<Original>>();

        let res = {
            let cmd = video_commands::SaveVideoCommand::new(&video);
            video_usecase::save_video(cmd).await
        };

        assert!(matches!(res, Ok(_)), "{:?}", res.unwrap_err());

        // 最初は成功する
        let res = {
            let cmd = video_commands::RemoveVideoCommand::new(video.id());
            video_usecase::remove_video::<Original>(cmd).await
        };

        assert!(matches!(res, Ok(_)), "{:?}", res.unwrap_err());

        // 次は失敗する
        let res = {
            let cmd = video_commands::RemoveVideoCommand::new(video.id());
            video_usecase::remove_video::<Original>(cmd).await
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
