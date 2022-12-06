use domain::movie_clip::{MovieClip, MovieClipId};
use domain::Date;

pub struct SaveMovieClipCommand {
    pub movie_clip: MovieClip,
}

impl SaveMovieClipCommand {
    pub fn new(movie_clip: MovieClip) -> Self {
        Self { movie_clip }
    }
}

pub struct EditMovieClipCommand {
    pub movie_clip: MovieClip,
}

impl EditMovieClipCommand {
    pub fn new(movie_clip: MovieClip) -> Self {
        Self { movie_clip }
    }
}

pub struct AllMovieClipCommand;

pub struct OrderByLikeLimitMovieClipCommand {
    pub length: usize,
}

impl OrderByLikeLimitMovieClipCommand {
    pub fn new(length: usize) -> Self {
        Self { length }
    }
}

pub struct OrderByCreateDateRangeMovieClipCommand {
    pub start: Date,
    pub end: Date,
}

impl OrderByCreateDateRangeMovieClipCommand {
    pub fn new(start: Date, end: Date) -> Self {
        Self { start, end }
    }
}

pub struct RemoveMovieClipCommand {
    pub id: MovieClipId,
}

impl RemoveMovieClipCommand {
    pub fn new(id: MovieClipId) -> Self {
        Self { id }
    }
}
