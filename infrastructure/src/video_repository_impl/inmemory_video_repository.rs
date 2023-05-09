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
    use crate::video_repository_impl::assert_video::{
        videos_assert_eq, videos_assert_eq_with_sort_by_key_and_filter,
    };
    use crate::InfraError;
    use domain::video::{Original, Video};
    use domain::VideoRepository;

    use fake::{Fake, Faker};
    use rand::seq::SliceRandom;
    use rand::Rng;
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

        let mut originals_res = repo.all().await?;

        videos_assert_eq(&mut originals_res, &mut originals);

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

        let mut originals_res = repo.all().await?;
        videos_assert_eq(&mut originals_res, &mut originals);

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

        let mut originals_res = repo.all().await?;
        videos_assert_eq(&mut originals_res, &mut originals);

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

        let mut originals_res = repo.order_by_like(length).await?;

        // originalsをlike(降順)・idの順に並べる．length分フィルタリング．
        videos_assert_eq_with_sort_by_key_and_filter(
            &mut originals_res,
            &mut originals,
            |x, y| y.like().cmp(&x.like()),
            Option::<fn(&Video<Original>) -> bool>::None,
            Some(length),
        );

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
            let reference_index = rand::thread_rng().gen_range(0..length);
            originals[reference_index].clone()
        };

        let mut originals_res = repo.order_by_like_later(&reference, length).await?;

        // originalsをlike(降順)・idの順に並べ，フィルタリングして比較
        videos_assert_eq_with_sort_by_key_and_filter(
            &mut originals_res,
            &mut originals,
            |x, y| y.like().cmp(&x.like()),
            Some(|original: &Video<Original>| {
                original.like() < reference.like()
                    || (original.like() == reference.like() && original.id() > reference.id())
            }),
            Some(length),
        );

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

        let mut originals_res = repo.order_by_date(length).await?;

        // originalsをdate(降順)・idの順に並べる．length分フィルタリング．
        videos_assert_eq_with_sort_by_key_and_filter(
            &mut originals_res,
            &mut originals,
            |x, y| y.date().cmp(&x.date()),
            Option::<fn(&Video<Original>) -> bool>::None,
            Some(length),
        );

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
            let reference_index = rand::thread_rng().gen_range(0..length);
            originals[reference_index].clone()
        };

        let mut originals_res = repo.order_by_date_later(&reference, length).await?;

        // originalsをdate(降順)・idの順に並べ，フィルタリング
        videos_assert_eq_with_sort_by_key_and_filter(
            &mut originals_res,
            &mut originals,
            |x, y| y.date().cmp(&x.date()),
            Some(|original: &Video<Original>| {
                original.date() < reference.date()
                    || (original.date() == reference.date() && original.id() > reference.id())
            }),
            Some(length),
        );

        Ok(())
    }

    #[rstest]
    #[tokio::test]
    async fn test_video_save_and_remove_and_all(
        original_videos: Result<Vec<Video<Original>>, InfraError>,
    ) -> Result<(), InfraError> {
        let originals = original_videos?;

        let repo = InMemoryVideoRepository::<Original>::new();

        for original in originals.iter().cloned() {
            repo.save(original).await?;
        }

        // originalsの一部を削除
        let remove_indices = (0..originals.len()).collect::<Vec<usize>>();
        let remove_indices = remove_indices.into_iter().take(20).collect::<Vec<_>>();

        let removed_originals = originals
            .iter()
            .cloned()
            .enumerate()
            .filter_map(|(i, original)| remove_indices.contains(&i).then_some(original))
            .collect::<Vec<_>>();
        let mut rest_originals = originals
            .iter()
            .cloned()
            .enumerate()
            .filter_map(|(i, original)| (!remove_indices.contains(&i)).then_some(original))
            .collect::<Vec<_>>();

        for original in removed_originals.into_iter() {
            repo.remove(original.id()).await?
        }

        let mut originals_res = repo.all().await?;

        videos_assert_eq(&mut originals_res, &mut rest_originals);

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
