use derive_new::new;
use domain::movie_clip::{MovieClip, MovieClipId};
use domain::Date;

#[derive(new)]
pub(crate) struct SaveMovieClipCommand {
    pub movie_clip: MovieClip,
}

#[derive(new)]
pub(crate) struct EditMovieClipCommand {
    pub movie_clip: MovieClip,
}

pub(crate) struct AllMovieClipCommand;

#[derive(new)]
pub(crate) struct OrderByLikeLimitMovieClipCommand {
    pub length: usize,
}

#[derive(new)]
pub(crate) struct OrderByCreateDateRangeMovieClipCommand {
    pub start: Date,
    pub end: Date,
}

#[derive(new)]
pub(crate) struct RemoveByIdMovieClipCommand {
    pub id: MovieClipId,
}
