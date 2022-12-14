#[cfg(test)]
use mockall::automock;

#[cfg_attr(test, automock)]
pub mod episode_usecases {
    use crate::commands::episode_commands::{
        AllEpisodeCommand, EditEpisodeCommand, OrderByDateRangeEpisodeCommand,
        RemoveByIdEpisodeCommand, SaveEpisodeCommand,
    };
    use common::AppCommonError;
    use domain::{episode::Episode, EpisodeRepository};
    use infrastructure::InfraError;
    use std::sync::Arc;

    pub(crate) async fn save_episode<T>(
        repo: Arc<T>,
        cmd: SaveEpisodeCommand,
    ) -> Result<(), AppCommonError>
    where
        T: EpisodeRepository<Error = InfraError> + 'static,
    {
        repo.save(cmd.episode).await?;
        Ok(())
    }

    pub(crate) async fn edit_episode<T>(
        repo: Arc<T>,
        cmd: EditEpisodeCommand,
    ) -> Result<(), AppCommonError>
    where
        T: EpisodeRepository<Error = InfraError> + 'static,
    {
        repo.edit(cmd.episode).await?;
        Ok(())
    }

    pub(crate) async fn all_episodes<T>(
        repo: Arc<T>,
        _cmd: AllEpisodeCommand,
    ) -> Result<Vec<Episode>, AppCommonError>
    where
        T: EpisodeRepository<Error = InfraError> + 'static,
    {
        Ok(repo.all().await?)
    }

    pub(crate) async fn order_by_date_range_episodes<T>(
        repo: Arc<T>,
        cmd: OrderByDateRangeEpisodeCommand,
    ) -> Result<Vec<Episode>, AppCommonError>
    where
        T: EpisodeRepository<Error = InfraError> + 'static,
    {
        Ok(repo.order_by_date_range(cmd.start, cmd.end).await?)
    }

    pub(crate) async fn remove_by_id_episode<T>(
        repo: Arc<T>,
        cmd: RemoveByIdEpisodeCommand,
    ) -> Result<(), AppCommonError>
    where
        T: EpisodeRepository<Error = InfraError> + 'static,
    {
        Ok(repo.remove_by_id(cmd.id).await?)
    }
}

#[cfg(test)]
mod test {
    use super::episode_usecases;
    use crate::commands::episode_commands;
    use assert_matches::assert_matches;
    use common::AppCommonError;
    use domain::{
        episode::{Episode, EpisodeId},
        Date,
    };
    use infrastructure::episode_repository_impl::MockEpisodeRepositoryImpl;
    use infrastructure::InfraError;
    use mockall::predicate;
    use pretty_assertions::assert_eq;
    use std::sync::Arc;

    #[tokio::test]
    async fn test_save_episode_usecase() {
        let episode = Episode::new((2022, 12, 4), "Some Contents".to_string()).unwrap();

        let mut mock_repo_ok = MockEpisodeRepositoryImpl::new();
        mock_repo_ok
            .expect_save()
            .with(predicate::eq(episode.clone()))
            .times(1)
            .return_const(Ok(()));

        let cmd = episode_commands::SaveEpisodeCommand::new(episode.clone());
        let res_ok = episode_usecases::save_episode(Arc::new(mock_repo_ok), cmd).await;
        assert_matches!(res_ok, Ok(_));

        let mut mock_repo_err = MockEpisodeRepositoryImpl::new();
        mock_repo_err
            .expect_save()
            .with(predicate::eq(episode.clone()))
            .times(1)
            .return_const(Err(InfraError::ConflictError));

        let cmd = episode_commands::SaveEpisodeCommand::new(episode.clone());
        let res_err = episode_usecases::save_episode(Arc::new(mock_repo_err), cmd).await;
        assert_matches!(res_err, Err(AppCommonError::ConflictError));
    }

    #[tokio::test]
    async fn test_edit_episode_usecase() {
        let episode = Episode::new((2022, 12, 4), "Some Contents".to_string()).unwrap();
        let mut mock_repo_ok = MockEpisodeRepositoryImpl::new();
        mock_repo_ok
            .expect_edit()
            .with(predicate::eq(episode.clone()))
            .times(1)
            .return_const(Ok(()));

        let cmd = episode_commands::EditEpisodeCommand::new(episode.clone());
        let res_ok = episode_usecases::edit_episode(Arc::new(mock_repo_ok), cmd).await;
        assert_matches!(res_ok, Ok(_));

        let mut mock_repo_err = MockEpisodeRepositoryImpl::new();
        mock_repo_err
            .expect_edit()
            .with(predicate::eq(episode.clone()))
            .times(1)
            .return_const(Err(InfraError::NoRecordError));

        let cmd = episode_commands::EditEpisodeCommand::new(episode.clone());
        let res_err = episode_usecases::edit_episode(Arc::new(mock_repo_err), cmd).await;
        assert_matches!(res_err, Err(AppCommonError::NoRecordError));
    }

    #[tokio::test]
    async fn test_all_episodes_usecase() {
        let episodes = vec![Episode::new((2022, 12, 4), "Some Contents".to_string()).unwrap()];
        let mut mock_repo = MockEpisodeRepositoryImpl::new();
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

    #[tokio::test]
    async fn test_order_by_date_range_episodes_usecase() {
        let episodes = vec![Episode::new((2022, 12, 4), "Some Contents".to_string()).unwrap()];
        let start = Date::from_ymd(2022, 12, 4).unwrap();
        let end = Date::from_ymd(2022, 12, 6).unwrap();

        let mut mock_repo = MockEpisodeRepositoryImpl::new();
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
    async fn test_remove_by_id_episode_usecase() {
        let episode_id = EpisodeId::generate();

        let mut mock_repo_ok = MockEpisodeRepositoryImpl::new();
        mock_repo_ok
            .expect_remove_by_id()
            .with(predicate::eq(episode_id))
            .return_const(Ok(()));

        let cmd = episode_commands::RemoveByIdEpisodeCommand::new(episode_id);
        let res_ok = episode_usecases::remove_by_id_episode(Arc::new(mock_repo_ok), cmd).await;
        assert_matches!(res_ok, Ok(_));

        let mut mock_repo_err = MockEpisodeRepositoryImpl::new();
        mock_repo_err
            .expect_remove_by_id()
            .with(predicate::eq(episode_id))
            .return_const(Err(InfraError::NoRecordError));

        let cmd = episode_commands::RemoveByIdEpisodeCommand::new(episode_id);
        let res_err = episode_usecases::remove_by_id_episode(Arc::new(mock_repo_err), cmd).await;
        assert_matches!(res_err, Err(AppCommonError::NoRecordError));
    }
}
