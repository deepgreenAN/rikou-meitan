use crate::DomainError;
use ammonia::Builder;
use maplit::hashset;
use serde::{Deserialize, Serialize};
use std::{fmt::Display, str::FromStr};

#[cfg(any(test, feature = "fake"))]
use fake::{faker::lorem::en::Words, Dummy, Fake, Faker};

#[cfg(any(test, feature = "fake"))]
use rand::Rng;

/// エピソードの内容．htmlを含む
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(try_from = "String", into = "String")]
pub struct EpisodeContent(String);

impl Default for EpisodeContent {
    fn default() -> Self {
        Self("default episode content.".to_string())
    }
}

// 高コスト
impl FromStr for EpisodeContent {
    type Err = DomainError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut builder = Builder::new();
        builder
            .tags(hashset![
                "a", "br", "em", "ul", "li", "ol", "p", "strong", "table", "td", "tr", "th"
            ])
            .url_schemes(hashset!["https"])
            .link_rel(None);

        let out = builder.clean(s).to_string();
        if out == s {
            builder.link_rel(Some("noopener noreferrer"));
            Ok(Self(builder.clean(s).to_string()))
        } else {
            Err(DomainError::DomainParseError(
                "Contains invalid html".to_string(),
            ))
        }
    }
}

impl TryFrom<String> for EpisodeContent {
    type Error = DomainError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        value.parse()
    }
}

impl Display for EpisodeContent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<EpisodeContent> for String {
    fn from(value: EpisodeContent) -> Self {
        value.to_string()
    }
}

// -------------------------------------------------------------------------------------------------
// Dummy trait

#[cfg(any(test, feature = "fake"))]
impl Dummy<Faker> for EpisodeContent {
    fn dummy_with_rng<R: Rng + ?Sized>(_config: &Faker, rng: &mut R) -> Self {
        let content = Words(2..50).fake_with_rng::<Vec<String>, R>(rng).join(" ");
        content
            .try_into()
            .expect("Generate fake EpisodeContent Error")
    }
}

#[cfg(test)]
mod test {
    use super::DomainError;
    use super::EpisodeContent;
    use fake::{Fake, Faker};

    #[test]
    fn try_from_string() {
        let valid_html = "おりコウの歌ってみたである<a href=\"https://www.youtube.com/watch?v=B7OPlsdBuVc\"> きみも悪いひとでよかった </a> は <strong>いいぞ</strong>".to_string();

        let res: Result<EpisodeContent, DomainError> = valid_html.try_into();
        assert!(matches!(res, Ok(_)));

        let invalid_html = "おりコウの歌ってみたである<a href=\"https://www.youtube.com/watch?v=B7OPlsdBuVc\"> きみも悪いひとでよかった </a> は <strong>いいぞ</strong> <script>alert();</script>".to_string();

        let res: Result<EpisodeContent, DomainError> = invalid_html.try_into();
        assert!(matches!(res, Err(DomainError::DomainParseError(_))));
    }

    #[test]
    fn generate_fake() {
        let _ = (0..100)
            .map(|_| Faker.fake::<EpisodeContent>())
            .collect::<Vec<_>>(); // 高コスト
    }
}
