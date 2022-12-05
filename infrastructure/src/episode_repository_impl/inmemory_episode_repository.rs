use crate::InfraError;
use async_trait::async_trait;
use domain::episode::{Episode, EpisodeId};
use domain::Date;
use domain::EpisodeRepository;

use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use uuid::Uuid;

// -------------------------------------------------------------------------------------------------
// # MockEpisodeRepository

/// 即席のEpisodeリポジトリ
#[derive(Default)]
pub struct InMemoryEpisodeRepository {
    map: Arc<Mutex<HashMap<Uuid, Episode>>>,
}

impl InMemoryEpisodeRepository {
    pub fn new() -> Self {
        Self {
            map: Arc::new(Mutex::new(HashMap::new())),
        }
    }
}

#[async_trait]
impl EpisodeRepository for InMemoryEpisodeRepository {
    type Error = InfraError;
    async fn save(&self, episode: Episode) -> Result<(), InfraError> {
        self.map
            .lock()
            .unwrap()
            .insert(episode.id().to_uuid(), episode);
        Ok(())
    }
    async fn edit(&self, episode: Episode) -> Result<(), InfraError> {
        match self.map.lock().unwrap().entry(episode.id().to_uuid()) {
            Entry::Vacant(_) => Err(InfraError::RemovedRecordError),
            Entry::Occupied(mut o) => {
                *o.get_mut() = episode;
                Ok(())
            }
        }
    }
    async fn all(&self) -> Result<Vec<Episode>, InfraError> {
        let episodes = self
            .map
            .lock()
            .unwrap()
            .values()
            .cloned()
            .collect::<Vec<_>>();
        Ok(episodes)
    }
    async fn order_by_date_range(
        &self,
        start: Date,
        end: Date,
    ) -> Result<Vec<Episode>, InfraError> {
        let mut episodes = self.all().await?;
        episodes.sort_by_key(|episode| episode.date());

        let episodes = episodes
            .into_iter()
            .filter(|episode| start <= episode.date() && episode.date() < end)
            .collect::<Vec<_>>();
        Ok(episodes)
    }
    async fn remove_by_id(&self, id: EpisodeId) -> Result<(), InfraError> {
        match self.map.lock().unwrap().remove(&id.to_uuid()) {
            None => Err(InfraError::RemovedRecordError),
            Some(_) => Ok(()),
        }
    }
}

#[cfg(test)]
mod test {
    use super::InMemoryEpisodeRepository;
    use crate::InfraError;
    use assert_matches::assert_matches;
    use domain::Date;
    use domain::{episode::Episode, EpisodeRepository};
    use pretty_assertions::assert_eq;
    use rstest::{fixture, rstest};

    #[fixture]
    fn episodes() -> Result<Vec<Episode>, InfraError> {
        Ok(vec![
            Episode::new((2022, 11, 21), "Some Episode Content1".to_string())?,
            Episode::new((2022, 11, 19), "Some Episode Content2".to_string())?,
            Episode::new((2022, 11, 22), "Some Episode Content3".to_string())?,
        ])
    }

    #[rstest]
    #[tokio::test]
    async fn test_episode_save_and_all(
        episodes: Result<Vec<Episode>, InfraError>,
    ) -> Result<(), InfraError> {
        let mut episodes = episodes?;

        let repo = InMemoryEpisodeRepository::new();
        for episode in episodes.iter().cloned() {
            repo.save(episode).await?;
        }

        let mut episodes_res = repo.all().await?;
        episodes_res.sort_by_key(|episode| episode.id());

        episodes.sort_by_key(|episode| episode.id());

        assert_eq!(episodes, episodes_res);
        Ok(())
    }

    #[rstest]
    #[tokio::test]
    async fn test_episode_save_and_edit_and_all(
        episodes: Result<Vec<Episode>, InfraError>,
    ) -> Result<(), InfraError> {
        let mut episodes = episodes?;

        let repo = InMemoryEpisodeRepository::new();
        for episode in episodes.iter().cloned() {
            repo.save(episode).await?;
        }

        let mut edited_episode = episodes[1].clone();
        edited_episode.edit_date(Date::from_ymd(2022, 11, 23)?)?;
        edited_episode.edit_content("Another Episode Content".to_string())?;
        episodes[1] = edited_episode.clone();

        repo.edit(edited_episode).await?;

        let mut episodes_res = repo.all().await?;
        episodes_res.sort_by_key(|episode| episode.id());

        episodes.sort_by_key(|episode| episode.id());

        assert_eq!(episodes, episodes_res);
        Ok(())
    }

    #[rstest]
    #[tokio::test]
    async fn test_episode_save_and_order_by_date_range(
        episodes: Result<Vec<Episode>, InfraError>,
    ) -> Result<(), InfraError> {
        let mut episodes = episodes?;

        let repo = InMemoryEpisodeRepository::new();
        for episode in episodes.iter().cloned() {
            repo.save(episode).await?;
        }

        let start = Date::from_ymd(2022, 11, 19)?;
        let end = Date::from_ymd(2022, 11, 22)?;

        let ordered_by_date_range = repo.order_by_date_range(start, end).await?;

        episodes.sort_by_key(|episode| episode.date());
        let episodes = episodes
            .into_iter()
            .filter(|episode| start <= episode.date() && episode.date() < end)
            .collect::<Vec<_>>();

        assert_eq!(episodes, ordered_by_date_range);
        Ok(())
    }

    #[rstest]
    #[tokio::test]
    async fn test_episode_save_and_remove_by_id(
        episodes: Result<Vec<Episode>, InfraError>,
    ) -> Result<(), InfraError> {
        let mut episodes = episodes?;

        let repo = InMemoryEpisodeRepository::new();
        for episode in episodes.iter().cloned() {
            repo.save(episode).await?;
        }

        let removed_episode = episodes.remove(1); // 二番目を削除
        repo.remove_by_id(removed_episode.id()).await?;

        let mut rest_episodes = repo.all().await?;
        rest_episodes.sort_by_key(|episode| episode.id());

        episodes.sort_by_key(|episode| episode.id());

        assert_eq!(episodes, rest_episodes);
        Ok(())
    }

    #[rstest]
    #[tokio::test]
    async fn test_episode_edit_no_exists() -> Result<(), InfraError> {
        let repo = InMemoryEpisodeRepository::new();

        let episode = Episode::new((2022, 11, 23), "Another Contents".to_string())?;

        let res = repo.edit(episode).await;
        assert_matches!(res, Err(InfraError::RemovedRecordError));

        Ok(())
    }

    #[rstest]
    #[tokio::test]
    async fn test_episode_remove_no_exists() -> Result<(), InfraError> {
        let repo = InMemoryEpisodeRepository::new();

        let episode = Episode::new((2022, 11, 23), "Another Contents".to_string())?;

        let res = repo.remove_by_id(episode.id()).await;
        assert_matches!(res, Err(InfraError::RemovedRecordError));

        Ok(())
    }
}
