use crate::InfraError;
use async_trait::async_trait;
use domain::movie_clip::{MovieClip, MovieClipId};
use domain::Date;
use domain::MovieClipRepository;

use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use uuid::Uuid;

// -------------------------------------------------------------------------------------------------
// # InMemoryMovieClipRepository

/// 即席のMovieClipリポジトリ
#[derive(Default, Debug, Clone)]
pub struct InMemoryMovieClipRepository {
    map: Arc<Mutex<HashMap<Uuid, MovieClip>>>,
}

impl InMemoryMovieClipRepository {
    pub fn new() -> Self {
        Self {
            map: Arc::new(Mutex::new(HashMap::new())),
        }
    }
}

#[async_trait]
impl MovieClipRepository for InMemoryMovieClipRepository {
    type Error = InfraError;
    async fn save(&self, clip: MovieClip) -> Result<(), InfraError> {
        let old_clip = self
            .map
            .lock()
            .map_err(|e| InfraError::OtherSQLXError(format!("Inmemory mutex error.{e}")))?
            .insert(clip.id().to_uuid(), clip);
        match old_clip {
            Some(_) => Err(InfraError::ConflictError),
            None => Ok(()),
        }
    }

    async fn edit(&self, clip: MovieClip) -> Result<(), InfraError> {
        match self
            .map
            .lock()
            .map_err(|e| InfraError::OtherSQLXError(format!("Inmemory mutex error.{e}")))?
            .entry(clip.id().to_uuid())
        {
            Entry::Vacant(_) => Err(InfraError::NoRecordError),
            Entry::Occupied(mut o) => {
                *o.get_mut() = clip;
                Ok(())
            }
        }
    }
    async fn increment_like(&self, id: MovieClipId) -> Result<(), InfraError> {
        match self
            .map
            .lock()
            .map_err(|e| InfraError::OtherSQLXError(format!("Inmemory mutex error.{e}")))?
            .entry(id.to_uuid())
        {
            Entry::Vacant(_) => Err(InfraError::NoRecordError),
            Entry::Occupied(mut o) => {
                o.get_mut().increment_like();
                Ok(())
            }
        }
    }

    async fn all(&self) -> Result<Vec<MovieClip>, InfraError> {
        let clips = self
            .map
            .lock()
            .map_err(|e| InfraError::OtherSQLXError(format!("Inmemory mutex error.{e}")))?
            .values()
            .cloned()
            .collect::<Vec<MovieClip>>();
        Ok(clips)
    }
    async fn order_by_like(&self, length: usize) -> Result<Vec<MovieClip>, InfraError> {
        let mut clips = self.all().await?;
        clips.sort_by(|x, y| y.like().cmp(&x.like()).then_with(|| x.id().cmp(&y.id())));
        Ok(clips.into_iter().take(length).collect::<Vec<_>>())
    }

    async fn order_by_like_later(
        &self,
        reference: &MovieClip,
        length: usize,
    ) -> Result<Vec<MovieClip>, InfraError> {
        let mut clips = self.all().await?;
        clips.sort_by(|x, y| y.like().cmp(&x.like()).then_with(|| x.id().cmp(&y.id())));
        Ok(clips
            .into_iter()
            .filter(|clip| {
                reference.like() > clip.like()
                    || (reference.like() == clip.like() && reference.id() < clip.id())
            })
            .take(length)
            .collect::<Vec<_>>())
    }

    async fn order_by_create_date_range(
        &self,
        start: Date,
        end: Date,
    ) -> Result<Vec<MovieClip>, InfraError> {
        let mut clips = self.all().await?;
        clips.sort_by_key(|clip| clip.create_date());
        Ok(clips
            .into_iter()
            .filter(|clip| start <= clip.create_date() && clip.create_date() < end)
            .collect::<Vec<_>>())
    }

    async fn order_by_create_date(
        &self,
        length: usize,
    ) -> Result<Vec<MovieClip>, <Self as MovieClipRepository>::Error> {
        let mut clips = self.all().await?;
        clips.sort_by(|x, y| {
            y.create_date()
                .cmp(&x.create_date())
                .then_with(|| x.id().cmp(&y.id()))
        });
        Ok(clips.into_iter().take(length).collect::<Vec<_>>())
    }

    async fn order_by_create_date_later(
        &self,
        reference: &MovieClip,
        length: usize,
    ) -> Result<Vec<MovieClip>, <Self as MovieClipRepository>::Error> {
        let mut clips = self.all().await?;
        clips.sort_by(|x, y| {
            y.create_date()
                .cmp(&x.create_date())
                .then_with(|| x.id().cmp(&y.id()))
        });
        Ok(clips
            .into_iter()
            .filter(|clip| {
                reference.create_date() > clip.create_date()
                    || (reference.create_date() == clip.create_date() && reference.id() < clip.id())
            })
            .take(length)
            .collect::<Vec<_>>())
    }

    async fn remove(&self, id: MovieClipId) -> Result<(), InfraError> {
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
    use super::InMemoryMovieClipRepository;
    use crate::movie_clip_repository_impl::assert_movie_clip::{
        clips_assert_eq, clips_assert_eq_with_sort_by_key_and_filter,
    };
    use crate::InfraError;
    use domain::MovieClipRepository;
    use domain::{
        movie_clip::{MovieClip, MovieClipId},
        Date,
    };

    use fake::{Fake, Faker};
    use rand::seq::SliceRandom;
    use rand::{thread_rng, Rng};
    use rstest::{fixture, rstest};

    #[fixture]
    fn movie_clips() -> Result<Vec<MovieClip>, InfraError> {
        Ok((0..100)
            .map(|_| Faker.fake::<MovieClip>())
            .collect::<Vec<_>>())
    }

    #[rstest]
    #[tokio::test]
    async fn test_movie_clip_save_and_all(
        movie_clips: Result<Vec<MovieClip>, InfraError>,
    ) -> Result<(), InfraError> {
        let mut clips = movie_clips?;

        let repo = InMemoryMovieClipRepository::new();

        for clip in clips.iter().cloned() {
            repo.save(clip).await?;
        }

        let mut clips_res = repo.all().await?;
        clips_assert_eq(&mut clips_res, &mut clips);

        Ok(())
    }

    #[rstest]
    #[tokio::test]
    async fn test_movie_clip_save_and_edit_and_all(
        movie_clips: Result<Vec<MovieClip>, InfraError>,
    ) -> Result<(), InfraError> {
        let mut clips = movie_clips?;

        let repo = InMemoryMovieClipRepository::new();

        for clip in clips.iter().cloned() {
            repo.save(clip).await?;
        }

        // 編集
        // clipsの一部を編集
        for _ in 0..(clips.len() / 2) {
            let edited_clip = clips.choose_mut(&mut rand::thread_rng()).unwrap();
            edited_clip.assign(Faker.fake());

            repo.edit(edited_clip.clone()).await?;
        }

        let mut clips_res = repo.all().await?;
        clips_assert_eq(&mut clips_res, &mut clips);
        Ok(())
    }

    #[rstest]
    #[tokio::test]
    async fn test_movie_clip_save_and_increment_like_and_all(
        movie_clips: Result<Vec<MovieClip>, InfraError>,
    ) -> Result<(), InfraError> {
        let mut clips = movie_clips?;

        let repo = InMemoryMovieClipRepository::new();

        for clip in clips.iter().cloned() {
            repo.save(clip).await?;
        }

        // clipsの一部をincrement_like
        for _ in 0..(clips.len() / 2) {
            let incremented_clip = clips.choose_mut(&mut rand::thread_rng()).unwrap();
            incremented_clip.increment_like();
            repo.increment_like(incremented_clip.id()).await?;
        }

        let mut clips_res = repo.all().await?;
        clips_assert_eq(&mut clips_res, &mut clips);

        Ok(())
    }

    #[rstest]
    #[tokio::test]
    async fn test_movie_clip_save_and_order_by_like(
        movie_clips: Result<Vec<MovieClip>, InfraError>,
    ) -> Result<(), InfraError> {
        let mut clips = movie_clips?;

        let repo = InMemoryMovieClipRepository::new();

        for clip in clips.iter().cloned() {
            repo.save(clip).await?;
        }

        let length = clips.len() / 2;

        let mut clips_res = repo.order_by_like(length).await?;

        // 参照元をlike(降順), idの順でソートして比較
        clips_assert_eq_with_sort_by_key_and_filter(
            &mut clips_res,
            &mut clips,
            |x, y| y.like().cmp(&x.like()),
            Option::<fn(&MovieClip) -> bool>::None,
            Some(length),
        );

        Ok(())
    }

    #[rstest]
    #[tokio::test]
    async fn test_movie_clip_save_and_order_by_like_later(
        movie_clips: Result<Vec<MovieClip>, InfraError>,
    ) -> Result<(), InfraError> {
        let mut clips = movie_clips?;

        let repo = InMemoryMovieClipRepository::new();

        for clip in clips.iter().cloned() {
            repo.save(clip).await?;
        }

        let length = clips.len() / 2;

        // referenceとなるclipを取得
        let reference = {
            let reference_index = thread_rng().gen_range(0..length);
            clips[reference_index].clone()
        };

        let mut clips_res = repo.order_by_like_later(&reference, length).await?;

        // 参照元をlike(降順), idの順でソート・フィルタリングして比較
        clips_assert_eq_with_sort_by_key_and_filter(
            &mut clips_res,
            &mut clips,
            |x, y| y.like().cmp(&x.like()),
            Some(|clip: &MovieClip| {
                reference.like() > clip.like()
                    || (reference.like() == clip.like() && reference.id() < clip.id())
            }),
            Some(length),
        );

        Ok(())
    }

    #[rstest]
    #[tokio::test]
    async fn test_movie_clip_save_and_order_by_date_range(
        movie_clips: Result<Vec<MovieClip>, InfraError>,
    ) -> Result<(), InfraError> {
        let mut clips = movie_clips?;

        let repo = InMemoryMovieClipRepository::new();

        for clip in clips.iter().cloned() {
            repo.save(clip).await?;
        }

        let start = Faker.fake::<Date>();
        let end = Faker.fake::<Date>();

        let mut clips_res = repo.order_by_create_date_range(start, end).await?;

        // 参照元をcreate_dateでソート・範囲をフィルタリング
        clips_assert_eq_with_sort_by_key_and_filter(
            &mut clips_res,
            &mut clips,
            |x, y| x.create_date().cmp(&y.create_date()),
            Some(|clip: &MovieClip| start <= clip.create_date() && clip.create_date() < end),
            None,
        );

        Ok(())
    }

    #[rstest]
    #[tokio::test]
    async fn test_movie_clip_save_and_order_by_date(
        movie_clips: Result<Vec<MovieClip>, InfraError>,
    ) -> Result<(), InfraError> {
        let mut clips = movie_clips?;

        let repo = InMemoryMovieClipRepository::new();

        for clip in clips.iter().cloned() {
            repo.save(clip).await?;
        }

        let length = clips.len() / 2;

        let mut clips_res = repo.order_by_create_date(length).await?;

        // 参照元をcreate_date(降順)でソート・範囲をフィルタリング
        clips_assert_eq_with_sort_by_key_and_filter(
            &mut clips_res,
            &mut clips,
            |x, y| y.create_date().cmp(&x.create_date()),
            Option::<fn(&MovieClip) -> bool>::None,
            Some(length),
        );

        Ok(())
    }

    #[rstest]
    #[tokio::test]
    async fn test_movie_clip_save_and_order_by_date_later(
        movie_clips: Result<Vec<MovieClip>, InfraError>,
    ) -> Result<(), InfraError> {
        let mut clips = movie_clips?;

        let repo = InMemoryMovieClipRepository::new();

        for clip in clips.iter().cloned() {
            repo.save(clip).await?;
        }

        let length = clips.len() / 2;

        // referenceとなるclipを取得
        let reference = {
            let reference_index = thread_rng().gen_range(0..length);
            clips[reference_index].clone()
        };

        let mut clips_res = repo.order_by_create_date_later(&reference, length).await?;

        // 参照元をcreate_date(降順)でソート・範囲をフィルタリング
        clips_assert_eq_with_sort_by_key_and_filter(
            &mut clips_res,
            &mut clips,
            |x, y| y.create_date().cmp(&x.create_date()),
            Some(|clip: &MovieClip| {
                clip.create_date() < reference.create_date()
                    || (reference.create_date() == clip.create_date() && clip.id() > reference.id())
            }),
            Some(length),
        );

        Ok(())
    }

    #[rstest]
    #[tokio::test]
    async fn test_movie_clip_save_and_remove_and_all(
        movie_clips: Result<Vec<MovieClip>, InfraError>,
    ) -> Result<(), InfraError> {
        let clips = movie_clips?;

        let repo = InMemoryMovieClipRepository::new();

        for clip in clips.iter().cloned() {
            repo.save(clip).await?;
        }

        // clipsの一部を削除
        let remove_indices = (0..clips.len()).collect::<Vec<usize>>();
        let remove_indices = remove_indices.into_iter().take(20).collect::<Vec<_>>();

        let removed_clips = clips
            .iter()
            .cloned()
            .enumerate()
            .filter_map(|(i, clip)| remove_indices.contains(&i).then_some(clip))
            .collect::<Vec<_>>();
        let mut rest_clips = clips
            .iter()
            .cloned()
            .enumerate()
            .filter_map(|(i, clip)| (!remove_indices.contains(&i)).then_some(clip))
            .collect::<Vec<_>>();

        for clip in removed_clips.into_iter() {
            repo.remove(clip.id()).await?
        }

        let mut clips_res = repo.all().await?;
        clips_assert_eq(&mut clips_res, &mut rest_clips);

        Ok(())
    }

    #[rstest]
    #[tokio::test]
    async fn test_movie_clip_edit_no_exists() -> Result<(), InfraError> {
        let repo = InMemoryMovieClipRepository::new();

        let clip = Faker.fake::<MovieClip>();

        let res = repo.edit(clip).await;
        assert!(matches!(res, Err(InfraError::NoRecordError)));

        Ok(())
    }

    #[rstest]
    #[tokio::test]
    async fn test_movie_clip_remove_no_exists() -> Result<(), InfraError> {
        let repo = InMemoryMovieClipRepository::new();

        let res = repo.remove(MovieClipId::generate()).await;

        assert!(matches!(res, Err(InfraError::NoRecordError)));

        Ok(())
    }
}
