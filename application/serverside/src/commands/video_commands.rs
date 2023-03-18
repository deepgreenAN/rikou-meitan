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

pub(crate) struct AllVideoCommand;

#[derive(new)]
pub(crate) struct OrderByLikeVideoCommand {
    pub length: usize,
}

#[derive(new)]
pub(crate) struct OrderByLikeLaterVideoCommand<T: VideoType> {
    pub reference: Video<T>,
    pub length: usize,
}

#[derive(new)]
pub(crate) struct OrderByDateVideoCommand {
    pub length: usize,
}

#[derive(new)]
pub(crate) struct OrderByDateLaterVideoCommand<T: VideoType> {
    pub reference: Video<T>,
    pub length: usize,
}

#[derive(new)]
pub(crate) struct RemoveVideoCommand {
    pub id: VideoId,
}
