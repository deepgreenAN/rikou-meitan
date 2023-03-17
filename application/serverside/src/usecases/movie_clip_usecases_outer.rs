#[cfg(test)]
use mockall::automock;

#[cfg_attr(test, automock)]
pub mod movie_clip_usecases {
    use crate::commands::movie_clip_commands::{
        AllMovieClipCommand, EditMovieClipCommand, OrderByCreateDateRangeMovieClipCommand,
        OrderByLikeLimitMovieClipCommand, RemoveByIdMovieClipCommand, SaveMovieClipCommand,
    };
    use common::AppCommonError;
    use domain::{movie_clip::MovieClip, MovieClipRepository};
    use infrastructure::InfraError;
    use std::sync::Arc;

    pub(crate) async fn save_movie_clip<T>(
        repo: Arc<T>,
        cmd: SaveMovieClipCommand,
    ) -> Result<(), AppCommonError>
    where
        T: MovieClipRepository<Error = InfraError> + 'static,
    {
        repo.save(cmd.movie_clip).await?;
        Ok(())
    }

    pub(crate) async fn edit_movie_clip<T>(
        repo: Arc<T>,
        cmd: EditMovieClipCommand,
    ) -> Result<(), AppCommonError>
    where
        T: MovieClipRepository<Error = InfraError> + 'static,
    {
        repo.edit(cmd.movie_clip).await?;
        Ok(())
    }

    pub(crate) async fn all_movie_clips<T>(
        repo: Arc<T>,
        _cmd: AllMovieClipCommand,
    ) -> Result<Vec<MovieClip>, AppCommonError>
    where
        T: MovieClipRepository<Error = InfraError> + 'static,
    {
        Ok(repo.all().await?)
    }

    pub(crate) async fn order_by_like_movie_clips<T>(
        repo: Arc<T>,
        cmd: OrderByLikeLimitMovieClipCommand,
    ) -> Result<Vec<MovieClip>, AppCommonError>
    where
        T: MovieClipRepository<Error = InfraError> + 'static,
    {
        Ok(repo.order_by_like(cmd.length).await?)
    }

    pub(crate) async fn order_by_create_date_range_movie_clips<T>(
        repo: Arc<T>,
        cmd: OrderByCreateDateRangeMovieClipCommand,
    ) -> Result<Vec<MovieClip>, AppCommonError>
    where
        T: MovieClipRepository<Error = InfraError> + 'static,
    {
        Ok(repo.order_by_create_date_range(cmd.start, cmd.end).await?)
    }

    pub(crate) async fn remove_movie_clip<T>(
        repo: Arc<T>,
        cmd: RemoveByIdMovieClipCommand,
    ) -> Result<(), AppCommonError>
    where
        T: MovieClipRepository<Error = InfraError> + 'static,
    {
        Ok(repo.remove(cmd.id).await?)
    }
}

#[cfg(test)]
mod test {
    use super::movie_clip_usecases;
    use crate::commands::movie_clip_commands;
    use infrastructure::InfraError;

    use assert_matches::assert_matches;
    use common::AppCommonError;
    use domain::{
        movie_clip::{MovieClip, MovieClipId},
        Date,
    };
    use infrastructure::movie_clip_repository_impl::MockMovieClipRepositoryImpl;
    use mockall::predicate;
    use pretty_assertions::assert_eq;
    use std::sync::Arc;

    #[tokio::test]
    async fn test_save_movie_clip_usecase() {
        let movie_clip = MovieClip::new(
            "Movie Clip Title".to_string(),
            "https://www.youtube.com/watch?v=B7OPlsdBuVc".to_string(),
            300,
            400,
            (2022, 12, 4),
        )
        .unwrap();

        let mut mock_repo_ok = MockMovieClipRepositoryImpl::new();
        mock_repo_ok
            .expect_save()
            .with(predicate::eq(movie_clip.clone()))
            .times(1)
            .return_const(Ok(()));

        let cmd = movie_clip_commands::SaveMovieClipCommand::new(movie_clip.clone());
        let res_ok = movie_clip_usecases::save_movie_clip(Arc::new(mock_repo_ok), cmd).await;
        assert_matches!(res_ok, Ok(_));

        let mut mock_repo_err = MockMovieClipRepositoryImpl::new();
        mock_repo_err
            .expect_save()
            .with(predicate::eq(movie_clip.clone()))
            .times(1)
            .return_const(Err(InfraError::ConflictError));

        let cmd = movie_clip_commands::SaveMovieClipCommand::new(movie_clip.clone());
        let res_err = movie_clip_usecases::save_movie_clip(Arc::new(mock_repo_err), cmd).await;
        assert_matches!(res_err, Err(AppCommonError::ConflictError));
    }

    #[tokio::test]
    async fn test_edit_movie_clip_usecase() {
        let movie_clip = MovieClip::new(
            "Movie Clip Title".to_string(),
            "https://www.youtube.com/watch?v=B7OPlsdBuVc".to_string(),
            300,
            400,
            (2022, 12, 4),
        )
        .unwrap();

        let mut mock_repo_ok = MockMovieClipRepositoryImpl::new();
        mock_repo_ok
            .expect_edit()
            .with(predicate::eq(movie_clip.clone()))
            .times(1)
            .return_const(Ok(()));

        let cmd = movie_clip_commands::EditMovieClipCommand::new(movie_clip.clone());
        let res_ok = movie_clip_usecases::edit_movie_clip(Arc::new(mock_repo_ok), cmd).await;
        assert_matches!(res_ok, Ok(_));

        let mut mock_repo_err = MockMovieClipRepositoryImpl::new();
        mock_repo_err
            .expect_edit()
            .with(predicate::eq(movie_clip.clone()))
            .times(1)
            .return_const(Err(InfraError::NoRecordError));

        let cmd = movie_clip_commands::EditMovieClipCommand::new(movie_clip.clone());
        let res_err = movie_clip_usecases::edit_movie_clip(Arc::new(mock_repo_err), cmd).await;
        assert_matches!(res_err, Err(AppCommonError::NoRecordError));
    }

    #[tokio::test]
    async fn test_all_movie_clips_usecase() {
        let movie_clips = vec![MovieClip::new(
            "Movie Clip Title".to_string(),
            "https://www.youtube.com/watch?v=B7OPlsdBuVc".to_string(),
            300,
            400,
            (2022, 12, 4),
        )
        .unwrap()];

        let mut mock_repo = MockMovieClipRepositoryImpl::new();
        mock_repo
            .expect_all()
            .times(1)
            .return_const(Ok(movie_clips.clone()));

        let cmd = movie_clip_commands::AllMovieClipCommand;
        let res_vec = movie_clip_usecases::all_movie_clips(Arc::new(mock_repo), cmd)
            .await
            .unwrap();
        assert_eq!(res_vec, movie_clips.clone());
    }

    #[tokio::test]
    async fn test_order_by_like_movie_clips_usecase() {
        let movie_clips = vec![MovieClip::new(
            "Movie Clip Title".to_string(),
            "https://www.youtube.com/watch?v=B7OPlsdBuVc".to_string(),
            300,
            400,
            (2022, 12, 4),
        )
        .unwrap()];

        let length = 1_usize;

        let mut mock_repo = MockMovieClipRepositoryImpl::new();
        mock_repo
            .expect_order_by_like()
            .with(predicate::eq(length))
            .return_const(Ok(movie_clips.clone()));

        let cmd = movie_clip_commands::OrderByLikeLimitMovieClipCommand::new(length);
        let res_vec = movie_clip_usecases::order_by_like_movie_clips(Arc::new(mock_repo), cmd)
            .await
            .unwrap();
        assert_eq!(res_vec, movie_clips);
    }

    #[tokio::test]
    async fn test_order_by_create_data_range_movie_clips_usecase() {
        let movie_clips = vec![MovieClip::new(
            "Movie Clip Title".to_string(),
            "https://www.youtube.com/watch?v=B7OPlsdBuVc".to_string(),
            300,
            400,
            (2022, 12, 4),
        )
        .unwrap()];

        let start = Date::from_ymd(2022, 12, 4).unwrap();
        let end = Date::from_ymd(2022, 12, 6).unwrap();

        let mut mock_repo = MockMovieClipRepositoryImpl::new();
        mock_repo
            .expect_order_by_create_date_range()
            .withf(move |s, e| start == *s && end == *e)
            .return_const(Ok(movie_clips.clone()));

        let cmd = movie_clip_commands::OrderByCreateDateRangeMovieClipCommand::new(start, end);
        let res_vec =
            movie_clip_usecases::order_by_create_date_range_movie_clips(Arc::new(mock_repo), cmd)
                .await
                .unwrap();
        assert_eq!(res_vec, movie_clips);
    }

    #[tokio::test]
    async fn test_remove_movie_clip_usecase() {
        let movie_clip_id = MovieClipId::generate();

        let mut mock_repo_ok = MockMovieClipRepositoryImpl::new();
        mock_repo_ok
            .expect_remove()
            .with(predicate::eq(movie_clip_id))
            .return_const(Ok(()));

        let cmd = movie_clip_commands::RemoveByIdMovieClipCommand::new(movie_clip_id);
        let res_ok = movie_clip_usecases::remove_movie_clip(Arc::new(mock_repo_ok), cmd).await;
        assert_matches!(res_ok, Ok(_));

        let mut mock_repo_err = MockMovieClipRepositoryImpl::new();
        mock_repo_err
            .expect_remove()
            .with(predicate::eq(movie_clip_id))
            .return_const(Err(InfraError::NoRecordError));

        let cmd = movie_clip_commands::RemoveByIdMovieClipCommand::new(movie_clip_id);
        let res_err = movie_clip_usecases::remove_movie_clip(Arc::new(mock_repo_err), cmd).await;
        assert_matches!(res_err, Err(AppCommonError::NoRecordError));
    }
}
