mod movie_url;
mod second;

pub use movie_url::MovieUrl;
pub use second::Second;
pub use second::SecondRange;

use crate::date::Date;
use crate::ids::Id;
use crate::DomainError::{self, DomainLogicError};
use serde::{Deserialize, Serialize};

#[cfg(feature = "server")]
use sqlx::{postgres::PgRow, FromRow, Row};

#[cfg(feature = "server")]
use uuid::Uuid;

#[cfg(feature = "server")]
use chrono::NaiveDate;

#[cfg(any(test, feature = "fake"))]
use fake::{faker::lorem::en::Words, Dummy, Fake, Faker};

// -------------------------------------------------------------------------------------------------
// # ClipId

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct MovieClipIdType;

/// MovieClipに対応するID
pub type MovieClipId = Id<MovieClipIdType>;

// -------------------------------------------------------------------------------------------------
// # VideoClip

/// VideoClipのエンティティ
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MovieClip {
    title: String,
    url: MovieUrl,
    range: SecondRange,
    id: MovieClipId,
    like: u32,
    create_date: Date,
}

impl MovieClip {
    /// プリミティブを用いたコンストラクタ．
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
            range: (start..end).try_into()?,
            id: MovieClipId::generate(),
            like: 0_u32,
            create_date: create_date_ymd.try_into()?,
        })
    }

    /// ドメイン固有型を用いたコンストラクタ
    pub fn new_with_domains(
        title: String,
        url: MovieUrl,
        range: SecondRange,
        create_date: Date,
    ) -> Self {
        Self {
            title,
            url,
            range,
            create_date,
            like: 0_u32,
            id: MovieClipId::generate(),
        }
    }
    /// titleを取得
    pub fn title(&self) -> &str {
        &self.title
    }
    /// titleの可変参照を取得
    pub fn title_mut(&mut self) -> &mut String {
        &mut self.title
    }
    /// urlを取得
    pub fn url(&self) -> &MovieUrl {
        &self.url
    }
    /// rangeを取得
    pub fn range(&self) -> &SecondRange {
        &self.range
    }
    /// rangeの可変参照を取得
    pub fn range_mut(&mut self) -> &mut SecondRange {
        &mut self.range
    }
    /// idを取得
    pub fn id(&self) -> MovieClipId {
        self.id
    }
    /// likeを取得
    pub fn like(&self) -> u32 {
        self.like
    }
    /// likeを一つ増やす
    pub fn like_increment(&mut self) {
        self.like += 1;
    }
    /// create_dateを取得
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
            range: (start as u32..end as u32).try_into()?,
            id: id.into(),
            like: like as u32,
            create_date: create_date.try_into()?,
        })
    }
}

// -------------------------------------------------------------------------------------------------
// Dummy trait

#[cfg(any(feature = "fake", test))]
impl Dummy<Faker> for MovieClip {
    fn dummy_with_rng<R: rand::Rng + ?Sized>(_config: &Faker, rng: &mut R) -> Self {
        let title = Words(2..50).fake_with_rng::<Vec<String>, R>(rng).join(" ");

        Self::new_with_domains(
            title,
            Faker.fake_with_rng(rng),
            Faker.fake_with_rng(rng),
            Faker.fake_with_rng(rng),
        )
    }
}

#[cfg(test)]
mod test {
    use super::MovieClip;
    use fake::{Fake, Faker};

    #[test]
    fn movie_clip_like_increment() {
        let mut movie_clip = Faker.fake::<MovieClip>();
        let like = movie_clip.like();

        movie_clip.like_increment();
        assert_eq!(like + 1, movie_clip.like());
    }

    #[cfg(feature = "fake")]
    #[test]
    fn generate_fake() {
        let _ = (0..10000)
            .map(|_| Faker.fake::<MovieClip>())
            .collect::<Vec<_>>();
    }
}
