use crate::InfraError;
use async_trait::async_trait;
use domain::video::{Kirinuki, Original, Video, VideoId};
use domain::VideoRepository;

use mockall::mock;

mock! {
    /// VideoRepositoryのモックリポジトリ
    pub VideoOriginalRepository{}

    #[async_trait]
    impl VideoRepository<Original> for VideoOriginalRepository {
        type Error = InfraError;
        async fn save(&self, video: Video<Original>) -> Result<(), InfraError>;
        async fn edit(&self, new_video: Video<Original>) -> Result<(), InfraError>;
        async fn increment_like(&self, id: VideoId) -> Result<(), InfraError>;
        async fn all(&self) -> Result<Vec<Video<Original>>,InfraError>;
        async fn order_by_date(
            &self,
            length: usize,
        ) -> Result<Vec<Video<Original>>, InfraError>;
        async fn order_by_date_later(
            &self,
            reference: &Video<Original>,
            length: usize,
        ) -> Result<Vec<Video<Original>>, InfraError>;
        async fn order_by_like(
            &self,
            length: usize,
        ) -> Result<Vec<Video<Original>>, InfraError>;
        async fn order_by_like_later(
            &self,
            reference: &Video<Original>,
            length: usize,
        ) -> Result<Vec<Video<Original>>, InfraError>;
        async fn remove(&self, id: VideoId) -> Result<(), InfraError>;
        }
}

mock! {
    /// VideoRepositoryのモックリポジトリ
    pub VideoKirinukiRepository{}

    #[async_trait]
    impl VideoRepository<Kirinuki> for VideoKirinukiRepository {
        type Error = InfraError;
        async fn save(&self, video: Video<Kirinuki>) -> Result<(), InfraError>;
        async fn edit(&self, new_video: Video<Kirinuki>) -> Result<(), InfraError>;
        async fn increment_like(&self, id: VideoId) -> Result<(), InfraError>;
        async fn all(&self) -> Result<Vec<Video<Kirinuki>>,InfraError>;
        async fn order_by_date(
            &self,
            length: usize,
        ) -> Result<Vec<Video<Kirinuki>>, InfraError>;
        async fn order_by_date_later(
            &self,
            reference: &Video<Kirinuki>,
            length: usize,
        ) -> Result<Vec<Video<Kirinuki>>, InfraError>;
        async fn order_by_like(
            &self,
            length: usize,
        ) -> Result<Vec<Video<Kirinuki>>, InfraError>;
        async fn order_by_like_later(
            &self,
            reference: &Video<Kirinuki>,
            length: usize,
        ) -> Result<Vec<Video<Kirinuki>>, InfraError>;
        async fn remove(&self, id: VideoId) -> Result<(), InfraError>;
        }
}
