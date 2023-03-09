use std::{fmt::Display, str::FromStr};

pub use crate::MovieUrl;

use crate::date::Date;
use crate::ids::Id;
use crate::DomainError;

use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::marker::PhantomData;

#[cfg(feature = "server")]
use sqlx::{postgres::PgRow, FromRow, Row};

#[cfg(any(test, feature = "fake"))]
use fake::{faker::lorem::en::Words, Dummy, Fake, Faker};

#[cfg(any(test, feature = "fake"))]
use rand::Rng;

// -------------------------------------------------------------------------------------------------
// # VideoId

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct VideoIdType;

pub type VideoId = Id<VideoIdType>;

// -------------------------------------------------------------------------------------------------
// # VideoTypeの各種型

// -------------------------------------------------------------------
// ## Original

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Original;

impl FromStr for Original {
    type Err = DomainError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == stringify!(Original) {
            Ok(Original)
        } else {
            Err(DomainError::DomainParseError(
                "Invalid string for 'Original' video type".to_string(),
            ))
        }
    }
}

impl TryFrom<String> for Original {
    type Error = DomainError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        value.parse()
    }
}

impl Display for Original {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", stringify!(Original))
    }
}

impl From<Original> for String {
    fn from(value: Original) -> Self {
        value.to_string()
    }
}

// -------------------------------------------------------------------
// ## Kirinuki

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Kirinuki;

impl FromStr for Kirinuki {
    type Err = DomainError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == stringify!(Kirinuki) {
            Ok(Kirinuki)
        } else {
            Err(DomainError::DomainParseError(
                "Invalid string for Kirinuki video type".to_string(),
            ))
        }
    }
}

impl TryFrom<String> for Kirinuki {
    type Error = DomainError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        value.parse()
    }
}

impl Display for Kirinuki {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", stringify!(Kirinuki))
    }
}

impl From<Kirinuki> for String {
    fn from(value: Kirinuki) -> Self {
        value.to_string()
    }
}

// -------------------------------------------------------------------------------------------------
// serialize and deserialize function

fn serialize_phantom<S, T>(_: &PhantomData<T>, s: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
    T: Into<String> + Default,
{
    let type_str: String = T::default().into();
    s.serialize_str(&type_str)
}

fn deserialize_phantom<'de, D, T>(d: D) -> Result<PhantomData<T>, D::Error>
where
    D: Deserializer<'de>,
    T: TryFrom<String>,
{
    let unit_type_str: String = Deserialize::deserialize(d)?;
    let _unit_type: T = unit_type_str.try_into().map_err(|_| {
        serde::de::Error::custom(DomainError::DomainParseError(
            "Invalid string for video type".to_string(),
        ))
    })?;

    Ok(PhantomData)
}

// -------------------------------------------------------------------------------------------------
// # Video

/// Videoのエンティティ
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Video<T> {
    /// タイトル
    title: String,
    /// 動画のURL
    url: MovieUrl,
    /// id
    id: VideoId,
    /// 動画の配信日時
    date: Date,
    /// 動画の投稿者
    author: String,
    /// ライク数
    like: u32,
    /// 動画の種類(Kirinuki, Original)
    #[serde(
        serialize_with = "serialize_phantom",
        deserialize_with = "deserialize_phantom"
    )]
    #[serde(bound(serialize = "T: Into<String> + Default"))]
    #[serde(bound(deserialize = "T: TryFrom<String>"))]
    video_type: PhantomData<T>,
}

impl<T> Video<T> {
    /// プリミティブを用いたコンストラクタ
    pub fn new(
        title: String,
        url: String,
        date_ymd: (u32, u32, u32),
        author: String,
    ) -> Result<Video<T>, DomainError> {
        Ok(Self {
            title,
            url: url.try_into()?,
            id: VideoId::generate(),
            date: date_ymd.try_into()?,
            author,
            like: 0,
            video_type: PhantomData,
        })
    }
    /// ドメイン固有型を用いたコンストラクタ
    pub fn new_with_domains(title: String, url: MovieUrl, date: Date, author: String) -> Video<T> {
        Self {
            title,
            url,
            id: VideoId::generate(),
            date,
            author,
            like: 0,
            video_type: PhantomData,
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
    /// urlの可変参照を取得
    pub fn url_mut(&mut self) -> &mut MovieUrl {
        &mut self.url
    }
    /// idを取得
    pub fn id(&self) -> VideoId {
        self.id
    }
    /// dateを取得
    pub fn date(&self) -> Date {
        self.date
    }
    /// dateの可変参照を取得
    pub fn date_mut(&mut self) -> &mut Date {
        &mut self.date
    }
    /// likeを取得
    pub fn like(&self) -> u32 {
        self.like
    }
    /// likeを一つ増やす
    pub fn like_increment(&mut self) {
        self.like += 1;
    }
    /// authorを取得
    pub fn author(&self) -> &str {
        &self.author
    }
    /// authorの可変参照を取得
    pub fn author_mut(&mut self) -> &mut String {
        &mut self.author
    }
    /// id, likeはそのままにotherをコピー
    pub fn assign(&mut self, other: Self) {
        let new_self = Self {
            id: self.id,
            like: self.like,
            ..other
        };
        *self = new_self;
    }
}

// -------------------------------------------------------------------------------------------------
// Video as entity

#[cfg(feature = "server")]
impl<T> FromRow<'_, PgRow> for Video<T>
where
    T: TryFrom<String, Error = DomainError>,
{
    fn from_row(row: &PgRow) -> Result<Self, sqlx::Error> {
        use chrono::NaiveDate;
        use uuid::Uuid;

        let title: String = row.try_get("title")?;
        let url: String = row.try_get("url")?;
        let id: Uuid = row.try_get("id")?;
        let date: NaiveDate = row.try_get("date")?;
        let author: String = row.try_get("author")?;
        let like: i32 = row.try_get("like")?;
        let video_type_str: String = row.try_get("video_type")?;
        let _video_type: T = video_type_str.try_into()?;

        Ok(Self {
            title,
            url: url.try_into()?,
            id: id.into(),
            date: date.try_into()?,
            author,
            like: like as u32,
            video_type: PhantomData,
        })
    }
}

// -------------------------------------------------------------------------------------------------
// Dummy trait

#[cfg(any(feature = "fake", test))]
impl<T: Default> Dummy<Faker> for Video<T> {
    fn dummy_with_rng<R: Rng + ?Sized>(_config: &Faker, rng: &mut R) -> Self {
        let title = Words(2..50).fake_with_rng::<Vec<String>, R>(rng).join(" ");
        let author = Words(1..3).fake_with_rng::<Vec<String>, R>(rng).join(" ");

        let mut video: Video<T> = Video::new_with_domains(
            title,
            Faker.fake_with_rng(rng),
            Faker.fake_with_rng(rng),
            author,
        );

        let like_num = (0..1000).fake_with_rng::<usize, R>(rng);
        for _ in 0..like_num {
            video.like_increment();
        }
        video
    }
}

#[cfg(any(feature = "fake", test))]
impl<T: Default> Dummy<std::ops::Range<Date>> for Video<T> {
    fn dummy_with_rng<R: Rng + ?Sized>(config: &std::ops::Range<Date>, rng: &mut R) -> Self {
        let title = Words(2..50).fake_with_rng::<Vec<String>, R>(rng).join(" ");
        let author = Words(1..3).fake_with_rng::<Vec<String>, R>(rng).join(" ");

        let mut video: Video<T> = Video::new_with_domains(
            title,
            Faker.fake_with_rng(rng),
            config.fake_with_rng(rng),
            author,
        );

        let like_num = (0..1000).fake_with_rng::<usize, R>(rng);
        for _ in 0..like_num {
            video.like_increment();
        }
        video
    }
}

#[cfg(test)]
mod test {
    use super::{Kirinuki, Original, Video};
    use fake::{Fake, Faker};
    use pretty_assertions::assert_eq;

    #[test]
    fn test_like_increment() {
        let mut video: Video<Original> = Faker.fake();

        let like = video.like();
        video.like_increment();
        assert_eq!(like + 1, video.like());
    }

    #[test]
    fn test_parse_video_types() {
        let original: Original = "Original".parse().unwrap();
        assert_eq!(original, Original);

        assert_eq!(Original.to_string(), "Original".to_string());

        let kirinuki: Kirinuki = "Kirinuki".parse().unwrap();
        assert_eq!(kirinuki, Kirinuki);

        assert_eq!(Kirinuki.to_string(), "Kirinuki".to_string());
    }

    #[test]
    fn serialize_and_deserialize() {
        let video = Faker.fake::<Video<Original>>();
        let video_json = serde_json::to_string(&video).unwrap();
        assert_eq!(
            video,
            serde_json::from_str::<Video<Original>>(&video_json).unwrap()
        );

        let res_err = serde_json::from_str::<Video<Kirinuki>>(&video_json);
        assert!(matches!(res_err, Err(_)))
    }

    #[test]
    fn generate_fake() {
        let _ = (0..1000)
            .map(|_| Faker.fake::<Video<Kirinuki>>())
            .collect::<Vec<_>>();
    }
}
