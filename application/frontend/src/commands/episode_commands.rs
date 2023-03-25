use domain::episode::{Episode, EpisodeId};
use domain::Date;

use derive_new::new;

#[derive(new)]
pub struct SaveEpisodeCommand<'a> {
    pub episode: &'a Episode,
}

#[derive(new)]
pub struct EditEpisodeCommand<'a> {
    pub episode: &'a Episode,
}

pub struct AllEpisodesCommand;

#[derive(new)]
pub struct OrderByDateRangeEpisodesCommand {
    pub start: Date,
    pub end: Date,
}

#[derive(new)]
pub struct RemoveEpisodeCommand {
    pub id: EpisodeId,
}
