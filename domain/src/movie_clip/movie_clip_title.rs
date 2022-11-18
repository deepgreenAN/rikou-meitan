use std::{fmt::Display, str::FromStr};

use crate::DomainError::{self, DomainParseError};
use config::CONFIG;

#[derive(Debug, Clone)]
pub struct MovieClipTitle(String);

impl FromStr for MovieClipTitle {
    type Err = DomainError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.chars().count() > CONFIG.video_clip_title_string_lim {
            return Err(DomainParseError(format!(
                "Video clip title must be less than {}",
                CONFIG.video_clip_title_string_lim
            )));
        }

        Ok(MovieClipTitle(s.to_string()))
    }
}

impl TryFrom<String> for MovieClipTitle {
    type Error = DomainError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        value.parse()
    }
}

impl Display for MovieClipTitle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(test)]
mod test {
    use super::MovieClipTitle;
    use crate::DomainError::DomainParseError;
    use assert_matches::assert_matches;

    #[test]
    fn parse_video_clip_title() {
        let video_clip_title = "some_video_clip_title".parse::<MovieClipTitle>().unwrap();
        assert_eq!(
            "some_video_clip_title".to_string(),
            video_clip_title.to_string()
        );

        let too_big_title = "
        aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa
        aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa
        aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa
        aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa
        ".to_string();
        let video_clip_res = too_big_title.parse::<MovieClipTitle>();

        assert_matches!(video_clip_res, Err(DomainParseError(_)));
    }
}
