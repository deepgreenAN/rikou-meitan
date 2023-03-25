use domain::movie_clip::{MovieClip, MovieClipId};
use domain::Date;

use derive_new::new;

#[derive(new)]
pub struct SaveMovieClipCommand<'a> {
    pub movie_clip: &'a MovieClip,
}

#[derive(new)]
pub struct EditMovieClipCommand<'a> {
    pub movie_clip: &'a MovieClip,
}

#[derive(new)]
pub struct IncrementLikeMovieClipCommand {
    pub id: MovieClipId,
}

pub struct AllMovieClipsCommand;

#[derive(new)]
pub struct OrderByLikeMovieClipsCommand {
    pub length: usize,
}

#[derive(new)]
pub struct OrderByLikeLaterMovieClipsCommand<'a> {
    pub reference: &'a MovieClip,
    pub length: usize,
}

#[derive(new)]
pub struct OrderByCreateDateRangeMovieClipsCommand {
    pub start: Date,
    pub end: Date,
}

#[derive(new)]
pub struct OrderByCreateDateMovieClipsCommand {
    pub length: usize,
}

#[derive(new)]
pub struct OrderByCreateDateLaterMovieClipsCommand<'a> {
    pub reference: &'a MovieClip,
    pub length: usize,
}

#[derive(new)]
pub struct RemoveMovieClipCommand {
    pub id: MovieClipId,
}
