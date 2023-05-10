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
#[derive(Default, Debug, Clone)]
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
        let old_episode = self
            .map
            .lock()
            .map_err(|e| InfraError::OtherSQLXError(format!("Inmemory mutex error.{e}")))?
            .insert(episode.id().to_uuid(), episode);
        match old_episode {
            Some(_) => Err(InfraError::ConflictError),
            None => Ok(()),
        }
    }
    async fn edit(&self, episode: Episode) -> Result<(), InfraError> {
        match self
            .map
            .lock()
            .map_err(|e| InfraError::OtherSQLXError(format!("Inmemory mutex error.{e}")))?
            .entry(episode.id().to_uuid())
        {
            Entry::Vacant(_) => Err(InfraError::NoRecordError),
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
            .map_err(|e| InfraError::OtherSQLXError(format!("Inmemory mutex error.{e}")))?
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
    async fn remove(&self, id: EpisodeId) -> Result<(), InfraError> {
        match self
            .map
            .lock()
            .map_err(|e| InfraError::OtherSQLXError(format!("Inmemory mutex error.{e}")))?
            .remove(&id.to_uuid())
        {
            None => Err(InfraError::NoRecordError),
            Some(_) => Ok(()),
        }
    }
}

#[cfg(test)]
mod test {
    use super::InMemoryEpisodeRepository;
    use crate::episode_repository_impl::episode_assert::{
        episodes_assert_eq, episodes_assert_eq_with_sort_by_key_and_filter,
    };
    use crate::InfraError;
    use domain::Date;
    use domain::{episode::Episode, EpisodeRepository};

    use fake::{Fake, Faker};
    use rand::seq::SliceRandom;
    use rstest::{fixture, rstest};

    #[fixture]
    fn episodes() -> Result<Vec<Episode>, InfraError> {
        Ok((0..100)
            .map(|_| Faker.fake::<Episode>())
            .collect::<Vec<_>>())
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

        episodes_assert_eq(&mut episodes_res, &mut episodes);
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
        // episodesの一部を編集
        for _ in 0..(episodes.len() / 2_usize) {
            let edited_episode = episodes.choose_mut(&mut rand::thread_rng()).unwrap();
            let new_episode = Faker.fake::<Episode>();
            edited_episode.assign(new_episode);

            repo.edit(edited_episode.clone()).await?;
        }

        let mut episodes_res = repo.all().await?;

        episodes_assert_eq(&mut episodes_res, &mut episodes);
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

        let start = Date::from_ymd(1000, 1, 1)?;
        let end = Date::from_ymd(2000, 1, 1)?;

        let mut episodes_res = repo.order_by_date_range(start, end).await?;

        episodes_assert_eq_with_sort_by_key_and_filter(
            &mut episodes_res,
            &mut episodes,
            |x, y| x.date().cmp(&y.date()),
            |episode| start <= episode.date() && episode.date() < end,
        );
        Ok(())
    }

    #[rstest]
    #[tokio::test]
    async fn test_episode_save_and_remove(
        episodes: Result<Vec<Episode>, InfraError>,
    ) -> Result<(), InfraError> {
        let episodes = episodes?;

        let repo = InMemoryEpisodeRepository::new();
        for episode in episodes.iter().cloned() {
            repo.save(episode).await?;
        }

        // episodesの一部を削除
        let remove_indices = (0..episodes.len()).collect::<Vec<usize>>();
        let remove_indices = remove_indices.into_iter().take(20).collect::<Vec<_>>();

        let removed_episodes = episodes
            .iter()
            .cloned()
            .enumerate()
            .filter_map(|(i, episode)| remove_indices.contains(&i).then_some(episode))
            .collect::<Vec<_>>();
        let mut rest_episodes = episodes
            .iter()
            .cloned()
            .enumerate()
            .filter_map(|(i, episode)| (!remove_indices.contains(&i)).then_some(episode))
            .collect::<Vec<_>>();

        for episode in removed_episodes.into_iter() {
            repo.remove(episode.id()).await?
        }

        let mut episodes_res = repo.all().await?;
        episodes_assert_eq(&mut episodes_res, &mut rest_episodes);

        Ok(())
    }

    #[rstest]
    #[tokio::test]
    async fn test_episode_edit_no_exists() -> Result<(), InfraError> {
        let repo = InMemoryEpisodeRepository::new();

        let episode = Faker.fake::<Episode>();

        let res = repo.edit(episode).await;
        assert!(matches!(res, Err(InfraError::NoRecordError)));

        Ok(())
    }

    #[rstest]
    #[tokio::test]
    async fn test_episode_remove_no_exists() -> Result<(), InfraError> {
        let repo = InMemoryEpisodeRepository::new();

        let episode = Faker.fake::<Episode>();

        let res = repo.remove(episode.id()).await;
        assert!(matches!(res, Err(InfraError::NoRecordError)));

        Ok(())
    }
}
