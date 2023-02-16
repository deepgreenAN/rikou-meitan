use crate::date::Date;
use crate::ids::Id;
use crate::DomainError::{self, NotChangedError};
use serde::{Deserialize, Serialize};

#[cfg(feature = "server")]
use chrono::NaiveDate;

#[cfg(feature = "server")]
use sqlx::{postgres::PgRow, FromRow, Row};

#[cfg(feature = "server")]
use uuid::Uuid;

#[cfg(feature = "fake")]
use fake::{Dummy, Fake, Faker, StringFaker};

#[cfg(feature = "fake")]
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
    content: String,
    /// UUID
    id: EpisodeId,
}

impl Episode {
    /// プリミティブな値からのコンストラクタ
    pub fn new(date_ymd: (u32, u32, u32), content: String) -> Result<Self, DomainError> {
        Ok(Self {
            date: date_ymd.try_into()?,
            content,
            id: EpisodeId::generate(),
        })
    }
    /// ドメイン固有型からのコンストラクタ
    pub fn new_with_domains(date: Date, content: String) -> Self {
        Self {
            date,
            content,
            id: EpisodeId::generate(),
        }
    }
    /// 日時を編集
    pub fn edit_date(&mut self, new_date: Date) -> Result<(), DomainError> {
        if self.date == new_date {
            return Err(NotChangedError("date not changed".to_string()));
        }
        self.date = new_date;
        Ok(())
    }
    /// 内容を編集
    pub fn edit_content(&mut self, new_content: String) -> Result<(), DomainError> {
        if self.content == new_content {
            return Err(NotChangedError("content not changed".to_string()));
        }
        self.content = new_content;
        Ok(())
    }
    /// 日時を取得
    pub fn date(&self) -> Date {
        self.date
    }
    /// 内容を取得
    pub fn content(&self) -> &str {
        &self.content
    }
    /// idを取得
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

// -------------------------------------------------------------------------------------------------
// Dummy trait

#[cfg(feature = "fake")]
impl Dummy<Faker> for Episode {
    fn dummy_with_rng<R: Rng + ?Sized>(_config: &Faker, rng: &mut R) -> Self {
        let date = Faker.fake_with_rng::<Date, R>(rng);
        const CHAR: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZあいうえおかきくけこさしすせそたちつてとなにぬねのはひふへほまみむめもやゆよらりるれろわおん";

        let content = StringFaker::with(Vec::from(CHAR), 0..300).fake_with_rng::<String, R>(rng);
        Episode::new_with_domains(date, content)
    }
}

#[cfg(test)]
mod test {
    use super::Episode;
    use crate::DomainError::NotChangedError;
    use crate::{Date, DomainError};
    use assert_matches::assert_matches;
    use rstest::{fixture, rstest};

    #[fixture]
    fn episode() -> Episode {
        Episode::new((2022, 11, 22), "Some content".to_string()).unwrap()
    }

    #[rstest]
    #[test]
    fn episode_edits(mut episode: Episode) -> Result<(), DomainError> {
        let same_date = episode.date();
        let res_err = episode.edit_date(same_date);
        assert_matches!(res_err, Err(NotChangedError(_)));

        let res_ok = episode.edit_date(Date::from_ymd(2022, 11, 23)?);
        assert_matches!(res_ok, Ok(_));

        Ok(())
    }

    #[cfg(feature = "fake")]
    #[test]
    fn generate_fake() {
        use fake::{Fake, Faker};

        let _ = (0..10000)
            .map(|_| Faker.fake::<Episode>())
            .collect::<Vec<_>>();
    }
}
