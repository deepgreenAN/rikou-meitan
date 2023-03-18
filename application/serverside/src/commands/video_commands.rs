use derive_new::new;
use domain::video::{Video, VideoId};

#[derive(new)]
pub(crate) struct SaveVideoCommand<T> {
    pub video: Video<T>,
}

#[derive(new)]
pub(crate) struct EditVideoCommand<T> {
    pub video: Video<T>,
}

#[derive(new)]
pub(crate) struct IncrementLikeVideoCommnd {
    pub id: VideoId,
}

pub(crate) struct AllVideoCommand;

#[derive(new)]
pub(crate) struct OrderByLikeVideoCommand {
    pub length: usize,
}

#[derive(new)]
pub(crate) struct OrderByLikeLaterVideoCommand<T> {
    pub reference: Video<T>,
    pub length: usize,
}

#[derive(new)]
pub(crate) struct OrderByDateVideoCommand {
    pub length: usize,
}

#[derive(new)]
pub(crate) struct OrderByDteLaterVideoCommand<T> {
    pub reference: Video<T>,
    pub length: usize,
}

#[derive(new)]
pub(crate) struct RemoveVideoCommand {
    pub id: VideoId,
}
