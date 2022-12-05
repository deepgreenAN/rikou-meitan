use infrastructure::episode_repository_impl::{EpisodePgDBRepository, InMemoryEpisodeRepository};
use infrastructure::movie_clip_repository_impl::{
    InMemoryMovieClipRepository, MovieClipPgDBRepository,
};

#[cfg(test)]
use infrastructure::episode_repository_impl::MockEpisodeRepositoryImpl;

#[cfg(test)]
use infrastructure::movie_clip_repository_impl::MockMovieClipRepositoryImpl;

use std::sync::Arc;

pub struct AppState {
    pub movie_clip_repo: Arc<MovieClipPgDBRepository>,
    pub episode_repo: Arc<EpisodePgDBRepository>,
}

impl Clone for AppState {
    fn clone(&self) -> Self {
        Self {
            movie_clip_repo: Arc::clone(&self.movie_clip_repo),
            episode_repo: Arc::clone(&self.episode_repo),
        }
    }
}

pub struct InMemoryAppState {
    pub movie_clip_repo: Arc<InMemoryMovieClipRepository>,
    pub episode_repo: Arc<InMemoryEpisodeRepository>,
}

impl Clone for InMemoryAppState {
    fn clone(&self) -> Self {
        Self {
            movie_clip_repo: Arc::clone(&self.movie_clip_repo),
            episode_repo: Arc::clone(&self.episode_repo),
        }
    }
}

#[cfg(test)]
#[derive(Default)]
pub struct MockAppState {
    pub movie_clip_repo: Arc<MockMovieClipRepositoryImpl>,
    pub episode_repo: Arc<MockEpisodeRepositoryImpl>,
}

#[cfg(test)]
impl MockAppState {
    pub fn new() -> Self {
        Self {
            movie_clip_repo: Arc::new(MockMovieClipRepositoryImpl::new()),
            episode_repo: Arc::new(MockEpisodeRepositoryImpl::new()),
        }
    }
}

#[cfg(test)]
impl Clone for MockAppState {
    fn clone(&self) -> Self {
        Self {
            movie_clip_repo: Arc::clone(&self.movie_clip_repo),
            episode_repo: Arc::clone(&self.episode_repo),
        }
    }
}
