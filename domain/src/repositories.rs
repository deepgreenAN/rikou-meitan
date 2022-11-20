use crate::episode::Episode;
use crate::movie_clip::MovieClip;
use async_trait::async_trait;
use uuid::Uuid;

#[async_trait]
pub trait MovieClipRepository {
    type Error;
    async fn save(movie_clip: MovieClip) -> Result<(), <Self as MovieClipRepository>::Error>;
    async fn by_id(id: Uuid) -> Result<MovieClip, <Self as MovieClipRepository>::Error>;
    async fn all() -> Result<Vec<MovieClip>, <Self as MovieClipRepository>::Error>;
    async fn remove_by_id(id: Uuid) -> Result<(), <Self as MovieClipRepository>::Error>;
}

#[async_trait]
pub trait EpisodeRepository {
    type Error;
    async fn save(episode: Episode) -> Result<(), <Self as EpisodeRepository>::Error>;
    async fn by_id(id: Uuid) -> Result<Episode, <Self as EpisodeRepository>::Error>;
    async fn all() -> Result<Vec<Episode>, <Self as EpisodeRepository>::Error>;
    async fn remove_by_id(id: Uuid) -> Result<(), <Self as EpisodeRepository>::Error>;
}
