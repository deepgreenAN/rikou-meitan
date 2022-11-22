mod movie_url;
mod second;

pub use movie_url::MovieUrl;
pub use second::Second;

use crate::date::Date;
use crate::ids::Id;
use crate::DomainError::{self, DomainLogicError, NotChangedError};

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
    pub fn edit_title(&mut self, new_title: String) -> Result<(), DomainError> {
        if self.title == new_title {
            return Err(NotChangedError("title not changed".to_string()));
        }
        self.title = new_title;
        Ok(())
    }
    pub fn edit_start(&mut self, new_start: Second) -> Result<(), DomainError> {
        if self.start == new_start {
            return Err(NotChangedError("start not changed".to_string()));
        }
        if new_start >= self.end {
            return Err(DomainLogicError("start must be less than end".to_string()));
        }
        self.start = new_start;
        Ok(())
    }
    pub fn edit_end(&mut self, new_end: Second) -> Result<(), DomainError> {
        if self.end == new_end {
            return Err(NotChangedError("end not changed".to_string()));
        }
        if new_end <= self.start {
            return Err(DomainLogicError(
                "end must be larger than start".to_string(),
            ));
        }
        self.end = new_end;
        Ok(())
    }
    pub fn edit_start_and_end(
        &mut self,
        new_start: Second,
        new_end: Second,
    ) -> Result<(), DomainError> {
        if self.start == new_start && self.end == new_end {
            return Err(NotChangedError("start and end not changed".to_string()));
        }
        if new_start >= new_end {
            return Err(DomainLogicError("It must be start < end".to_string()));
        }
        self.start = new_start;
        self.end = new_end;
        Ok(())
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
    use crate::DomainError::{DomainLogicError, NotChangedError};
    use assert_matches::assert_matches;

    use super::MovieClip;
    use rstest::{fixture, rstest};
    #[fixture]
    fn movie_clip() -> MovieClip {
        MovieClip::new(
            "Some Movie Clip".to_string(),
            "https://www.youtube.com/watch?v=SOMEvideoID".to_string(),
            500,
            600,
            (2018, 6, 20),
        )
        .unwrap()
    }

    #[rstest]
    #[test]
    fn movie_clip_like_increment(mut movie_clip: MovieClip) {
        assert_eq!(movie_clip.like(), 0);
        movie_clip.like_increment();
        assert_eq!(movie_clip.like(), 1);
    }

    #[rstest]
    #[test]
    fn movie_clip_edits(mut movie_clip: MovieClip) {
        // edit_title
        let same_title = movie_clip.title().to_string();
        let res_err = movie_clip.edit_title(same_title);
        assert_matches!(res_err, Err(NotChangedError(_)));

        let res_ok = movie_clip.edit_title("Another Movie Clip".to_string());
        assert_matches!(res_ok, Ok(_));

        // edit_start
        let same_start = movie_clip.start();
        let res_err = movie_clip.edit_start(same_start);
        assert_matches!(res_err, Err(NotChangedError(_)));

        let res_err = movie_clip.edit_start(700.into());
        assert_matches!(res_err, Err(DomainLogicError(_)));

        let res_ok = movie_clip.edit_start(300.into());
        assert_matches!(res_ok, Ok(_));

        // edit_end
        let same_end = movie_clip.end();
        let res_err = movie_clip.edit_end(same_end);
        assert_matches!(res_err, Err(NotChangedError(_)));

        let res_err = movie_clip.edit_end(200.into());
        assert_matches!(res_err, Err(DomainLogicError(_)));

        let res_ok = movie_clip.edit_end(800.into());
        assert_matches!(res_ok, Ok(_));

        // edit_start_and_end
        let same_start = movie_clip.start();
        let same_end = movie_clip.end();
        let res_err = movie_clip.edit_start_and_end(same_start, same_end);
        assert_matches!(res_err, Err(NotChangedError(_)));

        let res_err = movie_clip.edit_start_and_end(500.into(), 400.into());
        assert_matches!(res_err, Err(DomainLogicError(_)));

        let res_ok = movie_clip.edit_start_and_end(400.into(), 500.into());
        assert_matches!(res_ok, Ok(_));
    }
}
