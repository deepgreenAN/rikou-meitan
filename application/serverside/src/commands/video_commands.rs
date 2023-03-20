use derive_new::new;
use domain::video::{Video, VideoId, VideoType};

#[derive(new)]
pub(crate) struct SaveVideoCommand<T: VideoType> {
    pub video: Video<T>,
}

#[derive(new)]
pub(crate) struct EditVideoCommand<T: VideoType> {
    pub video: Video<T>,
}

#[derive(new)]
pub(crate) struct IncrementLikeVideoCommand {
    pub id: VideoId,
}

#[derive(new)]
pub(crate) struct AllVideosCommand;

#[derive(new)]
pub(crate) struct OrderByLikeVideosCommand {
    pub length: usize,
}

#[derive(new)]
pub(crate) struct OrderByLikeLaterVideosCommand<T: VideoType> {
    pub reference: Video<T>,
    pub length: usize,
}

#[derive(new)]
pub(crate) struct OrderByDateVideosCommand {
    pub length: usize,
}

#[derive(new)]
pub(crate) struct OrderByDateLaterVideosCommand<T: VideoType> {
    pub reference: Video<T>,
    pub length: usize,
}

#[derive(new)]
pub(crate) struct RemoveVideoCommand {
    pub id: VideoId,
}
