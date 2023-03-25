#[cfg(test)]
use mockall::automock;

#[cfg_attr(test, automock)]
pub mod video_usecases {
    use crate::commands::video_commands;
    use common::AppCommonError;
    use domain::{
        video::{Video, VideoType},
        VideoRepository,
    };
    use infrastructure::InfraError;
    use std::sync::Arc;

    pub(crate) async fn save_video<T, V>(
        repo: Arc<T>,
        cmd: video_commands::SaveVideoCommand<V>,
    ) -> Result<(), AppCommonError>
    where
        T: VideoRepository<V, Error = InfraError> + 'static,
        V: VideoType + 'static,
    {
        repo.save(cmd.video).await?;
        Ok(())
    }

    pub(crate) async fn edit_video<T, V>(
        repo: Arc<T>,
        cmd: video_commands::EditVideoCommand<V>,
    ) -> Result<(), AppCommonError>
    where
        T: VideoRepository<V, Error = InfraError> + 'static,
        V: VideoType + 'static,
    {
        repo.edit(cmd.video).await?;
        Ok(())
    }

    pub(crate) async fn increment_like_video<T, V>(
        repo: Arc<T>,
        cmd: video_commands::IncrementLikeVideoCommand,
    ) -> Result<(), AppCommonError>
    where
        T: VideoRepository<V, Error = InfraError> + 'static,
        V: VideoType + 'static,
    {
        repo.increment_like(cmd.id).await?;
        Ok(())
    }

    pub(crate) async fn all_videos<T, V>(
        repo: Arc<T>,
        _cmd: video_commands::AllVideosCommand,
    ) -> Result<Vec<Video<V>>, AppCommonError>
    where
        T: VideoRepository<V, Error = InfraError> + 'static,
        V: VideoType + 'static,
    {
        let videos = repo.all().await?;
        Ok(videos)
    }

    pub(crate) async fn order_by_like_videos<T, V>(
        repo: Arc<T>,
        cmd: video_commands::OrderByLikeVideosCommand,
    ) -> Result<Vec<Video<V>>, AppCommonError>
    where
        T: VideoRepository<V, Error = InfraError> + 'static,
        V: VideoType + 'static,
    {
        let videos = repo.order_by_like(cmd.length).await?;
        Ok(videos)
    }

    pub(crate) async fn order_by_like_later_videos<T, V>(
        repo: Arc<T>,
        cmd: video_commands::OrderByLikeLaterVideosCommand<V>,
    ) -> Result<Vec<Video<V>>, AppCommonError>
    where
        T: VideoRepository<V, Error = InfraError> + 'static,
        V: VideoType + 'static,
    {
        let videos = repo.order_by_like_later(&cmd.reference, cmd.length).await?;
        Ok(videos)
    }

    pub(crate) async fn order_by_date_videos<T, V>(
        repo: Arc<T>,
        cmd: video_commands::OrderByDateVideosCommand,
    ) -> Result<Vec<Video<V>>, AppCommonError>
    where
        T: VideoRepository<V, Error = InfraError> + 'static,
        V: VideoType + 'static,
    {
        let videos = repo.order_by_date(cmd.length).await?;
        Ok(videos)
    }

    pub(crate) async fn order_by_date_later_videos<T, V>(
        repo: Arc<T>,
        cmd: video_commands::OrderByDateLaterVideosCommand<V>,
    ) -> Result<Vec<Video<V>>, AppCommonError>
    where
        T: VideoRepository<V, Error = InfraError> + 'static,
        V: VideoType + 'static,
    {
        let videos = repo.order_by_date_later(&cmd.reference, cmd.length).await?;
        Ok(videos)
    }

    pub(crate) async fn remove_video<T, V>(
        repo: Arc<T>,
        cmd: video_commands::RemoveVideoCommand,
    ) -> Result<(), AppCommonError>
    where
        T: VideoRepository<V, Error = InfraError> + 'static,
        V: VideoType + 'static,
    {
        repo.remove(cmd.id).await?;
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::video_usecases;
    use crate::commands::video_commands;
    use common::AppCommonError;
    use domain::video::{Original, Video};
    use infrastructure::{video_repository_impl::MockVideoOriginalRepository, InfraError};

    use fake::{Fake, Faker};
    use mockall::predicate;
    use pretty_assertions::assert_eq;
    use rstest::{fixture, rstest};
    use std::sync::Arc;

    #[fixture]
    fn originals() -> Vec<Video<Original>> {
        (0..100)
            .map(|_| Faker.fake::<Video<Original>>())
            .collect::<Vec<_>>()
    }

    #[tokio::test]
    async fn test_save_video_usecase() {
        let original = Faker.fake::<Video<Original>>();

        let mut mock_repo_ok = MockVideoOriginalRepository::new();
        mock_repo_ok
            .expect_save()
            .with(predicate::eq(original.clone()))
            .times(1)
            .return_const(Ok(()));

        let cmd = video_commands::SaveVideoCommand::new(original.clone());
        let res_ok = video_usecases::save_video(Arc::new(mock_repo_ok), cmd).await;
        assert!(matches!(res_ok, Ok(_)));

        let mut mock_repo_err = MockVideoOriginalRepository::new();
        mock_repo_err
            .expect_save()
            .with(predicate::eq(original.clone()))
            .times(1)
            .return_const(Err(InfraError::ConflictError));

        let cmd = video_commands::SaveVideoCommand::new(original);
        let res_err = video_usecases::save_video(Arc::new(mock_repo_err), cmd).await;
        assert!(matches!(res_err, Err(AppCommonError::ConflictError)));
    }

    #[tokio::test]
    async fn test_edit_video_usecase() {
        let original = Faker.fake::<Video<Original>>();

        let mut mock_repo_ok = MockVideoOriginalRepository::new();
        mock_repo_ok
            .expect_edit()
            .with(predicate::eq(original.clone()))
            .times(1)
            .return_const(Ok(()));

        let cmd = video_commands::EditVideoCommand::new(original.clone());
        let res_ok = video_usecases::edit_video(Arc::new(mock_repo_ok), cmd).await;
        assert!(matches!(res_ok, Ok(_)));

        let mut mock_repo_err = MockVideoOriginalRepository::new();
        mock_repo_err
            .expect_edit()
            .with(predicate::eq(original.clone()))
            .times(1)
            .return_const(Err(InfraError::NoRecordError));

        let cmd = video_commands::EditVideoCommand::new(original);
        let res_err = video_usecases::edit_video(Arc::new(mock_repo_err), cmd).await;
        assert!(matches!(res_err, Err(AppCommonError::NoRecordError)));
    }

    #[tokio::test]
    async fn test_increment_like_video_usecase() {
        let original = Faker.fake::<Video<Original>>();

        let mut mock_repo_ok = MockVideoOriginalRepository::new();
        mock_repo_ok
            .expect_increment_like()
            .with(predicate::eq(original.id()))
            .times(1)
            .return_const(Ok(()));

        let cmd = video_commands::IncrementLikeVideoCommand::new(original.id());
        let res_ok = video_usecases::increment_like_video(Arc::new(mock_repo_ok), cmd).await;
        assert!(matches!(res_ok, Ok(_)));

        let mut mock_repo_err = MockVideoOriginalRepository::new();
        mock_repo_err
            .expect_increment_like()
            .with(predicate::eq(original.id()))
            .times(1)
            .return_const(Err(InfraError::NoRecordError));

        let cmd = video_commands::IncrementLikeVideoCommand::new(original.id());
        let res_err = video_usecases::increment_like_video(Arc::new(mock_repo_err), cmd).await;
        assert!(matches!(res_err, Err(AppCommonError::NoRecordError)));
    }

    #[rstest]
    #[tokio::test]
    async fn test_all_video_usecase(originals: Vec<Video<Original>>) {
        let mut mock_repo_ok = MockVideoOriginalRepository::new();
        mock_repo_ok
            .expect_all()
            .times(1)
            .return_const(Ok(originals.clone()));

        let cmd = video_commands::AllVideosCommand::new();
        let res_ok = video_usecases::all_videos(Arc::new(mock_repo_ok), cmd).await;
        assert_eq!(res_ok.unwrap(), originals);
    }

    #[rstest]
    #[tokio::test]
    async fn test_order_by_like_video_usecase(originals: Vec<Video<Original>>) {
        let length = 100_usize;

        let mut mock_repo_ok = MockVideoOriginalRepository::new();
        mock_repo_ok
            .expect_order_by_like()
            .with(predicate::eq(length))
            .times(1)
            .return_const(Ok(originals.clone()));

        let cmd = video_commands::OrderByLikeVideosCommand::new(length);
        let res_ok = video_usecases::order_by_like_videos(Arc::new(mock_repo_ok), cmd).await;
        assert_eq!(res_ok.unwrap(), originals);
    }

    #[rstest]
    #[tokio::test]
    async fn test_order_by_like_later_video_usecase(originals: Vec<Video<Original>>) {
        let reference = Faker.fake::<Video<Original>>();

        let length = 100_usize;

        let mut mock_repo_ok = MockVideoOriginalRepository::new();
        mock_repo_ok
            .expect_order_by_like_later()
            .withf({
                let reference = reference.clone();
                move |arg_reference, arg_length| {
                    *arg_reference == reference && *arg_length == length
                }
            })
            .times(1)
            .return_const(Ok(originals.clone()));

        let cmd = video_commands::OrderByLikeLaterVideosCommand::new(reference, length);
        let res_ok = video_usecases::order_by_like_later_videos(Arc::new(mock_repo_ok), cmd).await;
        assert_eq!(res_ok.unwrap(), originals);
    }

    #[rstest]
    #[tokio::test]
    async fn test_order_by_date_video_usecase(originals: Vec<Video<Original>>) {
        let length = 100_usize;

        let mut mock_repo_ok = MockVideoOriginalRepository::new();
        mock_repo_ok
            .expect_order_by_date()
            .with(predicate::eq(length))
            .times(1)
            .return_const(Ok(originals.clone()));

        let cmd = video_commands::OrderByDateVideosCommand::new(length);
        let res_ok = video_usecases::order_by_date_videos(Arc::new(mock_repo_ok), cmd).await;
        assert_eq!(res_ok.unwrap(), originals);
    }

    #[rstest]
    #[tokio::test]
    async fn test_order_by_date_later_video_usecase(originals: Vec<Video<Original>>) {
        let reference = Faker.fake::<Video<Original>>();

        let length = 100_usize;

        let mut mock_repo_ok = MockVideoOriginalRepository::new();
        mock_repo_ok
            .expect_order_by_date_later()
            .withf({
                let reference = reference.clone();
                move |arg_reference, arg_length| {
                    *arg_reference == reference && *arg_length == length
                }
            })
            .times(1)
            .return_const(Ok(originals.clone()));

        let cmd = video_commands::OrderByDateLaterVideosCommand::new(reference, length);
        let res_ok = video_usecases::order_by_date_later_videos(Arc::new(mock_repo_ok), cmd).await;
        assert_eq!(res_ok.unwrap(), originals);
    }

    #[tokio::test]
    async fn test_remove_video_usecase() {
        let original = Faker.fake::<Video<Original>>();

        let mut mock_repo_ok = MockVideoOriginalRepository::new();
        mock_repo_ok
            .expect_remove()
            .with(predicate::eq(original.id()))
            .times(1)
            .return_const(Ok(()));

        let cmd = video_commands::RemoveVideoCommand::new(original.id());
        let res_ok = video_usecases::remove_video(Arc::new(mock_repo_ok), cmd).await;
        assert!(matches!(res_ok, Ok(_)));

        let mut mock_repo_err = MockVideoOriginalRepository::new();
        mock_repo_err
            .expect_remove()
            .with(predicate::eq(original.id()))
            .times(1)
            .return_const(Err(InfraError::NoRecordError));

        let cmd = video_commands::RemoveVideoCommand::new(original.id());
        let res_err = video_usecases::remove_video(Arc::new(mock_repo_err), cmd).await;
        assert!(matches!(res_err, Err(AppCommonError::NoRecordError)));
    }
}
