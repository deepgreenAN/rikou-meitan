use domain::movie_clip::MovieClip;
use frontend::{
    commands::movie_clip_commands, usecases::movie_clip_usecase, AppCommonError, AppFrontError,
};

use fake::{Fake, Faker};
use pretty_assertions::assert_eq;
use rand::Rng;
use rand::{seq::SliceRandom, thread_rng};
use rstest::{fixture, rstest};
use serial_test::serial;
use std::cmp::Ordering;

async fn all_save(clips: &[MovieClip]) -> Result<(), AppFrontError> {
    for clip in clips.iter() {
        let cmd = movie_clip_commands::SaveMovieClipCommand::new(clip);
        movie_clip_usecase::save_movie_clip(cmd).await?
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
                let all_clips = {
                    let cmd = movie_clip_commands::AllMovieClipsCommand;
                    movie_clip_usecase::all_movie_clips(cmd).await
                };

                if let Ok(all_clips) = all_clips {
                    for clip in all_clips.into_iter() {
                        let cmd = movie_clip_commands::RemoveMovieClipCommand::new(clip.id());
                        movie_clip_usecase::remove_movie_clip(cmd).await;
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
#[serial("clip")]
fn test_edit_movie_clip_sequence(_teardown: TearDown) {
    let rt = tokio::runtime::Runtime::new().unwrap();

    rt.block_on(async move {
        let mut clips = (0..100)
            .map(|_| Faker.fake::<MovieClip>())
            .collect::<Vec<_>>();

        all_save(&clips).await.unwrap();

        // 変更するMovieClipsのインデックス
        let mut edit_indices = (0..clips.len()).collect::<Vec<_>>();
        edit_indices.shuffle(&mut thread_rng());
        let edit_number = 10;

        // movie_clipsの一部を変更．
        for i in edit_indices.into_iter().take(edit_number) {
            let clip = clips.get_mut(i).unwrap();
            let new_clip = Faker.fake::<MovieClip>();

            clip.assign(new_clip);

            let cmd = movie_clip_commands::EditMovieClipCommand::new(clip);
            movie_clip_usecase::edit_movie_clip(cmd).await.unwrap();
        }

        // 取得したものと比較
        let mut res = {
            let cmd = movie_clip_commands::AllMovieClipsCommand;
            movie_clip_usecase::all_movie_clips(cmd).await.unwrap()
        };

        res.sort_by_key(|clip| clip.id());
        clips.sort_by_key(|clip| clip.id());

        assert_eq!(res, clips);
    });
}

#[rstest]
#[test]
#[serial("clip")]
fn test_increment_like_movie_clip_sequence(_teardown: TearDown) {
    let rt = tokio::runtime::Runtime::new().unwrap();

    rt.block_on(async move {
        let mut clips = (0..10)
            .map(|_| Faker.fake::<MovieClip>())
            .collect::<Vec<_>>();

        all_save(&clips).await.unwrap();

        let like_lim = 10_usize;
        let like_numbers = (0..clips.len())
            .map(|_| thread_rng().gen_range(0..like_lim))
            .collect::<Vec<_>>();

        for (clip, like_number) in clips.iter_mut().zip(like_numbers.iter()) {
            for _ in 0..*like_number {
                clip.increment_like();

                let cmd = movie_clip_commands::IncrementLikeMovieClipCommand::new(clip.id());
                movie_clip_usecase::increment_like_movie_clip(cmd)
                    .await
                    .unwrap()
            }
        }

        // 取得したものと比較
        let mut res = {
            let cmd = movie_clip_commands::AllMovieClipsCommand;
            movie_clip_usecase::all_movie_clips(cmd).await.unwrap()
        };

        res.sort_by_key(|clip| clip.id());
        clips.sort_by_key(|clip| clip.id());

        assert_eq!(res, clips);
    });
}

#[rstest]
#[test]
#[serial("clip")]
fn test_order_by_like_sequence(_teardown: TearDown) {
    let rt = tokio::runtime::Runtime::new().unwrap();

    rt.block_on(async move {
        let mut clips = (0..100)
            .map(|_| Faker.fake::<MovieClip>())
            .collect::<Vec<_>>();

        all_save(&clips).await.unwrap();

        let length = 5_usize;

        let mut res = {
            let cmd = movie_clip_commands::OrderByLikeMovieClipsCommand::new(length);
            movie_clip_usecase::order_by_like_movie_clips(cmd)
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

        // Likeで降順・idで昇順にソート
        clips.sort_by(|x, y| y.like().cmp(&x.like()).then(x.id().cmp(&y.id())));
        let clips = clips.into_iter().take(length).collect::<Vec<_>>();

        assert_eq!(res, clips);
    });
}

#[rstest]
#[test]
#[serial("clip")]
fn test_order_by_like_later_sequence(_teardown: TearDown) {
    let rt = tokio::runtime::Runtime::new().unwrap();

    rt.block_on(async move {
        let mut clips = (0..100)
            .map(|_| Faker.fake::<MovieClip>())
            .collect::<Vec<_>>();

        all_save(&clips).await.unwrap();

        let length = 20_usize;
        let reference = clips.choose(&mut thread_rng()).unwrap().clone();

        let mut res = {
            let cmd =
                movie_clip_commands::OrderByLikeLaterMovieClipsCommand::new(&reference, length);
            movie_clip_usecase::order_by_like_later_movie_clips(cmd)
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
        clips.sort_by(|x, y| y.like().cmp(&x.like()).then(x.id().cmp(&y.id())));
        let clips = clips
            .into_iter()
            .filter(|clip| {
                reference.like() > clip.like()
                    || (reference.like() == clip.like() && reference.id() < clip.id())
            })
            .take(length)
            .collect::<Vec<_>>();

        assert_eq!(res, clips);
    });
}

#[rstest]
#[test]
#[serial("clip")]
fn test_order_by_create_date_sequence(_teardown: TearDown) {
    let rt = tokio::runtime::Runtime::new().unwrap();

    rt.block_on(async move {
        let mut clips = (0..100)
            .map(|_| Faker.fake::<MovieClip>())
            .collect::<Vec<_>>();

        all_save(&clips).await.unwrap();

        let length = 20_usize;

        let mut res = {
            let cmd = movie_clip_commands::OrderByCreateDateMovieClipsCommand::new(length);
            movie_clip_usecase::order_by_create_date_movie_clips(cmd)
                .await
                .unwrap()
        };

        // 作成日時が同じ場合はidで昇順．
        res.sort_by(|x, y| {
            if let Ordering::Equal = x.create_date().cmp(&y.create_date()) {
                x.id().cmp(&y.id())
            } else {
                Ordering::Equal
            }
        });

        // 日時で降順・idで昇順にソート
        clips.sort_by(|x, y| {
            y.create_date()
                .cmp(&x.create_date())
                .then(x.id().cmp(&y.id()))
        });
        let clips = clips.into_iter().take(length).collect::<Vec<_>>();

        assert_eq!(res, clips);
    });
}

#[rstest]
#[test]
#[serial("clip")]
fn test_order_by_create_date_later_sequence(_teardown: TearDown) {
    let rt = tokio::runtime::Runtime::new().unwrap();

    rt.block_on(async move {
        let mut clips = (0..100)
            .map(|_| Faker.fake::<MovieClip>())
            .collect::<Vec<_>>();

        all_save(&clips).await.unwrap();

        let length = 20_usize;
        let reference = clips.choose(&mut thread_rng()).unwrap().clone();

        let mut res = {
            let cmd = movie_clip_commands::OrderByCreateDateLaterMovieClipsCommand::new(
                &reference, length,
            );
            movie_clip_usecase::order_by_create_date_later_movie_clips(cmd)
                .await
                .unwrap()
        };

        // 日時が同じ場合はidで昇順．
        res.sort_by(|x, y| {
            if let Ordering::Equal = x.create_date().cmp(&y.create_date()) {
                x.id().cmp(&y.id())
            } else {
                Ordering::Equal
            }
        });

        // 日時で降順・idで昇順にソート・フィルタリング
        clips.sort_by(|x, y| {
            y.create_date()
                .cmp(&x.create_date())
                .then(x.id().cmp(&y.id()))
        });
        let clips = clips
            .into_iter()
            .filter(|clip| {
                reference.create_date() > clip.create_date()
                    || (reference.create_date() == clip.create_date() && reference.id() < clip.id())
            })
            .take(length)
            .collect::<Vec<_>>();

        assert_eq!(res, clips);
    });
}

#[rstest]
#[test]
#[serial("clip")]
fn test_save_movie_clip_failed(_teardown: TearDown) {
    let rt = tokio::runtime::Runtime::new().unwrap();

    rt.block_on(async move {
        let clip = Faker.fake::<MovieClip>();

        // 最初は成功する
        let res = {
            let cmd = movie_clip_commands::SaveMovieClipCommand::new(&clip);
            movie_clip_usecase::save_movie_clip(cmd).await
        };

        assert!(matches!(res, Ok(_)), "{:?}", res.unwrap_err());

        // 二度目は失敗する
        let res = {
            let cmd = movie_clip_commands::SaveMovieClipCommand::new(&clip);
            movie_clip_usecase::save_movie_clip(cmd).await
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
#[serial("clip")]
fn test_edit_clip_failed(_teardown: TearDown) {
    let rt = tokio::runtime::Runtime::new().unwrap();

    rt.block_on(async move {
        let clip = Faker.fake::<MovieClip>();

        let res = {
            let cmd = movie_clip_commands::SaveMovieClipCommand::new(&clip);
            movie_clip_usecase::save_movie_clip(cmd).await
        };

        assert!(matches!(res, Ok(_)), "{:?}", res.unwrap_err());

        // 最初は成功する
        let res = {
            let cmd = movie_clip_commands::EditMovieClipCommand::new(&clip);
            movie_clip_usecase::edit_movie_clip(cmd).await
        };

        assert!(matches!(res, Ok(_)), "{:?}", res.unwrap_err());

        let res = {
            let cmd = movie_clip_commands::RemoveMovieClipCommand::new(clip.id());
            movie_clip_usecase::remove_movie_clip(cmd).await
        };

        assert!(matches!(res, Ok(_)), "{:?}", res.unwrap_err());

        // 削除した後は失敗する
        let res = {
            let cmd = movie_clip_commands::EditMovieClipCommand::new(&clip);
            movie_clip_usecase::edit_movie_clip(cmd).await
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
#[serial("clip")]
fn test_remove_movie_clip_failed(_teardown: TearDown) {
    // ドロップ時に全てを削除
    let _tear_down = TearDown;
    let rt = tokio::runtime::Runtime::new().unwrap();

    rt.block_on(async move {
        let clip = Faker.fake::<MovieClip>();

        let res = {
            let cmd = movie_clip_commands::SaveMovieClipCommand::new(&clip);
            movie_clip_usecase::save_movie_clip(cmd).await
        };

        assert!(matches!(res, Ok(_)), "{:?}", res.unwrap_err());

        // 最初は成功する
        let res = {
            let cmd = movie_clip_commands::RemoveMovieClipCommand::new(clip.id());
            movie_clip_usecase::remove_movie_clip(cmd).await
        };

        assert!(matches!(res, Ok(_)), "{:?}", res.unwrap_err());

        // 次は失敗する
        let res = {
            let cmd = movie_clip_commands::RemoveMovieClipCommand::new(clip.id());
            movie_clip_usecase::remove_movie_clip(cmd).await
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
