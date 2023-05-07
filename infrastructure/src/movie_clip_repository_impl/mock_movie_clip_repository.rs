use crate::InfraError;
use async_trait::async_trait;
use domain::movie_clip::{MovieClip, MovieClipId};
use domain::Date;
use domain::MovieClipRepository;

use mockall::mock;

mock! {
    /// MovieClipのモックリポジトリ
    #[derive(Debug, Clone)]
    pub MovieClipRepository {}

    #[async_trait]
    impl MovieClipRepository for MovieClipRepository {
        type Error = InfraError;
        async fn save(&self, movie_clip: MovieClip)
        -> Result<(), InfraError>;
        async fn edit(&self, movie_clip: MovieClip)
            -> Result<(), InfraError>;
        async fn increment_like(
            &self,
            id: MovieClipId,
        ) -> Result<(), InfraError>;
        async fn all(&self) -> Result<Vec<MovieClip>, InfraError>;
        async fn order_by_like(
            &self,
            length: usize,
        ) -> Result<Vec<MovieClip>, InfraError>;
        async fn order_by_like_later(
            &self,
            reference: &MovieClip,
            length: usize,
        ) -> Result<Vec<MovieClip>, InfraError>;
        async fn order_by_create_date_range(
            &self,
            start: Date,
            end: Date,
        ) -> Result<Vec<MovieClip>, InfraError>;
        async fn order_by_create_date(
            &self,
            length: usize,
        ) -> Result<Vec<MovieClip>, InfraError>;
        async fn order_by_create_date_later(
            &self,
            reference: &MovieClip,
            length: usize,
        ) -> Result<Vec<MovieClip>, InfraError>;
        async fn remove(
            &self,
            id: MovieClipId,
        ) -> Result<(), InfraError>;
    }
}
