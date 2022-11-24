use derive_new::new;
use domain::episode::{Episode, EpisodeId};
use domain::Date;

#[derive(new)]
pub struct SaveEpisodeCommand {
    pub episode: Episode,
}

#[derive(new)]
pub struct EditEpisodeCommand {
    pub episode: Episode,
}

pub struct AllEpisodeCommand;

#[derive(new)]
pub struct OrderByDateRangeEpisodeCommand {
    pub start: Date,
    pub end: Date,
}

#[derive(new)]
pub struct RemoveByIdEpisodeCommand {
    pub id: EpisodeId,
}
