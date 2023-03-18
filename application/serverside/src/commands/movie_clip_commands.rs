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

#[derive(new)]
pub(crate) struct IncrementLikeMovieClipCommand {
    pub id: MovieClipId,
}

pub(crate) struct AllMovieClipCommand;

#[derive(new)]
pub(crate) struct OrderByLikeMovieClipCommand {
    pub length: usize,
}

#[derive(new)]
pub(crate) struct OrderByLikeLaterMovieClipCommand {
    pub reference: MovieClip,
    pub length: usize,
}

#[derive(new)]
pub(crate) struct OrderByCreateDateRangeMovieClipCommand {
    pub start: Date,
    pub end: Date,
}

#[derive(new)]
pub(crate) struct OrderByCreateDateMovieClipCommand {
    pub length: usize,
}

#[derive(new)]
pub(crate) struct OrderByCreateDateLaterMovieClipCommand {
    pub reference: MovieClip,
    pub length: usize,
}

#[derive(new)]
pub(crate) struct RemoveMovieClipCommand {
    pub id: MovieClipId,
}
