use crate::InfraError;
use async_trait::async_trait;
use domain::movie_clip::{MovieClip, MovieClipId};
use domain::Date;
use domain::MovieClipRepository;

use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use uuid::Uuid;

// -------------------------------------------------------------------------------------------------
// # MockMovieClipRepository

/// MovieClipのモックリポジトリ
#[derive(Default)]
pub struct MockMovieClipRepository {
    map: Arc<Mutex<HashMap<Uuid, MovieClip>>>,
}

impl MockMovieClipRepository {
    pub fn new() -> Self {
        Self {
            map: Arc::new(Mutex::new(HashMap::new())),
        }
    }
}

#[async_trait]
impl MovieClipRepository for MockMovieClipRepository {
    type Error = InfraError;
    async fn save(&self, movie_clip: MovieClip) -> Result<(), InfraError> {
        self.map
            .lock()
            .unwrap()
            .insert(movie_clip.id().to_uuid(), movie_clip);
        Ok(())
    }
    async fn all(&self) -> Result<Vec<MovieClip>, InfraError> {
        let movie_clips = self
            .map
            .lock()
            .unwrap()
            .values()
            .cloned()
            .collect::<Vec<MovieClip>>();
        Ok(movie_clips)
    }
    async fn order_by_like_limit(&self, length: usize) -> Result<Vec<MovieClip>, InfraError> {
        let mut movie_clips = self.all().await?;
        movie_clips.sort_by_key(|movie_clip| u32::MAX - movie_clip.like()); // 降順なため
        Ok(movie_clips.into_iter().take(length).collect::<Vec<_>>())
    }
    async fn order_by_create_date_range(
        &self,
        start: Date,
        end: Date,
    ) -> Result<Vec<MovieClip>, InfraError> {
        let mut movie_clips = self.all().await?;
        movie_clips.sort_by_key(|movie_clip| movie_clip.create_date());
        Ok(movie_clips
            .into_iter()
            .filter(|movie_clip| {
                start <= movie_clip.create_date() && movie_clip.create_date() < end
            })
            .collect::<Vec<_>>())
    }
    async fn remove_by_id(&self, id: MovieClipId) -> Result<(), InfraError> {
        self.map.lock().unwrap().remove(&id.to_uuid());
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::MockMovieClipRepository;
    use crate::InfraError;
    use domain::movie_clip::MovieClip;
    use domain::Date;
    use domain::MovieClipRepository;
    use pretty_assertions::assert_eq;
    use rstest::{fixture, rstest};

    #[fixture]
    fn movie_clips() -> Result<Vec<MovieClip>, InfraError> {
        Ok(vec![
            MovieClip::new(
                "MovieClip 1".to_string(),
                "https://www.youtube.com/watch?v=B7OPlsdBuVc".to_string(),
                100,
                200,
                (2022, 11, 21),
            )?,
            MovieClip::new(
                "MovieClip 2".to_string(),
                "https://www.youtube.com/watch?v=NHpILI4NpCI".to_string(),
                200,
                300,
                (2022, 11, 22),
            )?,
            MovieClip::new(
                "MovieClip 3".to_string(),
                "https://www.youtube.com/watch?v=6LAn0lbMpZ8".to_string(),
                400,
                500,
                (2022, 11, 19),
            )?,
        ])
    }

    #[rstest]
    #[tokio::test]
    async fn test_movie_clip_save_and_all(
        movie_clips: Result<Vec<MovieClip>, InfraError>,
    ) -> Result<(), InfraError> {
        let mut movie_clips = movie_clips?;

        let repo = MockMovieClipRepository::new();

        for i in movie_clips.iter().cloned() {
            repo.save(i).await?;
        }

        let mut movie_clips_res = repo.all().await?;
        movie_clips_res.sort_by_key(|movie_clip| movie_clip.id());

        movie_clips.sort_by_key(|movie_clip| movie_clip.id());

        assert_eq!(movie_clips, movie_clips_res);
        Ok(())
    }

    #[rstest]
    #[tokio::test]
    async fn test_movie_clip_save_and_order_by_like_limit(
        movie_clips: Result<Vec<MovieClip>, InfraError>,
    ) -> Result<(), InfraError> {
        let movie_clips = movie_clips?;
        let movie_clips_length = movie_clips.len();
        let mut movie_clips = movie_clips
            .into_iter()
            .enumerate()
            .map(|(i, mut movie_clip)| {
                for _ in 0..(movie_clips_length - i) {
                    movie_clip.like_increment(); // likeをインクリメント
                }
                movie_clip
            })
            .collect::<Vec<_>>();

        let repo = MockMovieClipRepository::new();

        for i in movie_clips.iter().cloned() {
            repo.save(i).await?;
        }

        let length = 2_usize;
        let ordered_by_like_movie_clips = repo.order_by_like_limit(length).await?;

        movie_clips.sort_by_key(|movie_clip| u32::MAX - movie_clip.like()); // 降順のため
        let movie_clips = movie_clips.into_iter().take(length).collect::<Vec<_>>();

        assert_eq!(movie_clips, ordered_by_like_movie_clips);
        Ok(())
    }

    #[rstest]
    #[tokio::test]
    async fn test_movie_clip_save_and_order_by_date_range(
        movie_clips: Result<Vec<MovieClip>, InfraError>,
    ) -> Result<(), InfraError> {
        let mut movie_clips = movie_clips?;
        let repo = MockMovieClipRepository::new();

        for i in movie_clips.iter().cloned() {
            repo.save(i).await?;
        }

        let start = Date::from_ymd(2022, 11, 19)?;
        let end = Date::from_ymd(2022, 11, 22)?;

        let ordered_by_date_range = repo.order_by_create_date_range(start, end).await?;

        movie_clips.sort_by_key(|movie_clip| movie_clip.create_date());
        let movie_clips = movie_clips
            .into_iter()
            .filter(|movie_clip| {
                start <= movie_clip.create_date() && movie_clip.create_date() < end
            })
            .collect::<Vec<_>>();

        assert_eq!(movie_clips, ordered_by_date_range);
        Ok(())
    }

    #[rstest]
    #[tokio::test]
    async fn test_movie_clip_save_and_remove_and_all(
        movie_clips: Result<Vec<MovieClip>, InfraError>,
    ) -> Result<(), InfraError> {
        let mut movie_clips = movie_clips?;
        let repo = MockMovieClipRepository::new();

        for i in movie_clips.iter().cloned() {
            repo.save(i).await?;
        }

        let removed_movie_clip = movie_clips.remove(1); // 二番目のデータ
        repo.remove_by_id(removed_movie_clip.id()).await?;

        let mut rest_movie_clips = repo.all().await?;
        rest_movie_clips.sort_by_key(|movie_clip| movie_clip.id());

        movie_clips.sort_by_key(|movie_clip| movie_clip.id());

        assert_eq!(movie_clips, rest_movie_clips);
        Ok(())
    }
}
