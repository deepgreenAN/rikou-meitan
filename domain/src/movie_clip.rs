mod movie_url;
mod second;

pub use movie_url::MovieUrl;
pub use second::Second;

use crate::date::Date;
use crate::ids::Id;
use crate::DomainError::{self, DomainLogicError};

#[cfg(feature = "server")]
use sqlx::{postgres::PgRow, FromRow, Row};

#[cfg(feature = "server")]
use uuid::Uuid;

#[cfg(feature = "server")]
use chrono::NaiveDate;

// -------------------------------------------------------------------------------------------------
// # ClipId

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct MovieClipIdType;

/// MovieClipに対応するID
pub type MovieClipId = Id<MovieClipIdType>;

// -------------------------------------------------------------------------------------------------
// # VideoClip

/// VideoClipのエンティティ
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MovieClip {
    title: String,
    url: MovieUrl,
    start: Second,
    end: Second,
    id: MovieClipId,
    like: u32,
    create_date: Date,
}

impl MovieClip {
    /// 新しくVideoClipを作成する．
    pub fn new(
        title: String,
        url: String,
        start: u32,
        end: u32,
        create_date_ymd: (u32, u32, u32),
    ) -> Result<Self, DomainError> {
        if start >= end {
            return Err(DomainLogicError("It is needed start < end".to_string()));
        }
        Ok(Self {
            title,
            url: url.try_into()?,
            start: start.into(),
            end: end.into(),
            id: MovieClipId::generate(),
            like: 0_u32,
            create_date: create_date_ymd.try_into()?,
        })
    }

    pub fn like_increment(&mut self) {
        self.like += 1;
    }
    pub fn edit_title(&mut self, new_title: String) {
        self.title = new_title;
    }
    pub fn title(&self) -> &str {
        &self.title
    }
    pub fn url(&self) -> &MovieUrl {
        &self.url
    }
    pub fn start(&self) -> Second {
        self.start
    }
    pub fn end(&self) -> Second {
        self.end
    }
    pub fn id(&self) -> MovieClipId {
        self.id
    }
    pub fn like(&self) -> u32 {
        self.like
    }
    pub fn create_date(&self) -> Date {
        self.create_date
    }
}

// -------------------------------------------------------------------------------------------------
// MovieClip as entity

#[cfg(feature = "server")]
impl FromRow<'_, PgRow> for MovieClip {
    fn from_row(row: &PgRow) -> Result<Self, sqlx::Error> {
        let title: String = row.try_get("title")?;
        let url: String = row.try_get("url")?;
        let start: i32 = row.try_get("start")?;
        let end: i32 = row.try_get("end")?;
        let id: Uuid = row.try_get("id")?;
        let like: i32 = row.try_get("like")?;
        let create_date: NaiveDate = row.try_get("create_date")?;

        Ok(Self {
            title,
            url: url.try_into()?,
            start: start.into(),
            end: end.into(),
            id: id.into(),
            like: like as u32,
            create_date: create_date.try_into()?,
        })
    }
}

#[cfg(test)]
mod test {
    use super::MovieClip;
    #[test]
    fn movie_clip_like_dislike() {
        let mut movie_clip = MovieClip::new(
            "Some Movie Clip".to_string(),
            "https://www.youtube.com/watch?v=SOMEvideoID".to_string(),
            500,
            600,
            (2018, 6, 20),
        )
        .unwrap();

        assert_eq!(movie_clip.like(), 0);
        movie_clip.like_increment();
        assert_eq!(movie_clip.like(), 1);
    }
}
