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
// # MockMovieClipRepository

/// 即席のMovieClipリポジトリ
#[derive(Default)]
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
        self.map.lock().unwrap().insert(clip.id().to_uuid(), clip);
        Ok(())
    }

    async fn edit(&self, clip: MovieClip) -> Result<(), InfraError> {
        match self.map.lock().unwrap().entry(clip.id().to_uuid()) {
            Entry::Vacant(_) => Err(InfraError::NoRecordError),
            Entry::Occupied(mut o) => {
                *o.get_mut() = clip;
                Ok(())
            }
        }
    }
    async fn increment_like(&self, id: MovieClipId) -> Result<(), InfraError> {
        match self.map.lock().unwrap().entry(id.to_uuid()) {
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
            .unwrap()
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
            .filter(|clip| reference.like() >= clip.like() && reference.id() < clip.id())
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
                reference.create_date() >= clip.create_date() && reference.id() < clip.id()
            })
            .take(length)
            .collect::<Vec<_>>())
    }

    async fn remove(&self, id: MovieClipId) -> Result<(), InfraError> {
        match self.map.lock().unwrap().remove(&id.to_uuid()) {
            None => Err(InfraError::NoRecordError),
            Some(_) => Ok(()),
        }
    }
}

#[cfg(test)]
mod test {
    use super::InMemoryMovieClipRepository;
    use crate::InfraError;
    use domain::MovieClipRepository;
    use domain::{
        movie_clip::{MovieClip, MovieClipId},
        Date,
    };

    use fake::{Fake, Faker};
    use pretty_assertions::assert_eq;
    use rand::{distributions::Distribution, seq::SliceRandom};
    use rstest::{fixture, rstest};
    use std::cmp::Ordering;

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
        clips_res.sort_by_key(|clip| clip.id());

        clips.sort_by_key(|clip| clip.id());

        assert_eq!(clips, clips_res);
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
        clips_res.sort_by_key(|clip| clip.id());

        clips.sort_by_key(|clip| clip.id());

        assert_eq!(clips, clips_res);
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
        clips_res.sort_by_key(|clip| clip.id());

        clips.sort_by_key(|clip| clip.id());

        assert_eq!(clips, clips_res);

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

        // 参照元をlike(降順), idの順でソート
        clips.sort_by(|x, y| y.like().cmp(&x.like()).then_with(|| x.id().cmp(&y.id())));
        let clips = clips.into_iter().take(length).collect::<Vec<_>>();

        let clips_res = repo.order_by_like(length).await?;

        assert_eq!(clips, clips_res);

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
            let reference_index =
                rand::distributions::Uniform::from(0..length).sample(&mut rand::thread_rng());
            clips[reference_index].clone()
        };

        // 参照元をlike(降順), idの順でソート・フィルタリング
        clips.sort_by(|x, y| y.like().cmp(&x.like()).then_with(|| x.id().cmp(&y.id())));
        let clips = clips
            .into_iter()
            .filter(|clip| reference.like() >= clip.like() && reference.id() < clip.id())
            .take(length)
            .collect::<Vec<_>>();

        let clips_res = repo.order_by_like_later(&reference, length).await?;

        assert_eq!(clips, clips_res);

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

        // 参照元をcreate_date, idでソート・範囲をフィルタリング
        clips.sort_by(|x, y| {
            x.create_date()
                .cmp(&y.create_date())
                .then_with(|| x.id().cmp(&y.id()))
        });
        clips.retain(|clip| start <= clip.create_date() && clip.create_date() < end);

        // 得られた結果をcreate_dataが同じ場合のみidでソート
        let mut clips_res = repo.order_by_create_date_range(start, end).await?;
        clips_res.sort_by(|x, y| {
            if let Ordering::Equal = x.create_date().cmp(&y.create_date()) {
                x.id().cmp(&y.id())
            } else {
                Ordering::Equal
            }
        });

        assert_eq!(clips, clips_res);

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

        // 参照元をcreate_date(降順), idでソート・範囲をフィルタリング
        clips.sort_by(|x, y| {
            y.create_date()
                .cmp(&x.create_date())
                .then_with(|| x.id().cmp(&y.id()))
        });
        let clips = clips.into_iter().take(length).collect::<Vec<_>>();

        let clips_res = repo.order_by_create_date(length).await?;

        assert_eq!(clips, clips_res);

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
            let reference_index =
                rand::distributions::Uniform::from(0..length).sample(&mut rand::thread_rng());
            clips[reference_index].clone()
        };

        // 参照元をcreate_date(降順), idでソート・範囲をフィルタリング
        clips.sort_by(|x, y| {
            y.create_date()
                .cmp(&x.create_date())
                .then_with(|| x.id().cmp(&y.id()))
        });
        let clips = clips
            .into_iter()
            .filter(|clip| {
                clip.create_date() <= reference.create_date() && clip.id() > reference.id()
            })
            .take(length)
            .collect::<Vec<_>>();

        let clips_res = repo.order_by_create_date_later(&reference, length).await?;

        assert_eq!(clips, clips_res);

        Ok(())
    }

    #[rstest]
    #[tokio::test]
    async fn test_movie_clip_save_and_remove_and_all(
        movie_clips: Result<Vec<MovieClip>, InfraError>,
    ) -> Result<(), InfraError> {
        let mut clips = movie_clips?;

        let repo = InMemoryMovieClipRepository::new();

        for clip in clips.iter().cloned() {
            repo.save(clip).await?;
        }

        // clipsの一部を削除
        let mut clips_len = clips.len();
        let remove_number = clips_len / 10;
        for _ in 0..remove_number {
            let remove_index = rand::distributions::Uniform::from(0_usize..clips_len)
                .sample(&mut rand::thread_rng());
            let removed_clip = clips.remove(remove_index);
            repo.remove(removed_clip.id()).await?;

            // clips_lenを一つ減らす
            clips_len -= 1;
        }

        let mut rest_clips = repo.all().await?;
        rest_clips.sort_by_key(|clip| clip.id());

        clips.sort_by_key(|clip| clip.id());

        assert_eq!(clips, rest_clips);

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
