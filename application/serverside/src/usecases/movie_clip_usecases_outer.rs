#[cfg(test)]
use mockall::automock;

#[cfg_attr(test, automock)]
pub mod movie_clip_usecases {
    use crate::commands::movie_clip_commands;
    use common::AppCommonError;
    use domain::{movie_clip::MovieClip, MovieClipRepository};
    use infrastructure::InfraError;
    use std::sync::Arc;

    pub(crate) async fn save_movie_clip<T>(
        repo: Arc<T>,
        cmd: movie_clip_commands::SaveMovieClipCommand,
    ) -> Result<(), AppCommonError>
    where
        T: MovieClipRepository<Error = InfraError> + 'static,
    {
        repo.save(cmd.movie_clip).await?;
        Ok(())
    }

    pub(crate) async fn edit_movie_clip<T>(
        repo: Arc<T>,
        cmd: movie_clip_commands::EditMovieClipCommand,
    ) -> Result<(), AppCommonError>
    where
        T: MovieClipRepository<Error = InfraError> + 'static,
    {
        repo.edit(cmd.movie_clip).await?;
        Ok(())
    }

    pub(crate) async fn increment_like_movie_clip<T>(
        repo: Arc<T>,
        cmd: movie_clip_commands::IncrementLikeMovieClipCommand,
    ) -> Result<(), AppCommonError>
    where
        T: MovieClipRepository<Error = InfraError> + 'static,
    {
        repo.increment_like(cmd.id).await?;
        Ok(())
    }

    pub(crate) async fn all_movie_clips<T>(
        repo: Arc<T>,
        _cmd: movie_clip_commands::AllMovieClipCommand,
    ) -> Result<Vec<MovieClip>, AppCommonError>
    where
        T: MovieClipRepository<Error = InfraError> + 'static,
    {
        Ok(repo.all().await?)
    }

    pub(crate) async fn order_by_like_movie_clips<T>(
        repo: Arc<T>,
        cmd: movie_clip_commands::OrderByLikeMovieClipCommand,
    ) -> Result<Vec<MovieClip>, AppCommonError>
    where
        T: MovieClipRepository<Error = InfraError> + 'static,
    {
        Ok(repo.order_by_like(cmd.length).await?)
    }

    pub(crate) async fn order_by_like_later_movie_clips<T>(
        repo: Arc<T>,
        cmd: movie_clip_commands::OrderByLikeLaterMovieClipCommand,
    ) -> Result<Vec<MovieClip>, AppCommonError>
    where
        T: MovieClipRepository<Error = InfraError> + 'static,
    {
        Ok(repo.order_by_like_later(&cmd.reference, cmd.length).await?)
    }

    pub(crate) async fn order_by_create_date_range_movie_clips<T>(
        repo: Arc<T>,
        cmd: movie_clip_commands::OrderByCreateDateRangeMovieClipCommand,
    ) -> Result<Vec<MovieClip>, AppCommonError>
    where
        T: MovieClipRepository<Error = InfraError> + 'static,
    {
        Ok(repo.order_by_create_date_range(cmd.start, cmd.end).await?)
    }

    pub(crate) async fn order_by_create_date_movie_clips<T>(
        repo: Arc<T>,
        cmd: movie_clip_commands::OrderByCreateDateMovieClipCommand,
    ) -> Result<Vec<MovieClip>, AppCommonError>
    where
        T: MovieClipRepository<Error = InfraError> + 'static,
    {
        Ok(repo.order_by_create_date(cmd.length).await?)
    }

    pub(crate) async fn order_by_create_date_later_movie_clips<T>(
        repo: Arc<T>,
        cmd: movie_clip_commands::OrderByCreateDateLaterMovieClipCommand,
    ) -> Result<Vec<MovieClip>, AppCommonError>
    where
        T: MovieClipRepository<Error = InfraError> + 'static,
    {
        Ok(repo
            .order_by_create_date_later(&cmd.reference, cmd.length)
            .await?)
    }

    pub(crate) async fn remove_movie_clip<T>(
        repo: Arc<T>,
        cmd: movie_clip_commands::RemoveMovieClipCommand,
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
    use common::AppCommonError;
    use domain::{
        movie_clip::{MovieClip, MovieClipId},
        Date,
    };
    use infrastructure::movie_clip_repository_impl::MockMovieClipRepository;
    use infrastructure::InfraError;

    use fake::{Fake, Faker};
    use mockall::predicate;
    use pretty_assertions::assert_eq;
    use std::sync::Arc;

    #[tokio::test]
    async fn test_save_movie_clip_usecase() {
        let movie_clip = Faker.fake::<MovieClip>();

        {
            let mut mock_repo_ok = MockMovieClipRepository::new();
            mock_repo_ok
                .expect_save()
                .with(predicate::eq(movie_clip.clone()))
                .times(1)
                .return_const(Ok(()));

            let cmd = movie_clip_commands::SaveMovieClipCommand::new(movie_clip.clone());
            let res_ok = movie_clip_usecases::save_movie_clip(Arc::new(mock_repo_ok), cmd).await;
            assert!(matches!(res_ok, Ok(_)));
        }
        {
            let mut mock_repo_err = MockMovieClipRepository::new();
            mock_repo_err
                .expect_save()
                .with(predicate::eq(movie_clip.clone()))
                .times(1)
                .return_const(Err(InfraError::ConflictError));

            let cmd = movie_clip_commands::SaveMovieClipCommand::new(movie_clip.clone());
            let res_err = movie_clip_usecases::save_movie_clip(Arc::new(mock_repo_err), cmd).await;
            assert!(matches!(res_err, Err(AppCommonError::ConflictError)));
        }
    }

    #[tokio::test]
    async fn test_edit_movie_clip_usecase() {
        let movie_clip = Faker.fake::<MovieClip>();

        {
            let mut mock_repo_ok = MockMovieClipRepository::new();
            mock_repo_ok
                .expect_edit()
                .with(predicate::eq(movie_clip.clone()))
                .times(1)
                .return_const(Ok(()));

            let cmd = movie_clip_commands::EditMovieClipCommand::new(movie_clip.clone());
            let res_ok = movie_clip_usecases::edit_movie_clip(Arc::new(mock_repo_ok), cmd).await;
            assert!(matches!(res_ok, Ok(_)));
        }
        {
            let mut mock_repo_err = MockMovieClipRepository::new();
            mock_repo_err
                .expect_edit()
                .with(predicate::eq(movie_clip.clone()))
                .times(1)
                .return_const(Err(InfraError::NoRecordError));

            let cmd = movie_clip_commands::EditMovieClipCommand::new(movie_clip.clone());
            let res_err = movie_clip_usecases::edit_movie_clip(Arc::new(mock_repo_err), cmd).await;
            assert!(matches!(res_err, Err(AppCommonError::NoRecordError)));
        }
    }

    #[tokio::test]
    async fn test_increment_like_movie_clip_usecase() {
        let id = MovieClipId::generate();

        {
            let mut mock_repo_ok = MockMovieClipRepository::new();
            mock_repo_ok
                .expect_increment_like()
                .with(predicate::eq(id))
                .times(1)
                .return_const(Ok(()));

            let cmd = movie_clip_commands::IncrementLikeMovieClipCommand::new(id);
            let res_ok =
                movie_clip_usecases::increment_like_movie_clip(Arc::new(mock_repo_ok), cmd).await;
            assert!(matches!(res_ok, Ok(_)));
        }
        {
            let mut mock_repo_err = MockMovieClipRepository::new();
            mock_repo_err
                .expect_increment_like()
                .with(predicate::eq(id))
                .times(1)
                .return_const(Err(InfraError::NoRecordError));

            let cmd = movie_clip_commands::IncrementLikeMovieClipCommand::new(id);
            let res_err =
                movie_clip_usecases::increment_like_movie_clip(Arc::new(mock_repo_err), cmd).await;
            assert!(matches!(res_err, Err(AppCommonError::NoRecordError)));
        }
    }

    #[tokio::test]
    async fn test_all_movie_clips_usecase() {
        let movie_clips = vec![Faker.fake::<MovieClip>(); 100];

        let mut mock_repo = MockMovieClipRepository::new();
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
        let clips = vec![Faker.fake::<MovieClip>(); 100];

        let length = 100_usize;

        let mut mock_repo = MockMovieClipRepository::new();
        mock_repo
            .expect_order_by_like()
            .with(predicate::eq(length))
            .return_const(Ok(clips.clone()));

        let cmd = movie_clip_commands::OrderByLikeMovieClipCommand::new(length);
        let res_vec = movie_clip_usecases::order_by_like_movie_clips(Arc::new(mock_repo), cmd)
            .await
            .unwrap();
        assert_eq!(res_vec, clips);
    }

    #[tokio::test]
    async fn test_order_by_like_later_movie_clips_usecase() {
        let reference = Faker.fake::<MovieClip>();
        let length = 100_usize;

        let clips = vec![Faker.fake::<MovieClip>(); 100];

        let mut mock_repo = MockMovieClipRepository::new();
        mock_repo
            .expect_order_by_like_later()
            .withf({
                let reference = reference.clone();
                move |arg_reference, arg_length| {
                    *arg_reference == reference && *arg_length == length
                }
            })
            .return_const(Ok(clips.clone()));

        let cmd = movie_clip_commands::OrderByLikeLaterMovieClipCommand::new(reference, length);
        let res_vec =
            movie_clip_usecases::order_by_like_later_movie_clips(Arc::new(mock_repo), cmd)
                .await
                .unwrap();
        assert_eq!(res_vec, clips);
    }

    #[tokio::test]
    async fn test_order_by_create_data_range_movie_clips_usecase() {
        let clips = vec![Faker.fake::<MovieClip>(); 100];

        let start = Faker.fake::<Date>();
        let end = Faker.fake::<Date>();

        let mut mock_repo = MockMovieClipRepository::new();
        mock_repo
            .expect_order_by_create_date_range()
            .withf(move |arg_start, arg_end| start == *arg_start && end == *arg_end)
            .return_const(Ok(clips.clone()));

        let cmd = movie_clip_commands::OrderByCreateDateRangeMovieClipCommand::new(start, end);
        let res_vec =
            movie_clip_usecases::order_by_create_date_range_movie_clips(Arc::new(mock_repo), cmd)
                .await
                .unwrap();
        assert_eq!(res_vec, clips);
    }

    #[tokio::test]
    async fn test_order_by_create_date_movie_clips_usecase() {
        let clips = vec![Faker.fake::<MovieClip>(); 100];

        let length = 100_usize;

        let mut mock_repo = MockMovieClipRepository::new();
        mock_repo
            .expect_order_by_create_date()
            .with(predicate::eq(length))
            .return_const(Ok(clips.clone()));

        let cmd = movie_clip_commands::OrderByCreateDateMovieClipCommand::new(length);
        let res_vec =
            movie_clip_usecases::order_by_create_date_movie_clips(Arc::new(mock_repo), cmd)
                .await
                .unwrap();
        assert_eq!(res_vec, clips);
    }

    #[tokio::test]
    async fn test_order_by_create_date_later_movie_clips_usecase() {
        let reference = Faker.fake::<MovieClip>();
        let length = 100_usize;

        let clips = vec![Faker.fake::<MovieClip>(); 100];

        let mut mock_repo = MockMovieClipRepository::new();
        mock_repo
            .expect_order_by_create_date_later()
            .withf({
                let reference = reference.clone();
                move |arg_reference, arg_length| {
                    *arg_reference == reference && *arg_length == length
                }
            })
            .return_const(Ok(clips.clone()));

        let cmd =
            movie_clip_commands::OrderByCreateDateLaterMovieClipCommand::new(reference, length);
        let res_vec =
            movie_clip_usecases::order_by_create_date_later_movie_clips(Arc::new(mock_repo), cmd)
                .await
                .unwrap();
        assert_eq!(res_vec, clips);
    }

    #[tokio::test]
    async fn test_remove_movie_clip_usecase() {
        let id = MovieClipId::generate();

        {
            let mut mock_repo_ok = MockMovieClipRepository::new();
            mock_repo_ok
                .expect_remove()
                .with(predicate::eq(id))
                .return_const(Ok(()));

            let cmd = movie_clip_commands::RemoveMovieClipCommand::new(id);
            let res_ok = movie_clip_usecases::remove_movie_clip(Arc::new(mock_repo_ok), cmd).await;
            assert!(matches!(res_ok, Ok(_)));
        }
        {
            let mut mock_repo_err = MockMovieClipRepository::new();
            mock_repo_err
                .expect_remove()
                .with(predicate::eq(id))
                .return_const(Err(InfraError::NoRecordError));

            let cmd = movie_clip_commands::RemoveMovieClipCommand::new(id);
            let res_err =
                movie_clip_usecases::remove_movie_clip(Arc::new(mock_repo_err), cmd).await;
            assert!(matches!(res_err, Err(AppCommonError::NoRecordError)));
        }
    }
}
