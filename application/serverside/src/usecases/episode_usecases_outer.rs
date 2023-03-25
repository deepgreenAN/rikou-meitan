#[cfg(test)]
use mockall::automock;

#[cfg_attr(test, automock)]
pub mod episode_usecases {
    use crate::commands::episode_commands;
    use common::AppCommonError;
    use domain::{episode::Episode, EpisodeRepository};
    use infrastructure::InfraError;
    use std::sync::Arc;

    pub(crate) async fn save_episode<T>(
        repo: Arc<T>,
        cmd: episode_commands::SaveEpisodeCommand,
    ) -> Result<(), AppCommonError>
    where
        T: EpisodeRepository<Error = InfraError> + 'static,
    {
        repo.save(cmd.episode).await?;
        Ok(())
    }

    pub(crate) async fn edit_episode<T>(
        repo: Arc<T>,
        cmd: episode_commands::EditEpisodeCommand,
    ) -> Result<(), AppCommonError>
    where
        T: EpisodeRepository<Error = InfraError> + 'static,
    {
        repo.edit(cmd.episode).await?;
        Ok(())
    }

    pub(crate) async fn all_episodes<T>(
        repo: Arc<T>,
        _cmd: episode_commands::AllEpisodeCommand,
    ) -> Result<Vec<Episode>, AppCommonError>
    where
        T: EpisodeRepository<Error = InfraError> + 'static,
    {
        Ok(repo.all().await?)
    }

    pub(crate) async fn order_by_date_range_episodes<T>(
        repo: Arc<T>,
        cmd: episode_commands::OrderByDateRangeEpisodeCommand,
    ) -> Result<Vec<Episode>, AppCommonError>
    where
        T: EpisodeRepository<Error = InfraError> + 'static,
    {
        Ok(repo.order_by_date_range(cmd.start, cmd.end).await?)
    }

    pub(crate) async fn remove_episode<T>(
        repo: Arc<T>,
        cmd: episode_commands::RemoveEpisodeCommand,
    ) -> Result<(), AppCommonError>
    where
        T: EpisodeRepository<Error = InfraError> + 'static,
    {
        Ok(repo.remove(cmd.id).await?)
    }
}

#[cfg(test)]
mod test {
    use super::episode_usecases;
    use crate::commands::episode_commands;
    use common::AppCommonError;
    use domain::{
        episode::{Episode, EpisodeId},
        Date,
    };
    use fake::{Fake, Faker};
    use infrastructure::episode_repository_impl::MockEpisodeRepository;
    use infrastructure::InfraError;
    use mockall::predicate;
    use pretty_assertions::assert_eq;
    use rstest::{fixture, rstest};
    use std::sync::Arc;

    #[fixture]
    fn episodes() -> Vec<Episode> {
        (0..100)
            .map(|_| Faker.fake::<Episode>())
            .collect::<Vec<_>>()
    }

    #[tokio::test]
    async fn test_save_episode_usecase() {
        let episode = Faker.fake::<Episode>();
        {
            let mut mock_repo_ok = MockEpisodeRepository::new();
            mock_repo_ok
                .expect_save()
                .with(predicate::eq(episode.clone()))
                .times(1)
                .return_const(Ok(()));

            let cmd = episode_commands::SaveEpisodeCommand::new(episode.clone());
            let res_ok = episode_usecases::save_episode(Arc::new(mock_repo_ok), cmd).await;
            assert!(matches!(res_ok, Ok(_)));
        }
        {
            let mut mock_repo_err = MockEpisodeRepository::new();
            mock_repo_err
                .expect_save()
                .with(predicate::eq(episode.clone()))
                .times(1)
                .return_const(Err(InfraError::ConflictError));

            let cmd = episode_commands::SaveEpisodeCommand::new(episode.clone());
            let res_err = episode_usecases::save_episode(Arc::new(mock_repo_err), cmd).await;
            assert!(matches!(res_err, Err(AppCommonError::ConflictError)));
        }
    }

    #[tokio::test]
    async fn test_edit_episode_usecase() {
        let episode = Faker.fake::<Episode>();
        {
            let mut mock_repo_ok = MockEpisodeRepository::new();
            mock_repo_ok
                .expect_edit()
                .with(predicate::eq(episode.clone()))
                .times(1)
                .return_const(Ok(()));

            let cmd = episode_commands::EditEpisodeCommand::new(episode.clone());
            let res_ok = episode_usecases::edit_episode(Arc::new(mock_repo_ok), cmd).await;
            assert!(matches!(res_ok, Ok(_)));
        }
        {
            let mut mock_repo_err = MockEpisodeRepository::new();
            mock_repo_err
                .expect_edit()
                .with(predicate::eq(episode.clone()))
                .times(1)
                .return_const(Err(InfraError::NoRecordError));

            let cmd = episode_commands::EditEpisodeCommand::new(episode.clone());
            let res_err = episode_usecases::edit_episode(Arc::new(mock_repo_err), cmd).await;
            assert!(matches!(res_err, Err(AppCommonError::NoRecordError)));
        }
    }

    #[rstest]
    #[tokio::test]
    async fn test_all_episodes_usecase(episodes: Vec<Episode>) {
        let mut mock_repo = MockEpisodeRepository::new();
        mock_repo
            .expect_all()
            .times(1)
            .return_const(Ok(episodes.clone()));

        let cmd = episode_commands::AllEpisodeCommand;
        let res_episodes = episode_usecases::all_episodes(Arc::new(mock_repo), cmd)
            .await
            .unwrap();
        assert_eq!(res_episodes, episodes);
    }

    #[rstest]
    #[tokio::test]
    async fn test_order_by_date_range_episodes_usecase(episodes: Vec<Episode>) {
        let start = Faker.fake::<Date>();
        let end = Faker.fake::<Date>();

        let mut mock_repo = MockEpisodeRepository::new();
        mock_repo
            .expect_order_by_date_range()
            .withf(move |s, e| start == *s && end == *e)
            .times(1)
            .return_const(Ok(episodes.clone()));

        let cmd = episode_commands::OrderByDateRangeEpisodeCommand::new(start, end);
        let res_vec = episode_usecases::order_by_date_range_episodes(Arc::new(mock_repo), cmd)
            .await
            .unwrap();
        assert_eq!(res_vec, episodes);
    }

    #[tokio::test]
    async fn test_remove_episode_usecase() {
        let episode_id = EpisodeId::generate();

        {
            let mut mock_repo_ok = MockEpisodeRepository::new();
            mock_repo_ok
                .expect_remove()
                .with(predicate::eq(episode_id))
                .return_const(Ok(()));

            let cmd = episode_commands::RemoveEpisodeCommand::new(episode_id);
            let res_ok = episode_usecases::remove_episode(Arc::new(mock_repo_ok), cmd).await;
            assert!(matches!(res_ok, Ok(_)));
        }
        {
            let mut mock_repo_err = MockEpisodeRepository::new();
            mock_repo_err
                .expect_remove()
                .with(predicate::eq(episode_id))
                .return_const(Err(InfraError::NoRecordError));

            let cmd = episode_commands::RemoveEpisodeCommand::new(episode_id);
            let res_err = episode_usecases::remove_episode(Arc::new(mock_repo_err), cmd).await;
            assert!(matches!(res_err, Err(AppCommonError::NoRecordError)));
        }
    }
}
