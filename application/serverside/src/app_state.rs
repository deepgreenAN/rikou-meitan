use domain::video::{Kirinuki, Original};

use infrastructure::episode_repository_impl::{EpisodePgDBRepository, InMemoryEpisodeRepository};
use infrastructure::movie_clip_repository_impl::{
    InMemoryMovieClipRepository, MovieClipPgDBRepository,
};
use infrastructure::video_repository_impl::{InMemoryVideoRepository, VideoPgDbRepository};

#[cfg(test)]
use infrastructure::episode_repository_impl::MockEpisodeRepository;

#[cfg(test)]
use infrastructure::movie_clip_repository_impl::MockMovieClipRepository;

#[cfg(test)]
use infrastructure::video_repository_impl::{
    MockVideoKirinukiRepository, MockVideoOriginalRepository,
};

use std::sync::Arc;

pub struct AppState {
    pub movie_clip_repo: Arc<MovieClipPgDBRepository>,
    pub episode_repo: Arc<EpisodePgDBRepository>,
    pub original_repo: Arc<VideoPgDbRepository<Original>>,
    pub kirinuki_repo: Arc<VideoPgDbRepository<Kirinuki>>,
}

impl Clone for AppState {
    fn clone(&self) -> Self {
        Self {
            movie_clip_repo: Arc::clone(&self.movie_clip_repo),
            episode_repo: Arc::clone(&self.episode_repo),
            original_repo: Arc::clone(&self.original_repo),
            kirinuki_repo: Arc::clone(&self.kirinuki_repo),
        }
    }
}

#[derive(Default)]
pub struct InMemoryAppState {
    pub movie_clip_repo: Arc<InMemoryMovieClipRepository>,
    pub episode_repo: Arc<InMemoryEpisodeRepository>,
    pub original_repo: Arc<InMemoryVideoRepository<Original>>,
    pub kirinuki_repo: Arc<InMemoryVideoRepository<Kirinuki>>,
}

impl InMemoryAppState {
    pub fn new() -> Self {
        Self {
            movie_clip_repo: Arc::new(InMemoryMovieClipRepository::new()),
            episode_repo: Arc::new(InMemoryEpisodeRepository::new()),
            original_repo: Arc::new(InMemoryVideoRepository::<Original>::new()),
            kirinuki_repo: Arc::new(InMemoryVideoRepository::<Kirinuki>::new()),
        }
    }
}

impl Clone for InMemoryAppState {
    fn clone(&self) -> Self {
        Self {
            movie_clip_repo: Arc::clone(&self.movie_clip_repo),
            episode_repo: Arc::clone(&self.episode_repo),
            original_repo: Arc::clone(&self.original_repo),
            kirinuki_repo: Arc::clone(&self.kirinuki_repo),
        }
    }
}

#[cfg(test)]
#[derive(Default)]
pub struct MockAppState {
    pub movie_clip_repo: Arc<MockMovieClipRepository>,
    pub episode_repo: Arc<MockEpisodeRepository>,
    pub original_repo: Arc<MockVideoOriginalRepository>,
    pub kirinuki_repo: Arc<MockVideoKirinukiRepository>,
}

#[cfg(test)]
impl MockAppState {
    pub fn new() -> Self {
        Self {
            movie_clip_repo: Arc::new(MockMovieClipRepository::new()),
            episode_repo: Arc::new(MockEpisodeRepository::new()),
            original_repo: Arc::new(MockVideoOriginalRepository::new()),
            kirinuki_repo: Arc::new(MockVideoKirinukiRepository::new()),
        }
    }
}

#[cfg(test)]
impl Clone for MockAppState {
    fn clone(&self) -> Self {
        Self {
            movie_clip_repo: Arc::clone(&self.movie_clip_repo),
            episode_repo: Arc::clone(&self.episode_repo),
            original_repo: Arc::clone(&self.original_repo),
            kirinuki_repo: Arc::clone(&self.kirinuki_repo),
        }
    }
}
