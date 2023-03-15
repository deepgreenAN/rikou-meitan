use crate::episode::{Episode, EpisodeId};
use crate::movie_clip::{MovieClip, MovieClipId};
use crate::video::{Video, VideoId};
use crate::Date;
use async_trait::async_trait;

/// MovieClipのリポジトリのトレイト
#[async_trait]
pub trait MovieClipRepository {
    type Error;
    /// MovieClipを保存する．
    async fn save(&self, movie_clip: MovieClip)
        -> Result<(), <Self as MovieClipRepository>::Error>;
    /// MovieClipを編集する．
    async fn edit(&self, movie_clip: MovieClip)
        -> Result<(), <Self as MovieClipRepository>::Error>;
    /// idをもつMovieClipのLikeをインクリメントする．
    async fn increment_like(
        &self,
        id: MovieClipId,
    ) -> Result<(), <Self as MovieClipRepository>::Error>;
    /// 全てのMovieClipを取得する．
    async fn all(&self) -> Result<Vec<MovieClip>, <Self as MovieClipRepository>::Error>;
    /// Likeで並べてlength分のMovieClipを取得する．
    async fn order_by_like(
        &self,
        length: usize,
    ) -> Result<Vec<MovieClip>, <Self as MovieClipRepository>::Error>;
    /// Likeで並べてreference以降のMovieClipをlength分取得する．
    async fn order_by_like_later(
        &self,
        reference: &MovieClip,
        length: usize,
    ) -> Result<Vec<MovieClip>, <Self as MovieClipRepository>::Error>;
    /// create_dateで並べてstartからendまでの範囲分のMovieClipを取得する．
    async fn order_by_create_date_range(
        &self,
        start: Date,
        end: Date,
    ) -> Result<Vec<MovieClip>, <Self as MovieClipRepository>::Error>;
    /// create_dateで並べてlength分のMovieClipを取得する．
    async fn order_by_create_date(
        &self,
        length: usize,
    ) -> Result<Vec<MovieClip>, <Self as MovieClipRepository>::Error>;
    /// create_dateで並べてreference以降のMovieClipをlength分取得する．
    async fn order_by_create_date_later(
        &self,
        reference: &MovieClip,
        length: usize,
    ) -> Result<Vec<MovieClip>, <Self as MovieClipRepository>::Error>;
    /// idを持つ要素を削除する．
    async fn remove(&self, id: MovieClipId) -> Result<(), <Self as MovieClipRepository>::Error>;
}

/// Episodeのリポジトリのトレイト
#[async_trait]
pub trait EpisodeRepository {
    type Error;
    /// Episodeを保存する．
    async fn save(&self, episode: Episode) -> Result<(), <Self as EpisodeRepository>::Error>;
    /// Episodeを編集する．
    async fn edit(&self, episode: Episode) -> Result<(), <Self as EpisodeRepository>::Error>;
    /// 全てのEpisodeを取得する．
    async fn all(&self) -> Result<Vec<Episode>, <Self as EpisodeRepository>::Error>;
    /// dateで並べてstartからendまでの範囲分のEpisodeを取得する．
    async fn order_by_date_range(
        &self,
        start: Date,
        end: Date,
    ) -> Result<Vec<Episode>, <Self as EpisodeRepository>::Error>;
    /// idをもつEpisodeを削除する．
    async fn remove(&self, id: EpisodeId) -> Result<(), <Self as EpisodeRepository>::Error>;
}

/// Video<T>のリポジトリのトレイト
#[async_trait]
pub trait VideoRepository<T> {
    type Error;
    /// Video<T>を保存する．
    async fn save(&self, video: Video<T>) -> Result<(), <Self as VideoRepository<T>>::Error>;
    /// Video<T>を編集する．
    async fn edit(&self, new_video: Video<T>) -> Result<(), <Self as VideoRepository<T>>::Error>;
    /// `id`をもつVideo<T>のLikeをインクリメントする．
    async fn increment_like(&self, id: VideoId) -> Result<(), <Self as VideoRepository<T>>::Error>;
    /// 全てのVideo<T>を取得する．
    async fn all(&self) -> Result<Vec<Video<T>>, <Self as VideoRepository<T>>::Error>;
    /// dateで並べて`length`分のVideo<T>を取得する．
    async fn order_by_date(
        &self,
        length: usize,
    ) -> Result<Vec<Video<T>>, <Self as VideoRepository<T>>::Error>;
    /// dateで並べて`reference`以降のVideo<T>を`length`分取得する．
    async fn order_by_date_later(
        &self,
        reference: &Video<T>,
        length: usize,
    ) -> Result<Vec<Video<T>>, <Self as VideoRepository<T>>::Error>;
    /// Likeで並べてVideo<T>を`length`分取得する．
    async fn order_by_like(
        &self,
        length: usize,
    ) -> Result<Vec<Video<T>>, <Self as VideoRepository<T>>::Error>;
    /// Likeで並べて`reference`以降のVideo<T>を`length`分取得する．
    async fn order_by_like_later(
        &self,
        reference: &Video<T>,
        length: usize,
    ) -> Result<Vec<Video<T>>, <Self as VideoRepository<T>>::Error>;
    async fn remove(&self, id: VideoId) -> Result<(), <Self as VideoRepository<T>>::Error>;
}
