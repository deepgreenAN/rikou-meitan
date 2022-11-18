use crate::DomainError::{self, UrlParseError};
use std::{fmt::Display, str::FromStr};

use config::CONFIG;

pub const MOVIE_URL_ALLOW_PREFIX: [&str; 2] = ["https://www.youtube.com/", "https://youtu.be/"];

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MovieUrl {
    url_string: String, // 成型されたurl全体
    video_id: String,   // 動画プラットフォームの動画ID
}

impl MovieUrl {
    pub fn url_str(&self) -> &str {
        &self.url_string
    }
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
            return Err(UrlParseError(format!(
                "Url length must be less than {}",
                CONFIG.url_string_lim
            )));
        }

        let common_base_url = MOVIE_URL_ALLOW_PREFIX[0]; // 最終的なutl_strのベース

        if s.starts_with(MOVIE_URL_ALLOW_PREFIX[0]) {
            let prefix_string = format!("{}watch?", MOVIE_URL_ALLOW_PREFIX[0]); // watch?までの部分
            let query_str = s
                .strip_prefix(&prefix_string)
                .ok_or_else(|| UrlParseError("Invalid query parameter".to_string()))?; // watch?以降の残りの部分

            let video_id_query = query_str
                .split('&')
                .find(|one_query| one_query.starts_with("v=")); // &で分割しv=で始まる部分
            let video_id = video_id_query
                .map(|video_id_query_str| video_id_query_str.strip_prefix("v=").unwrap()); // idの文字列
            match video_id {
                Some(video_id) => Ok(MovieUrl {
                    url_string: format!("{}watch?v={}", common_base_url, video_id),
                    video_id: video_id.to_string(),
                }),
                None => Err(UrlParseError("Invalid query parameter".to_string())),
            }
        } else if s.starts_with(MOVIE_URL_ALLOW_PREFIX[1]) {
            let prefix_str = MOVIE_URL_ALLOW_PREFIX[1];
            let mut query_str = s
                .strip_prefix(prefix_str)
                .ok_or_else(|| UrlParseError("Invalid query parameter".to_string()))?;

            //watch?v=がある場合それを削除
            if query_str.starts_with("watch?v=") {
                query_str = query_str.strip_prefix("watch?v=").unwrap();
            }

            let video_id = query_str.split('?').next();
            match video_id {
                Some(video_id) => Ok(MovieUrl {
                    url_string: format!("{}watch?v={}", common_base_url, video_id),
                    video_id: video_id.to_string(),
                }),
                None => Err(UrlParseError("Invalid query parameter".to_string())),
            }
        } else {
            Err(UrlParseError(format!(
                "Invalid Url. url must have prefix: {:?}",
                MOVIE_URL_ALLOW_PREFIX
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

impl Display for MovieUrl {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.url_string)
    }
}

#[cfg(test)]
mod test {
    use super::MovieUrl;

    #[test]
    fn parse_url() {
        // 基本的なurlを与えた場合
        let movie_url =
            MovieUrl::from_url_str("https://www.youtube.com/watch?v=LjU5OOHu_As").unwrap();
        assert_eq!(
            "https://www.youtube.com/watch?v=LjU5OOHu_As",
            movie_url.url_str()
        );
        assert_eq!("LjU5OOHu_As", movie_url.video_id());

        // 基本的なurlを与えた場合2
        let movie_url = MovieUrl::from_url_str("https://youtu.be/LjU5OOHu_As").unwrap();
        assert_eq!(
            "https://www.youtube.com/watch?v=LjU5OOHu_As",
            movie_url.url_str()
        );
        assert_eq!("LjU5OOHu_As", movie_url.video_id());

        // その他のクエリパラメーターがある場合．
        let movie_url =
            MovieUrl::from_url_str("https://www.youtube.com/watch?v=LjU5OOHu_As&t=100s").unwrap();
        assert_eq!(
            "https://www.youtube.com/watch?v=LjU5OOHu_As",
            movie_url.url_str()
        );
        assert_eq!("LjU5OOHu_As", movie_url.video_id());

        let movie_url = MovieUrl::from_url_str("https://youtu.be/LjU5OOHu_As?t=100s").unwrap();
        assert_eq!(
            "https://www.youtube.com/watch?v=LjU5OOHu_As",
            movie_url.url_str()
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
            movie_url.url_str()
        );
        assert_eq!("LjU5OOHu_As", movie_url.video_id());
    }
}
