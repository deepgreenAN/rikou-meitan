mod episode_content;

use crate::date::Date;
use crate::ids::Id;
use crate::DomainError;
pub use episode_content::EpisodeContent;
use serde::{Deserialize, Serialize};

#[cfg(feature = "server")]
use chrono::NaiveDate;

#[cfg(feature = "server")]
use sqlx::{postgres::PgRow, FromRow, Row};

#[cfg(feature = "server")]
use uuid::Uuid;

#[cfg(any(test, feature = "fake"))]
use fake::{Dummy, Fake, Faker};

#[cfg(any(test, feature = "fake"))]
use rand::Rng;

// -------------------------------------------------------------------------------------------------
// # EpisodeId

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct EpisodeIdType;

pub type EpisodeId = Id<EpisodeIdType>;

// -------------------------------------------------------------------------------------------------
// # Episode

/// Episodeのエンティティ
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Episode {
    /// エピソードの日時
    date: Date,
    /// エピソードの内容
    content: EpisodeContent,
    /// UUID
    id: EpisodeId,
}

impl Episode {
    /// プリミティブな値からのコンストラクタ
    pub fn new(date_ymd: (u32, u32, u32), content: String) -> Result<Self, DomainError> {
        Ok(Self {
            date: date_ymd.try_into()?,
            content: content.try_into()?,
            id: EpisodeId::generate(),
        })
    }
    /// ドメイン固有型からのコンストラクタ
    pub fn new_with_domains(date: Date, content: EpisodeContent) -> Self {
        Self {
            date,
            content,
            id: EpisodeId::generate(),
        }
    }
    /// 日時を取得
    pub fn date(&self) -> Date {
        self.date
    }
    /// 内容を取得
    pub fn content(&self) -> &EpisodeContent {
        &self.content
    }
    /// idを取得
    pub fn id(&self) -> EpisodeId {
        self.id
    }
    /// 日時の可変参照を取得
    pub fn date_mut(&mut self) -> &mut Date {
        &mut self.date
    }
    /// 内容を編集
    pub fn content_mut(&mut self) -> &mut EpisodeContent {
        &mut self.content
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
            content: content.try_into()?,
            id: id.into(),
        })
    }
}

// -------------------------------------------------------------------------------------------------
// Dummy trait

#[cfg(any(test, feature = "fake"))]
impl Dummy<Faker> for Episode {
    fn dummy_with_rng<R: Rng + ?Sized>(_config: &Faker, rng: &mut R) -> Self {
        Self::new_with_domains(Faker.fake_with_rng(rng), Faker.fake_with_rng(rng))
    }
}

#[cfg(any(test, feature = "fake"))]
impl Dummy<std::ops::Range<Date>> for Episode {
    fn dummy_with_rng<R: Rng + ?Sized>(config: &std::ops::Range<Date>, rng: &mut R) -> Self {
        Self::new_with_domains(config.fake_with_rng(rng), Faker.fake_with_rng(rng))
    }
}

#[cfg(test)]
mod test {
    use super::{Episode, EpisodeContent};
    use crate::Date;
    use fake::{Fake, Faker};
    use pretty_assertions::assert_eq;

    #[test]
    fn episode() {
        Episode::new((2022, 11, 22), "Some content".to_string()).unwrap();
        let date = Faker.fake::<Date>();
        let content = Faker.fake::<EpisodeContent>();
        Episode::new_with_domains(date, content);
    }

    #[test]
    fn modify_episode() {
        let mut episode = Faker.fake::<Episode>();
        let new_date = Faker.fake::<Date>();

        *episode.date_mut() = new_date;
        assert_eq!(episode.date(), new_date);

        let new_content = Faker.fake::<EpisodeContent>();

        *episode.content_mut() = new_content.clone();
        assert_eq!(episode.content().clone(), new_content);
    }

    #[test]
    fn generate_fake() {
        let _ = (0..10000)
            .map(|_| Faker.fake::<Episode>())
            .collect::<Vec<_>>();
    }
}
