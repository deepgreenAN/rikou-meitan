use crate::DomainError;
use config::CONFIG;
use serde::{Deserialize, Serialize};
use std::{fmt::Display, str::FromStr};

#[cfg(any(test, feature = "fake"))]
use fake::{Dummy, Fake, Faker, StringFaker};

#[cfg(any(test, feature = "fake"))]
use rand::Rng;

pub const MOVIE_URL_ALLOW_PREFIX: [&str; 2] = ["https://www.youtube.com/", "https://youtu.be/"];

/// MovieClipで用いるURL
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(try_from = "String", into = "String")]
pub struct MovieUrl {
    url_string: String, // 成型されたurl全体
    video_id: String,   // 動画プラットフォームの動画ID
}

impl MovieUrl {
    pub fn video_id(&self) -> &str {
        &self.video_id
    }
    pub fn from_url_str(url_str: &str) -> Result<Self, DomainError> {
        url_str.parse()
    }
}

impl FromStr for MovieUrl {
    type Err = DomainError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // 規定以上の長さの場合はエラーとなる
        if s.len() > CONFIG.url_string_lim {
            return Err(DomainError::DomainParseError(format!(
                "Url length must be less than {}",
                CONFIG.url_string_lim
            )));
        }

        let common_base_url = MOVIE_URL_ALLOW_PREFIX[0]; // 最終的なutl_strのベース

        if s.starts_with(MOVIE_URL_ALLOW_PREFIX[0]) {
            let prefix_string = format!("{}watch?", MOVIE_URL_ALLOW_PREFIX[0]); // watch?までの部分
            let query_str = s.strip_prefix(&prefix_string).ok_or_else(|| {
                DomainError::DomainParseError("Invalid query parameter".to_string())
            })?; // watch?以降の残りの部分

            let video_id_query = query_str
                .split('&')
                .find(|one_query| one_query.starts_with("v=")); // &で分割しv=で始まる部分
            let video_id = video_id_query
                .map(|video_id_query_str| video_id_query_str.strip_prefix("v=").unwrap()); // idの文字列
            match video_id {
                Some(video_id) => Ok(MovieUrl {
                    url_string: format!("{common_base_url}watch?v={video_id}"),
                    video_id: video_id.to_string(),
                }),
                None => Err(DomainError::DomainParseError(
                    "Invalid query parameter".to_string(),
                )),
            }
        } else if s.starts_with(MOVIE_URL_ALLOW_PREFIX[1]) {
            let prefix_str = MOVIE_URL_ALLOW_PREFIX[1];
            let mut query_str = s.strip_prefix(prefix_str).ok_or_else(|| {
                DomainError::DomainParseError("Invalid query parameter".to_string())
            })?;

            //watch?v=がある場合それを削除
            if query_str.starts_with("watch?v=") {
                query_str = query_str.strip_prefix("watch?v=").unwrap();
            }

            let video_id = query_str.split('?').next();
            match video_id {
                Some(video_id) => Ok(MovieUrl {
                    url_string: format!("{common_base_url}watch?v={video_id}"),
                    video_id: video_id.to_string(),
                }),
                None => Err(DomainError::DomainParseError(
                    "Invalid query parameter".to_string(),
                )),
            }
        } else {
            Err(DomainError::DomainParseError(format!(
                "Invalid Url. url must have prefix: {MOVIE_URL_ALLOW_PREFIX:?}"
            )))
        }
    }
}

impl TryFrom<String> for MovieUrl {
    type Error = DomainError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        value.parse()
    }
}

impl From<MovieUrl> for String {
    fn from(value: MovieUrl) -> Self {
        value.to_string()
    }
}

impl Display for MovieUrl {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.url_string)
    }
}

// -------------------------------------------------------------------------------------------------
// Dummy trait

#[cfg(any(test, feature = "fake"))]
impl Dummy<Faker> for MovieUrl {
    fn dummy_with_rng<R: Rng + ?Sized>(_config: &Faker, rng: &mut R) -> Self {
        let mut url = "https://www.youtube.com/watch?v=".to_string();
        const ALPHANUMERIC: &str = "0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
        let fake_video_id =
            StringFaker::with(Vec::from(ALPHANUMERIC), 11).fake_with_rng::<String, R>(rng);
        url.push_str(&fake_video_id);

        url.try_into().expect("Generate fake error.")
    }
}

#[cfg(test)]
mod test {
    use super::MovieUrl;
    use pretty_assertions::assert_eq;

    #[test]
    fn parse_url() {
        // 基本的なurlを与えた場合
        let movie_url =
            MovieUrl::from_url_str("https://www.youtube.com/watch?v=LjU5OOHu_As").unwrap();
        assert_eq!(
            "https://www.youtube.com/watch?v=LjU5OOHu_As",
            movie_url.to_string()
        );
        assert_eq!("LjU5OOHu_As", movie_url.video_id());

        // 基本的なurlを与えた場合2
        let movie_url = MovieUrl::from_url_str("https://youtu.be/LjU5OOHu_As").unwrap();
        assert_eq!(
            "https://www.youtube.com/watch?v=LjU5OOHu_As",
            movie_url.to_string()
        );
        assert_eq!("LjU5OOHu_As", movie_url.video_id());

        // その他のクエリパラメーターがある場合．
        let movie_url =
            MovieUrl::from_url_str("https://www.youtube.com/watch?v=LjU5OOHu_As&t=100s").unwrap();
        assert_eq!(
            "https://www.youtube.com/watch?v=LjU5OOHu_As",
            movie_url.to_string()
        );
        assert_eq!("LjU5OOHu_As", movie_url.video_id());

        let movie_url = MovieUrl::from_url_str("https://youtu.be/LjU5OOHu_As?t=100s").unwrap();
        assert_eq!(
            "https://www.youtube.com/watch?v=LjU5OOHu_As",
            movie_url.to_string()
        );
        assert_eq!("LjU5OOHu_As", movie_url.video_id());
    }

    #[test]
    fn from_str() {
        let movie_url = "https://www.youtube.com/watch?v=LjU5OOHu_As"
            .parse::<MovieUrl>()
            .unwrap();
        assert_eq!(
            "https://www.youtube.com/watch?v=LjU5OOHu_As",
            movie_url.to_string()
        );
        assert_eq!("LjU5OOHu_As", movie_url.video_id());
    }

    #[test]
    fn serialize_and_deserialize() {
        let movie_url =
            MovieUrl::from_url_str("https://www.youtube.com/watch?v=LjU5OOHu_As").unwrap();
        let json_url = serde_json::to_string(&movie_url).unwrap();
        assert_eq!(
            r#""https://www.youtube.com/watch?v=LjU5OOHu_As""#.to_string(),
            json_url
        );

        let json_url = r#""https://www.youtube.com/watch?v=LjU5OOHu_As&t=100s""#.to_string();
        let movie_url = serde_json::from_str::<MovieUrl>(&json_url).unwrap();

        assert_eq!(
            "https://www.youtube.com/watch?v=LjU5OOHu_As".to_string(),
            Into::<String>::into(movie_url)
        );
    }

    #[cfg(feature = "fake")]
    #[test]
    fn generate_fake() {
        use fake::{Fake, Faker};

        let _ = (0..10000)
            .map(|_| Faker.fake::<MovieUrl>())
            .collect::<Vec<_>>();
    }
}
