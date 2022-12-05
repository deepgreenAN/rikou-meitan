use crate::InfraError;
use async_trait::async_trait;
use domain::movie_clip::{MovieClip, MovieClipId};
use domain::Date;
use domain::MovieClipRepository;

use mockall::mock;

mock! {
    /// MovieClipのモックリポジトリ
    pub MovieClipRepositoryImpl {}

    #[async_trait]
    impl MovieClipRepository for MovieClipRepositoryImpl {
        type Error = InfraError;
        async fn save(&self, movie_clip: MovieClip)
        -> Result<(), InfraError>;
        async fn edit(&self, movie_clip: MovieClip)
            -> Result<(), InfraError>;
        async fn all(&self) -> Result<Vec<MovieClip>, InfraError>;
        async fn order_by_like_limit(
            &self,
            length: usize,
        ) -> Result<Vec<MovieClip>, InfraError>;
        async fn order_by_create_date_range(
            &self,
            start: Date,
            end: Date,
        ) -> Result<Vec<MovieClip>, InfraError>;
        async fn remove_by_id(
            &self,
            id: MovieClipId,
        ) -> Result<(), InfraError>;
    }
}
