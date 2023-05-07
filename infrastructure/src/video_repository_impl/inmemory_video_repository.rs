use crate::InfraError;
use async_trait::async_trait;
use domain::video::{Video, VideoId, VideoType};
use domain::VideoRepository;

use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use uuid::Uuid;

// -------------------------------------------------------------------------------------------------
// # InmemoryVideoRepository

/// 即席のVideoRepository
#[derive(Default, Debug, Clone)]
pub struct InMemoryVideoRepository<T: VideoType> {
    map: Arc<Mutex<HashMap<Uuid, Video<T>>>>,
}

impl<T: VideoType> InMemoryVideoRepository<T> {
    pub fn new() -> Self {
        Self {
            map: Arc::new(Mutex::new(HashMap::new())),
        }
    }
}

#[async_trait]
impl<T: VideoType> VideoRepository<T> for InMemoryVideoRepository<T> {
    type Error = InfraError;
    async fn save(&self, video: Video<T>) -> Result<(), InfraError> {
        let old_video = self.map.lock().unwrap().insert(video.id().to_uuid(), video);

        match old_video {
            Some(_) => Err(InfraError::ConflictError),
            None => Ok(()),
        }
    }
    async fn edit(&self, new_video: Video<T>) -> Result<(), InfraError> {
        match self.map.lock().unwrap().entry(new_video.id().to_uuid()) {
            Entry::Vacant(_) => Err(InfraError::NoRecordError),
            Entry::Occupied(mut o) => {
                o.insert(new_video);
                Ok(())
            }
        }
    }
    async fn increment_like(&self, id: VideoId) -> Result<(), InfraError> {
        match self.map.lock().unwrap().entry(id.to_uuid()) {
            Entry::Vacant(_) => Err(InfraError::NoRecordError),
            Entry::Occupied(mut o) => {
                o.get_mut().increment_like();
                Ok(())
            }
        }
    }
    async fn all(&self) -> Result<Vec<Video<T>>, InfraError> {
        let videos = self
            .map
            .lock()
            .unwrap()
            .values()
            .cloned()
            .collect::<Vec<_>>();

        Ok(videos)
    }
    async fn order_by_date(&self, length: usize) -> Result<Vec<Video<T>>, InfraError> {
        let mut clips = self.all().await?;
        clips.sort_by(|x, y| y.date().cmp(&x.date()).then_with(|| x.id().cmp(&y.id())));
        let clips = clips.into_iter().take(length).collect::<Vec<_>>();
        Ok(clips)
    }
    async fn order_by_date_later(
        &self,
        reference: &Video<T>,
        length: usize,
    ) -> Result<Vec<Video<T>>, InfraError> {
        let mut clips = self.all().await?;
        clips.sort_by(|x, y| y.date().cmp(&x.date()).then_with(|| x.id().cmp(&y.id())));
        let clips = clips
            .into_iter()
            .filter(|clip| {
                reference.date() > clip.date()
                    || (reference.date() == clip.date() && reference.id() < clip.id())
            })
            .take(length)
            .collect::<Vec<_>>();
        Ok(clips)
    }
    async fn order_by_like(&self, length: usize) -> Result<Vec<Video<T>>, InfraError> {
        let mut clips = self.all().await?;
        clips.sort_by(|x, y| y.like().cmp(&x.like()).then_with(|| x.id().cmp(&y.id())));
        let clips = clips.into_iter().take(length).collect::<Vec<_>>();
        Ok(clips)
    }
    async fn order_by_like_later(
        &self,
        reference: &Video<T>,
        length: usize,
    ) -> Result<Vec<Video<T>>, InfraError> {
        let mut clips = self.all().await?;
        clips.sort_by(|x, y| y.like().cmp(&x.like()).then_with(|| x.id().cmp(&y.id())));
        let clips = clips
            .into_iter()
            .filter(|clip| {
                reference.like() > clip.like()
                    || (reference.like() == clip.like() && reference.id() < clip.id())
            })
            .take(length)
            .collect::<Vec<_>>();
        Ok(clips)
    }
    async fn remove(&self, id: VideoId) -> Result<(), InfraError> {
        match self.map.lock().unwrap().remove(&id.to_uuid()) {
            None => Err(InfraError::NoRecordError),
            Some(_) => Ok(()),
        }
    }
}

#[cfg(test)]
mod test {
    use super::InMemoryVideoRepository;
    use crate::InfraError;
    use domain::video::{Original, Video};
    use domain::VideoRepository;

    use fake::{Fake, Faker};
    use pretty_assertions::assert_eq;
    use rand::{distributions::Distribution, seq::SliceRandom};
    use rstest::{fixture, rstest};

    #[fixture]
    fn original_videos() -> Result<Vec<Video<Original>>, InfraError> {
        Ok((0..100)
            .map(|_| Faker.fake::<Video<Original>>())
            .collect::<Vec<_>>())
    }

    #[rstest]
    #[tokio::test]
    async fn test_video_save_and_all(
        original_videos: Result<Vec<Video<Original>>, InfraError>,
    ) -> Result<(), InfraError> {
        let mut originals = original_videos?;

        let repo = InMemoryVideoRepository::<Original>::new();

        for original in originals.iter().cloned() {
            repo.save(original).await?;
        }

        originals.sort_by_key(|original| original.id());

        let mut originals_res = repo.all().await?;
        originals_res.sort_by_key(|original| original.id());

        assert_eq!(originals, originals_res);

        Ok(())
    }

    #[rstest]
    #[tokio::test]
    async fn test_video_save_and_edit_and_all(
        original_videos: Result<Vec<Video<Original>>, InfraError>,
    ) -> Result<(), InfraError> {
        let mut originals = original_videos?;

        let repo = InMemoryVideoRepository::<Original>::new();

        for original in originals.iter().cloned() {
            repo.save(original).await?;
        }

        // originalsの一部を編集．
        for _ in 0..(originals.len() / 2) {
            let edited_original = originals.choose_mut(&mut rand::thread_rng()).unwrap();
            edited_original.assign(Faker.fake());
            repo.edit(edited_original.clone()).await?;
        }

        originals.sort_by_key(|original| original.id());

        let mut originals_res = repo.all().await?;
        originals_res.sort_by_key(|original| original.id());

        assert_eq!(originals, originals_res);

        Ok(())
    }

    #[rstest]
    #[tokio::test]
    async fn test_video_save_and_increment_like_and_all(
        original_videos: Result<Vec<Video<Original>>, InfraError>,
    ) -> Result<(), InfraError> {
        let mut originals = original_videos?;

        let repo = InMemoryVideoRepository::<Original>::new();

        for original in originals.iter().cloned() {
            repo.save(original).await?;
        }

        // originalsの一部をincrement_like
        for _ in 0..(originals.len() / 2) {
            let incremented_original = originals.choose_mut(&mut rand::thread_rng()).unwrap();
            incremented_original.increment_like();
            repo.increment_like(incremented_original.id()).await?;
        }

        originals.sort_by_key(|original| original.id());

        let mut originals_res = repo.all().await?;
        originals_res.sort_by_key(|original| original.id());

        assert_eq!(originals, originals_res);

        Ok(())
    }

    #[rstest]
    #[tokio::test]
    async fn test_video_save_and_order_by_like_and_all(
        original_videos: Result<Vec<Video<Original>>, InfraError>,
    ) -> Result<(), InfraError> {
        let mut originals = original_videos?;

        let repo = InMemoryVideoRepository::<Original>::new();

        for original in originals.iter().cloned() {
            repo.save(original).await?;
        }

        let length = originals.len() / 2;

        // originalsをlike(降順)・idの順に並べる．length分フィルタリング．
        originals.sort_by(|x, y| y.like().cmp(&x.like()).then_with(|| x.id().cmp(&y.id())));
        let originals = originals.into_iter().take(length).collect::<Vec<_>>();

        let originals_res = repo.order_by_like(length).await?;

        assert_eq!(originals, originals_res);

        Ok(())
    }

    #[rstest]
    #[tokio::test]
    async fn test_video_save_and_order_by_like_later_and_all(
        original_videos: Result<Vec<Video<Original>>, InfraError>,
    ) -> Result<(), InfraError> {
        let mut originals = original_videos?;

        let repo = InMemoryVideoRepository::<Original>::new();

        for original in originals.iter().cloned() {
            repo.save(original).await?;
        }

        let length = originals.len() / 2;

        // referenceとなるvideoを取得
        let reference = {
            let reference_index =
                rand::distributions::Uniform::from(0..length).sample(&mut rand::thread_rng());
            originals[reference_index].clone()
        };

        // originalsをlike(降順)・idの順に並べ，フィルタリング
        originals.sort_by(|x, y| y.like().cmp(&x.like()).then_with(|| x.id().cmp(&y.id())));
        let originals = originals
            .into_iter()
            .filter(|original| {
                original.like() < reference.like()
                    || (original.like() == reference.like() && original.id() > reference.id())
            })
            .take(length)
            .collect::<Vec<_>>();

        let originals_res = repo.order_by_like_later(&reference, length).await?;

        assert_eq!(originals, originals_res);

        Ok(())
    }

    #[rstest]
    #[tokio::test]
    async fn test_video_save_and_order_by_date_and_all(
        original_videos: Result<Vec<Video<Original>>, InfraError>,
    ) -> Result<(), InfraError> {
        let mut originals = original_videos?;

        let repo = InMemoryVideoRepository::<Original>::new();

        for original in originals.iter().cloned() {
            repo.save(original).await?;
        }

        let length = originals.len() / 2;

        // originalsをdate(降順)・idの順に並べる．length分フィルタリング．
        originals.sort_by(|x, y| y.date().cmp(&x.date()).then_with(|| x.id().cmp(&y.id())));
        let originals = originals.into_iter().take(length).collect::<Vec<_>>();

        let originals_res = repo.order_by_date(length).await?;

        assert_eq!(originals, originals_res);

        Ok(())
    }

    #[rstest]
    #[tokio::test]
    async fn test_video_save_and_order_by_date_later_and_all(
        original_videos: Result<Vec<Video<Original>>, InfraError>,
    ) -> Result<(), InfraError> {
        let mut originals = original_videos?;

        let repo = InMemoryVideoRepository::<Original>::new();

        for original in originals.iter().cloned() {
            repo.save(original).await?;
        }

        let length = originals.len() / 2;

        // referenceとなるvideoを取得
        let reference = {
            let reference_index =
                rand::distributions::Uniform::from(0..length).sample(&mut rand::thread_rng());
            originals[reference_index].clone()
        };

        // originalsをdate(降順)・idの順に並べ，フィルタリング
        originals.sort_by(|x, y| y.date().cmp(&x.date()).then_with(|| x.id().cmp(&y.id())));
        let originals = originals
            .into_iter()
            .filter(|original| {
                original.date() < reference.date()
                    || (original.date() == reference.date() && original.id() > reference.id())
            })
            .take(length)
            .collect::<Vec<_>>();

        let originals_res = repo.order_by_date_later(&reference, length).await?;

        assert_eq!(originals, originals_res);

        Ok(())
    }

    #[rstest]
    #[tokio::test]
    async fn test_video_save_and_remove_and_all(
        original_videos: Result<Vec<Video<Original>>, InfraError>,
    ) -> Result<(), InfraError> {
        let mut originals = original_videos?;

        let repo = InMemoryVideoRepository::<Original>::new();

        for original in originals.iter().cloned() {
            repo.save(original).await?;
        }

        // originalsの一部を削除
        let mut originals_len = originals.len();
        let remove_number = originals_len / 10;
        for _ in 0..remove_number {
            let remove_index = rand::distributions::Uniform::from(0_usize..originals_len)
                .sample(&mut rand::thread_rng());
            let removed_original = originals.remove(remove_index);
            repo.remove(removed_original.id()).await?;

            // originals_lenを一つ減らす
            originals_len -= 1;
        }

        originals.sort_by_key(|original| original.id());

        let mut originals_res = repo.all().await?;
        originals_res.sort_by_key(|original| original.id());

        assert_eq!(originals, originals_res);

        Ok(())
    }

    #[rstest]
    #[tokio::test]
    async fn test_video_edit_no_exists() -> Result<(), InfraError> {
        let repo = InMemoryVideoRepository::<Original>::new();

        let original = Faker.fake::<Video<Original>>();

        let res = repo.edit(original).await;

        assert!(matches!(res, Err(InfraError::NoRecordError)));

        Ok(())
    }

    #[rstest]
    #[tokio::test]
    async fn test_video_remove_no_exists() -> Result<(), InfraError> {
        let repo = InMemoryVideoRepository::<Original>::new();

        let original = Faker.fake::<Video<Original>>();

        let res = repo.remove(original.id()).await;

        assert!(matches!(res, Err(InfraError::NoRecordError)));

        Ok(())
    }
}
