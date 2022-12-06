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
    async fn save(&self, movie_clip: MovieClip) -> Result<(), InfraError> {
        self.map
            .lock()
            .unwrap()
            .insert(movie_clip.id().to_uuid(), movie_clip);
        Ok(())
    }

    async fn edit(&self, movie_clip: MovieClip) -> Result<(), InfraError> {
        match self.map.lock().unwrap().entry(movie_clip.id().to_uuid()) {
            Entry::Vacant(_) => Err(InfraError::NoRecordError),
            Entry::Occupied(mut o) => {
                *o.get_mut() = movie_clip;
                Ok(())
            }
        }
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
    use assert_matches::assert_matches;
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

        let repo = InMemoryMovieClipRepository::new();

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
    async fn test_movie_clip_save_and_edit_and_all(
        movie_clips: Result<Vec<MovieClip>, InfraError>,
    ) -> Result<(), InfraError> {
        let mut movie_clips = movie_clips?;

        let repo = InMemoryMovieClipRepository::new();

        for i in movie_clips.iter().cloned() {
            repo.save(i).await?;
        }

        // 編集
        let mut edited_movie_clip = movie_clips[1].clone(); // 二番目を編集
        edited_movie_clip.edit_title("Another Movie Clip".to_string())?;
        edited_movie_clip.edit_start_and_end(1200.into(), 1300.into())?;
        movie_clips[1] = edited_movie_clip.clone();

        repo.edit(edited_movie_clip).await?;

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

        let repo = InMemoryMovieClipRepository::new();

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
        let repo = InMemoryMovieClipRepository::new();

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
        let repo = InMemoryMovieClipRepository::new();

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

    #[rstest]
    #[tokio::test]
    async fn test_movie_clip_edit_no_exists() -> Result<(), InfraError> {
        let repo = InMemoryMovieClipRepository::new();

        let movie_clip = MovieClip::new(
            "Another Title".to_string(),
            "https://www.youtube.com/watch?v=lwSEI1ATLWQ".to_string(),
            1000,
            1500,
            (2022, 11, 23),
        )?;

        let res = repo.edit(movie_clip).await;
        assert_matches!(res, Err(InfraError::NoRecordError));
        Ok(())
    }

    #[rstest]
    #[tokio::test]
    async fn test_movie_clip_remove_no_exists() -> Result<(), InfraError> {
        let repo = InMemoryMovieClipRepository::new();

        let movie_clip = MovieClip::new(
            "Another Title".to_string(),
            "https://www.youtube.com/watch?v=lwSEI1ATLWQ".to_string(),
            1000,
            1500,
            (2022, 11, 23),
        )?;

        let res = repo.remove_by_id(movie_clip.id()).await;
        assert_matches!(res, Err(InfraError::NoRecordError));
        Ok(())
    }
}
