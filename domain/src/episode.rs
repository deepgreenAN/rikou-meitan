use crate::date::Date;
use crate::ids::Id;
use crate::DomainError;

#[cfg(feature = "server")]
use chrono::NaiveDate;

#[cfg(feature = "server")]
use sqlx::{postgres::PgRow, FromRow, Row};

#[cfg(feature = "server")]
use uuid::Uuid;

// -------------------------------------------------------------------------------------------------
// # EpisodeId

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct EpisodeIdType;

pub type EpisodeId = Id<EpisodeIdType>;

// -------------------------------------------------------------------------------------------------
// # Episode

/// Episodeのエンティティ
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Episode {
    date: Date,
    content: String,
    id: EpisodeId,
}

impl Episode {
    pub fn new(date_ymd: (u32, u32, u32), content: String) -> Result<Self, DomainError> {
        Ok(Self {
            date: date_ymd.try_into()?,
            content,
            id: EpisodeId::generate(),
        })
    }
    pub fn date(&self) -> Date {
        self.date
    }
    pub fn content(&self) -> &str {
        &self.content
    }
    pub fn id(&self) -> EpisodeId {
        self.id
    }
}

// -------------------------------------------------------------------------------------------------
// Episode as entity

#[cfg(feature = "server")]
impl FromRow<'_, PgRow> for Episode {
    fn from_row(row: &PgRow) -> Result<Self, sqlx::Error> {
        let date: NaiveDate = row.try_get("date")?;
        let content: String = row.try_get("content")?;
        let id: Uuid = row.try_get("id")?;

        Ok(Self {
            date: date.try_into()?,
            content,
            id: id.into(),
        })
    }
}
