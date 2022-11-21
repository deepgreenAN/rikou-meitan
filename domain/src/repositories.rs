use crate::episode::{Episode, EpisodeId};
use crate::movie_clip::{MovieClip, MovieClipId};
use crate::Date;
use async_trait::async_trait;

/// MovieClipのリポジトリのトレイト
#[async_trait]
pub trait MovieClipRepository {
    type Error;
    async fn save(&self, movie_clip: MovieClip)
        -> Result<(), <Self as MovieClipRepository>::Error>;
    async fn all(&self) -> Result<Vec<MovieClip>, <Self as MovieClipRepository>::Error>;
    async fn order_by_like_limit(
        &self,
        length: usize,
    ) -> Result<Vec<MovieClip>, <Self as MovieClipRepository>::Error>;
    async fn order_by_create_date_range(
        &self,
        start: Date,
        end: Date,
    ) -> Result<Vec<MovieClip>, <Self as MovieClipRepository>::Error>;
    async fn remove_by_id(
        &self,
        id: MovieClipId,
    ) -> Result<(), <Self as MovieClipRepository>::Error>;
}

/// Episodeのリポジトリのトレイト
#[async_trait]
pub trait EpisodeRepository {
    type Error;
    async fn save(&self, episode: Episode) -> Result<(), <Self as EpisodeRepository>::Error>;
    async fn all(&self) -> Result<Vec<Episode>, <Self as EpisodeRepository>::Error>;
    async fn order_by_date_range(
        &self,
        start: Date,
        end: Date,
    ) -> Result<Vec<Episode>, <Self as EpisodeRepository>::Error>;
    async fn remove_by_id(&self, id: EpisodeId) -> Result<(), <Self as EpisodeRepository>::Error>;
}
