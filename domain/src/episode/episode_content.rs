use crate::DomainError::{self, DomainParseError};
use config::CONFIG;
use std::{fmt::Display, str::FromStr};

#[derive(Debug, Clone)]
pub struct EpisodeContent {
    content: String,
}

impl FromStr for EpisodeContent {
    type Err = DomainError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // 日本語を含めるためバイト長に変換
        if s.len() > CONFIG.episode_string_lim * 3 {
            return Err(DomainParseError(format!(
                "Episode content must be less than {}",
                CONFIG.episode_string_lim * 3
            )));
        }
        Ok(EpisodeContent {
            content: s.to_string(),
        })
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
        write!(f, "{}", self.content)
    }
}

#[cfg(test)]
mod test {
    use super::EpisodeContent;
    use crate::DomainError;
    use assert_matches::assert_matches;

    #[test]
    fn parse_episode_content() {
        let episode_str = "Some Episode".to_string();
        let episode_res_ok: Result<EpisodeContent, DomainError> = episode_str.try_into();
        assert_eq!(
            "Some Episode".to_string(),
            episode_res_ok.unwrap().to_string()
        );

        let episode_err_str = "
        ああああああああああああああああああああああああああああああああああああああああああああああああああああああああああああああああああああああああああああああああああああああああああああああああああああああああああああああああああああああああああああああああああああああああああああああああああああああああああああああああああああああああああああああああああああああああああああああああああああああああああああああああああああああああああああああああああああああああああああああああああああああああああああああああああああああああああああああああ
        ああああああああああああああああああああああああああああああああああああああああああああああああああああああああああああああああああああああああああああああああああああああああああああああああああああああああああああああああああああああああああああああああああああああああああああああああああああああああああああああああああああああああああああああああああああああああああああああああああああああああああああああああああああああああああああああああああああああああああああああああああああああああああああああああああああああああああああああああああああああああああああああああああああああああああああああああああああああああああああああああああああああああああああああああああああああああああああああああああああああああああああああああああああああああああああああああああああああああああああああああああああああああ
        "
        .to_string();
        let episode_res_err: Result<EpisodeContent, DomainError> = episode_err_str.try_into();
        assert_matches!(episode_res_err, Err(DomainError::DomainParseError(_)));
    }
}
