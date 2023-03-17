mod db_movie_clip_repository;
mod inmemory_movie_clip_repository;
mod mock_movie_clip_repository;

pub use db_movie_clip_repository::MovieClipPgDBRepository;
pub use inmemory_movie_clip_repository::InMemoryMovieClipRepository;
pub use mock_movie_clip_repository::MockMovieClipRepository;
