use domain::video::{Video, VideoId, VideoType};

use derive_new::new;

#[derive(new)]
pub struct SaveVideoCommand<'a, T: VideoType> {
    pub video: &'a Video<T>,
}

#[derive(new)]
pub struct EditVideoCommand<'a, T: VideoType> {
    pub video: &'a Video<T>,
}

#[derive(new)]
pub struct IncrementLikeVideoCommand {
    pub id: VideoId,
}

#[derive(new)]
pub struct AllVideosCommand;

#[derive(new)]
pub struct OrderByLikeVideosCommand {
    pub length: usize,
}

#[derive(new)]
pub struct OrderByLikeLaterVideosCommand<'a, T: VideoType> {
    pub reference: &'a Video<T>,
    pub length: usize,
}

#[derive(new)]
pub struct OrderByDateVideosCommand {
    pub length: usize,
}

#[derive(new)]
pub struct OrderByDateLaterVideosCommand<'a, T: VideoType> {
    pub reference: &'a Video<T>,
    pub length: usize,
}

#[derive(new)]
pub struct RemoveVideoCommand {
    pub id: VideoId,
}
