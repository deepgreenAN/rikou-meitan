use crate::commands::movie_clip_commands::{
    AllMovieClipCommand, EditMovieClipCommand, OrderByCreateDateRangeMovieClipCommand,
    OrderByLikeLimitMovieClipCommand, RemoveByIdMovieClipCommand, SaveMovieClipCommand,
};
use common::AppCommonError;
use domain::{movie_clip::MovieClip, MovieClipRepository};
use infrastructure::InfraError;
use std::sync::Arc;

pub(crate) async fn save_movie_clip_usecase<T>(
    repo: Arc<T>,
    cmd: SaveMovieClipCommand,
) -> Result<(), AppCommonError>
where
    T: MovieClipRepository<Error = InfraError>,
{
    repo.save(cmd.movie_clip).await?;
    Ok(())
}

pub(crate) async fn edit_movie_clip_usecase<T>(
    repo: Arc<T>,
    cmd: EditMovieClipCommand,
) -> Result<(), AppCommonError>
where
    T: MovieClipRepository<Error = InfraError>,
{
    repo.edit(cmd.movie_clip).await?;
    Ok(())
}

pub(crate) async fn all_movie_clips_usecase<T>(
    repo: Arc<T>,
    _cmd: AllMovieClipCommand,
) -> Result<Vec<MovieClip>, AppCommonError>
where
    T: MovieClipRepository<Error = InfraError>,
{
    Ok(repo.all().await?)
}

pub(crate) async fn order_by_like_limit_movie_clips_usecase<T>(
    repo: Arc<T>,
    cmd: OrderByLikeLimitMovieClipCommand,
) -> Result<Vec<MovieClip>, AppCommonError>
where
    T: MovieClipRepository<Error = InfraError>,
{
    Ok(repo.order_by_like_limit(cmd.length).await?)
}

pub(crate) async fn order_by_create_date_range_movie_clips_usecase<T>(
    repo: Arc<T>,
    cmd: OrderByCreateDateRangeMovieClipCommand,
) -> Result<Vec<MovieClip>, AppCommonError>
where
    T: MovieClipRepository<Error = InfraError>,
{
    Ok(repo.order_by_create_date_range(cmd.start, cmd.end).await?)
}

pub(crate) async fn remove_by_id_movie_clip_usecase<T>(
    repo: Arc<T>,
    cmd: RemoveByIdMovieClipCommand,
) -> Result<(), AppCommonError>
where
    T: MovieClipRepository<Error = InfraError>,
{
    Ok(repo.remove_by_id(cmd.id).await?)
}

#[cfg(test)]
mod test {
    use assert_matches::assert_matches;
    use common::AppCommonError;
    use domain::{
        movie_clip::{MovieClip, MovieClipId},
        Date,
    };
    use infrastructure::MockMovieClipRepository;
    use pretty_assertions::assert_eq;
    use rstest::{fixture, rstest};
    use std::sync::Arc;

    #[fixture]
    async fn movie_clips_and_saved_repo(
    ) -> Result<(Vec<MovieClip>, Arc<MockMovieClipRepository>), AppCommonError> {
        let movie_clips = vec![
            MovieClip::new(
                "MovieClip 1".to_string(),
                "https://www.youtube.com/watch?v=B7OPlsdBuVc".to_string(),
                100,
                200,
                (2022, 11, 21),
            )?,
            MovieClip::new(
                "MovieClip 2".to_string(),
                "https://www.youtube.com/watch?v=NHpILI4NpCI".to_string(),
                200,
                300,
                (2022, 11, 22),
            )?,
            MovieClip::new(
                "MovieClip 3".to_string(),
                "https://www.youtube.com/watch?v=6LAn0lbMpZ8".to_string(),
                400,
                500,
                (2022, 11, 19),
            )?,
        ];

        let movie_clips_length = movie_clips.len();

        let movie_clips = movie_clips
            .into_iter()
            .enumerate()
            .map(|(i, mut movie_clip)| {
                for _ in 0..(movie_clips_length - i) {
                    movie_clip.like_increment(); // likeをインクリメント
                }
                movie_clip
            })
            .collect::<Vec<_>>();

        let repo = Arc::new(MockMovieClipRepository::new());
        for movie_clip in movie_clips.iter().cloned() {
            let cmd = super::SaveMovieClipCommand::new(movie_clip);
            super::save_movie_clip_usecase(Arc::clone(&repo), cmd).await?;
        }
        Ok((movie_clips, repo))
    }

    #[rstest]
    #[tokio::test]
    async fn test_edit_movie_clip_usecase(
        #[future] movie_clips_and_saved_repo: Result<
            (Vec<MovieClip>, Arc<MockMovieClipRepository>),
            AppCommonError,
        >,
    ) -> Result<(), AppCommonError> {
        let (mut movie_clips, repo) = movie_clips_and_saved_repo.await?;

        let mut edited_movie_clip = movie_clips[0].clone();
        edited_movie_clip.edit_title("New Movie Clip".to_string())?;
        movie_clips[0] = edited_movie_clip.clone();

        let cmd = super::EditMovieClipCommand::new(edited_movie_clip);
        super::edit_movie_clip_usecase(Arc::clone(&repo), cmd).await?;

        let cmd = super::AllMovieClipCommand;
        let mut all_movie_clips = super::all_movie_clips_usecase(Arc::clone(&repo), cmd).await?;

        all_movie_clips.sort_by_key(|movie_clip| movie_clip.id());
        movie_clips.sort_by_key(|movie_clip| movie_clip.id());

        assert_eq!(movie_clips, all_movie_clips);
        Ok(())
    }

    #[rstest]
    #[tokio::test]
    async fn test_all_movie_clips_usecase(
        #[future] movie_clips_and_saved_repo: Result<
            (Vec<MovieClip>, Arc<MockMovieClipRepository>),
            AppCommonError,
        >,
    ) -> Result<(), AppCommonError> {
        let (mut movie_clips, repo) = movie_clips_and_saved_repo.await?;

        let cmd = super::AllMovieClipCommand;
        let mut all_movie_clips = super::all_movie_clips_usecase(Arc::clone(&repo), cmd).await?;

        all_movie_clips.sort_by_key(|movie_clip| movie_clip.id());
        movie_clips.sort_by_key(|movie_clip| movie_clip.id());

        assert_eq!(movie_clips, all_movie_clips);
        Ok(())
    }

    #[rstest]
    #[tokio::test]
    async fn test_order_by_like_limit_movie_clips_usecase(
        #[future] movie_clips_and_saved_repo: Result<
            (Vec<MovieClip>, Arc<MockMovieClipRepository>),
            AppCommonError,
        >,
    ) -> Result<(), AppCommonError> {
        let (mut movie_clips, repo) = movie_clips_and_saved_repo.await?;

        let length = 2_usize;
        let cmd = super::OrderByLikeLimitMovieClipCommand::new(length);
        let all_movie_clips =
            super::order_by_like_limit_movie_clips_usecase(Arc::clone(&repo), cmd).await?;

        movie_clips.sort_by_key(|movie_clip| u32::MAX - movie_clip.like()); // 降順
        let movie_clips = movie_clips.into_iter().take(length).collect::<Vec<_>>();

        assert_eq!(movie_clips, all_movie_clips);
        Ok(())
    }

    #[rstest]
    #[tokio::test]
    async fn test_order_by_create_data_range_movie_clips_usecase(
        #[future] movie_clips_and_saved_repo: Result<
            (Vec<MovieClip>, Arc<MockMovieClipRepository>),
            AppCommonError,
        >,
    ) -> Result<(), AppCommonError> {
        let (mut movie_clips, repo) = movie_clips_and_saved_repo.await?;

        let start = Date::from_ymd(2022, 12, 19)?;
        let end = Date::from_ymd(2022, 12, 22)?;
        let cmd = super::OrderByCreateDateRangeMovieClipCommand::new(start, end);
        let all_movie_clips =
            super::order_by_create_date_range_movie_clips_usecase(Arc::clone(&repo), cmd).await?;

        movie_clips.sort_by_key(|movie_clip| movie_clip.create_date());
        let movie_clips = movie_clips
            .into_iter()
            .filter(|movie_clip| {
                start <= movie_clip.create_date() && end < movie_clip.create_date()
            })
            .collect::<Vec<_>>();

        assert_eq!(movie_clips, all_movie_clips);
        Ok(())
    }

    #[rstest]
    #[tokio::test]
    async fn test_remove_by_id_movie_clips_usecase(
        #[future] movie_clips_and_saved_repo: Result<
            (Vec<MovieClip>, Arc<MockMovieClipRepository>),
            AppCommonError,
        >,
    ) -> Result<(), AppCommonError> {
        let (mut movie_clips, repo) = movie_clips_and_saved_repo.await?;

        let removed_movie_clip = movie_clips.remove(1); // 二番目を削除
        let cmd = super::RemoveByIdMovieClipCommand::new(removed_movie_clip.id());
        super::remove_by_id_movie_clip_usecase(Arc::clone(&repo), cmd).await?;

        let cmd = super::AllMovieClipCommand;
        let mut all_movie_clips = super::all_movie_clips_usecase(Arc::clone(&repo), cmd).await?;

        all_movie_clips.sort_by_key(|movie_clip| movie_clip.id());
        movie_clips.sort_by_key(|movie_clip| movie_clip.id());

        assert_eq!(movie_clips, all_movie_clips);
        Ok(())
    }

    #[rstest]
    #[tokio::test]
    async fn test_edit_movie_clip_usecase_not_exists() -> Result<(), AppCommonError> {
        let repo = Arc::new(MockMovieClipRepository::new());

        let movie_clip = MovieClip::new(
            "MovieClip 1".to_string(),
            "https://www.youtube.com/watch?v=B7OPlsdBuVc".to_string(),
            100,
            200,
            (2022, 11, 21),
        )?;

        let cmd = super::EditMovieClipCommand::new(movie_clip);
        let res = super::edit_movie_clip_usecase(Arc::clone(&repo), cmd).await;

        assert_matches!(res, Err(AppCommonError::RemovedRecordError));
        Ok(())
    }

    #[rstest]
    #[tokio::test]
    async fn test_remove_by_id_movie_clip_usecase_not_exists() -> Result<(), AppCommonError> {
        let repo = Arc::new(MockMovieClipRepository::new());

        let cmd = super::RemoveByIdMovieClipCommand::new(MovieClipId::generate());
        let res = super::remove_by_id_movie_clip_usecase(Arc::clone(&repo), cmd).await;

        assert_matches!(res, Err(AppCommonError::RemovedRecordError));
        Ok(())
    }
}
