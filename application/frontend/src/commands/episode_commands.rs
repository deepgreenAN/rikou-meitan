use domain::episode::{Episode, EpisodeId};
use domain::Date;

pub struct SaveEpisodeCommand {
    pub episode: Episode,
}

impl SaveEpisodeCommand {
    pub fn new(episode: Episode) -> Self {
        Self { episode }
    }
}

pub struct EditEpisodeCommand {
    pub episode: Episode,
}

impl EditEpisodeCommand {
    pub fn new(episode: Episode) -> Self {
        Self { episode }
    }
}

pub struct AllEpisodeCommand;

pub struct OrderByDateRangeEpisodeCommand {
    pub start: Date,
    pub end: Date,
}

impl OrderByDateRangeEpisodeCommand {
    pub fn new(start: Date, end: Date) -> Self {
        Self { start, end }
    }
}

pub struct RemoveByIdEpisodeCommand {
    pub id: EpisodeId,
}

impl RemoveByIdEpisodeCommand {
    pub fn new(id: EpisodeId) -> Self {
        Self { id }
    }
}
