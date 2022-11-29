use derive_new::new;
use domain::episode::{Episode, EpisodeId};
use domain::Date;

#[derive(new)]
pub(crate) struct SaveEpisodeCommand {
    pub episode: Episode,
}

#[derive(new)]
pub(crate) struct EditEpisodeCommand {
    pub episode: Episode,
}

pub(crate) struct AllEpisodeCommand;

#[derive(new)]
pub(crate) struct OrderByDateRangeEpisodeCommand {
    pub start: Date,
    pub end: Date,
}

#[derive(new)]
pub(crate) struct RemoveByIdEpisodeCommand {
    pub id: EpisodeId,
}
