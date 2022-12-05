use crate::InfraError;
use async_trait::async_trait;
use domain::episode::{Episode, EpisodeId};
use domain::Date;
use domain::EpisodeRepository;

use mockall::mock;

mock! {
    /// EpisodeRepositoryトレイトのモック
    pub EpisodeRepositoryImpl {}

    #[async_trait]
    impl EpisodeRepository for EpisodeRepositoryImpl {
        type Error = InfraError;
        async fn save(&self, episode: Episode) -> Result<(), <Self as EpisodeRepository>::Error>;
        async fn edit(&self, episode: Episode) -> Result<(), <Self as EpisodeRepository>::Error>;
        async fn all(&self) -> Result<Vec<Episode>, <Self as EpisodeRepository>::Error>;
        async fn order_by_date_range(
            &self,
            start: Date,
            end: Date,
        ) -> Result<Vec<Episode>, <Self as EpisodeRepository>::Error>;
        async fn remove_by_id(&self, id: EpisodeId) -> Result<(), <Self as EpisodeRepository>::Error>;
    }
}
