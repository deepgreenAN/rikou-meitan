use crate::commands::episode_commands::{
    AllEpisodeCommand, EditEpisodeCommand, OrderByDateRangeEpisodeCommand,
    RemoveByIdEpisodeCommand, SaveEpisodeCommand,
};
use common::AppCommonError;
use domain::{episode::Episode, EpisodeRepository};
use infrastructure::InfraError;
use std::sync::Arc;

async fn save_episode_usecase<T>(
    repo: Arc<T>,
    cmd: SaveEpisodeCommand,
) -> Result<(), AppCommonError>
where
    T: EpisodeRepository<Error = InfraError>,
{
    repo.save(cmd.episode).await?;
    Ok(())
}

async fn edit_episode_usecase<T>(
    repo: Arc<T>,
    cmd: EditEpisodeCommand,
) -> Result<(), AppCommonError>
where
    T: EpisodeRepository<Error = InfraError>,
{
    repo.edit(cmd.episode).await?;
    Ok(())
}

async fn all_episodes_usecase<T>(
    repo: Arc<T>,
    _cmd: AllEpisodeCommand,
) -> Result<Vec<Episode>, AppCommonError>
where
    T: EpisodeRepository<Error = InfraError>,
{
    Ok(repo.all().await?)
}

async fn order_by_date_range_episodes_usecase<T>(
    repo: Arc<T>,
    cmd: OrderByDateRangeEpisodeCommand,
) -> Result<Vec<Episode>, AppCommonError>
where
    T: EpisodeRepository<Error = InfraError>,
{
    Ok(repo.order_by_date_range(cmd.start, cmd.end).await?)
}

async fn remove_by_id_episode_usecase<T>(
    repo: Arc<T>,
    cmd: RemoveByIdEpisodeCommand,
) -> Result<(), AppCommonError>
where
    T: EpisodeRepository<Error = InfraError>,
{
    Ok(repo.remove_by_id(cmd.id).await?)
}

#[cfg(test)]
mod test {
    use assert_matches::assert_matches;
    use common::AppCommonError;
    use domain::{
        episode::{Episode, EpisodeId},
        Date,
    };
    use infrastructure::MockEpisodeRepository;
    use pretty_assertions::assert_eq;
    use rstest::{fixture, rstest};
    use std::sync::Arc;

    use super::remove_by_id_episode_usecase;

    #[fixture]
    async fn episodes_and_saved_repo(
    ) -> Result<(Vec<Episode>, Arc<MockEpisodeRepository>), AppCommonError> {
        let episodes = vec![
            Episode::new((2022, 11, 21), "Some Episode Content1".to_string())?,
            Episode::new((2022, 11, 19), "Some Episode Content2".to_string())?,
            Episode::new((2022, 11, 22), "Some Episode Content3".to_string())?,
        ];

        let repo = Arc::new(MockEpisodeRepository::new());

        for episode in episodes.iter().cloned() {
            let cmd = super::SaveEpisodeCommand::new(episode);
            super::save_episode_usecase(Arc::clone(&repo), cmd).await?;
        }
        Ok((episodes, Arc::clone(&repo)))
    }

    #[rstest]
    #[tokio::test]
    async fn test_edit_episode_usecase(
        #[future] episodes_and_saved_repo: Result<
            (Vec<Episode>, Arc<MockEpisodeRepository>),
            AppCommonError,
        >,
    ) -> Result<(), AppCommonError> {
        let (mut episodes, repo) = episodes_and_saved_repo.await?;

        let mut edited_episode = episodes[0].clone(); // 一番目を編集
        edited_episode.edit_content("New Content".to_string())?;
        episodes[0] = edited_episode.clone();

        let cmd = super::EditEpisodeCommand::new(edited_episode.clone());
        super::edit_episode_usecase(Arc::clone(&repo), cmd).await?;

        let cmd = super::AllEpisodeCommand;
        let mut res_all_episodes = super::all_episodes_usecase(Arc::clone(&repo), cmd).await?;

        res_all_episodes.sort_by_key(|episode| episode.id());
        episodes.sort_by_key(|episode| episode.id());

        assert_eq!(episodes, res_all_episodes);

        Ok(())
    }

    #[rstest]
    #[tokio::test]
    async fn test_all_episodes_usecase(
        #[future] episodes_and_saved_repo: Result<
            (Vec<Episode>, Arc<MockEpisodeRepository>),
            AppCommonError,
        >,
    ) -> Result<(), AppCommonError> {
        let (mut episodes, repo) = episodes_and_saved_repo.await?;

        let cmd = super::AllEpisodeCommand;
        let mut res_all_episodes = super::all_episodes_usecase(repo, cmd).await?;

        res_all_episodes.sort_by_key(|episode| episode.id());
        episodes.sort_by_key(|episode| episode.id());

        assert_eq!(episodes, res_all_episodes);
        Ok(())
    }

    #[rstest]
    #[tokio::test]
    async fn test_order_by_date_range_episodes_usecase(
        #[future] episodes_and_saved_repo: Result<
            (Vec<Episode>, Arc<MockEpisodeRepository>),
            AppCommonError,
        >,
    ) -> Result<(), AppCommonError> {
        let (mut episodes, repo) = episodes_and_saved_repo.await?;

        let start = Date::from_ymd(2022, 11, 19)?;
        let end = Date::from_ymd(2022, 11, 22)?;
        let cmd = super::OrderByDateRangeEpisodeCommand::new(start, end);

        let res_episodes =
            super::order_by_date_range_episodes_usecase(Arc::clone(&repo), cmd).await?;

        episodes.sort_by_key(|episode| episode.date());
        let episodes = episodes
            .into_iter()
            .filter(|episode| start <= episode.date() && episode.date() < end)
            .collect::<Vec<_>>();

        assert_eq!(episodes, res_episodes);
        Ok(())
    }

    #[rstest]
    #[tokio::test]
    async fn test_remove_by_id_episode_usecase(
        #[future] episodes_and_saved_repo: Result<
            (Vec<Episode>, Arc<MockEpisodeRepository>),
            AppCommonError,
        >,
    ) -> Result<(), AppCommonError> {
        let (mut episodes, repo) = episodes_and_saved_repo.await?;

        let removed_episode = episodes.remove(1); // 二番目を削除
        let cmd = super::RemoveByIdEpisodeCommand::new(removed_episode.id());
        super::remove_by_id_episode_usecase(Arc::clone(&repo), cmd).await?;

        let cmd = super::AllEpisodeCommand;
        let mut res_episodes = super::all_episodes_usecase(Arc::clone(&repo), cmd).await?;

        res_episodes.sort_by_key(|episode| episode.id());
        episodes.sort_by_key(|episode| episode.id());

        assert_eq!(episodes, res_episodes);
        Ok(())
    }

    #[rstest]
    #[tokio::test]
    async fn test_edit_episode_usecases_not_exists() -> Result<(), AppCommonError> {
        let episode = Episode::new((2022, 11, 21), "Some Episode Content1".to_string())?;
        let repo = Arc::new(MockEpisodeRepository::new());

        let cmd = super::EditEpisodeCommand::new(episode);
        let res = super::edit_episode_usecase(Arc::clone(&repo), cmd).await;

        assert_matches!(res, Err(AppCommonError::RemovedRecordError));
        Ok(())
    }

    #[rstest]
    #[tokio::test]
    async fn test_remove_by_id_episode_usecases_not_exists() -> Result<(), AppCommonError> {
        let repo = Arc::new(MockEpisodeRepository::new());

        let cmd = super::RemoveByIdEpisodeCommand::new(EpisodeId::generate());
        let res = super::remove_by_id_episode_usecase(Arc::clone(&repo), cmd).await;

        assert_matches!(res, Err(AppCommonError::RemovedRecordError));
        Ok(())
    }
}
