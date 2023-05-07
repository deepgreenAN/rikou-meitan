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
            .unwrap()
            .insert(episode.id().to_uuid(), episode);
        match old_episode {
            Some(_) => Err(InfraError::ConflictError),
            None => Ok(()),
        }
    }
    async fn edit(&self, episode: Episode) -> Result<(), InfraError> {
        match self.map.lock().unwrap().entry(episode.id().to_uuid()) {
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
    async fn remove(&self, id: EpisodeId) -> Result<(), InfraError> {
        match self.map.lock().unwrap().remove(&id.to_uuid()) {
            None => Err(InfraError::NoRecordError),
            Some(_) => Ok(()),
        }
    }
}

#[cfg(test)]
mod test {
    use super::InMemoryEpisodeRepository;
    use crate::InfraError;
    use domain::Date;
    use domain::{episode::Episode, EpisodeRepository};

    use fake::{Fake, Faker};
    use pretty_assertions::assert_eq;
    use rand::{distributions::Distribution, seq::SliceRandom};
    use rstest::{fixture, rstest};
    use std::cmp::Ordering;

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
        // 取得結果をidでソート
        episodes_res.sort_by_key(|episode| episode.id());
        // 参照元をidでソート
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
        // episodesの一部を編集
        for _ in 0..(episodes.len() / 2_usize) {
            let edited_episode = episodes.choose_mut(&mut rand::thread_rng()).unwrap();
            let new_episode = Faker.fake::<Episode>();
            edited_episode.assign(new_episode);

            repo.edit(edited_episode.clone()).await?;
        }

        let mut episodes_res = repo.all().await?;
        // 取得結果をidでソート
        episodes_res.sort_by_key(|episode| episode.id());
        // 参照元をidでソート
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

        // 参照元をDate, idの順でソート・フィルター
        episodes.sort_by(|x, y| x.date().cmp(&y.date()).then_with(|| x.id().cmp(&y.id())));
        episodes.retain(|episode| start <= episode.date() && episode.date() < end);

        let mut episodes_res = repo.order_by_date_range(start, end).await?;
        // データベースから得られた結果をDateが同じ場合のみidでソート
        episodes_res.sort_by(|x, y| {
            if let Ordering::Equal = x.date().cmp(&y.date()) {
                x.id().cmp(&y.id())
            } else {
                Ordering::Equal
            }
        });

        assert_eq!(episodes, episodes_res);
        Ok(())
    }

    #[rstest]
    #[tokio::test]
    async fn test_episode_save_and_remove(
        episodes: Result<Vec<Episode>, InfraError>,
    ) -> Result<(), InfraError> {
        let mut episodes = episodes?;

        let repo = InMemoryEpisodeRepository::new();
        for episode in episodes.iter().cloned() {
            repo.save(episode).await?;
        }

        // episodesの一部を削除
        let mut episodes_len = episodes.len();
        let remove_number = episodes_len / 10;
        for _ in 0..remove_number {
            let remove_index = rand::distributions::Uniform::from(0_usize..episodes_len)
                .sample(&mut rand::thread_rng());
            let removed_episode = episodes.remove(remove_index);
            repo.remove(removed_episode.id()).await?;

            // episode_renを一つ減らす．
            episodes_len -= 1;
        }

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
        assert!(matches!(res, Err(InfraError::NoRecordError)));

        Ok(())
    }

    #[rstest]
    #[tokio::test]
    async fn test_episode_remove_no_exists() -> Result<(), InfraError> {
        let repo = InMemoryEpisodeRepository::new();

        let episode = Episode::new((2022, 11, 23), "Another Contents".to_string())?;

        let res = repo.remove(episode.id()).await;
        assert!(matches!(res, Err(InfraError::NoRecordError)));

        Ok(())
    }
}
