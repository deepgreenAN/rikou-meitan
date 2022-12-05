use derive_new::new;
use domain::movie_clip::{MovieClip, MovieClipId};
use domain::Date;

#[derive(new)]
pub struct SaveMovieClipCommand {
    pub movie_clip: MovieClip,
}

#[derive(new)]
pub struct EditMovieClipCommand {
    pub movie_clip: MovieClip,
}

pub struct AllMovieClipCommand;

#[derive(new)]
pub struct OrderByLikeLimitMovieClipCommand {
    pub length: usize,
}

#[derive(new)]
pub struct OrderByCreateDateRangeMovieClipCommand {
    pub start: Date,
    pub end: Date,
}

#[derive(new)]
pub struct RemoveMovieClipCommand {
    pub id: MovieClipId,
}
