mod episode_content;

pub use episode_content::EpisodeContent;

use crate::date::Date;
use crate::ids::Id;
use crate::DomainError;

// -------------------------------------------------------------------------------------------------
// # EpisodeId

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct EpisodeIdType;

pub type EpisodeId = Id<EpisodeIdType>;

// -------------------------------------------------------------------------------------------------
// # Episode

/// Episodeのエンティティ
#[derive(Debug, Clone)]
pub struct Episode {
    date: Date,
    content: EpisodeContent,
    id: EpisodeId,
}

impl Episode {
    pub fn new(date_ymd: (u32, u32, u32), content: String) -> Result<Self, DomainError> {
        Ok(Self {
            date: date_ymd.try_into()?,
            content: content.try_into()?,
            id: Default::default(),
        })
    }
    pub fn date(&self) -> &Date {
        &self.date
    }
    pub fn content(&self) -> &EpisodeContent {
        &self.content
    }
    pub fn id(&self) -> EpisodeId {
        self.id
    }
}
