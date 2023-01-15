use crate::domain_error::DomainError;
use std::str::FromStr;
use time::macros::format_description;
// -------------------------------------------------------------------------------------------------
// PostalCode

/// 郵便番号
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct PostalCode(String);

impl FromStr for PostalCode {
    type Err = DomainError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // - で分ける
        let (first, second) = s.split_once('-').ok_or(DomainError::DomainParseError)?;
        if first.chars().count() == 3_usize
            && first.chars().all(|c| c.is_ascii_digit())
            && second.chars().count() == 4_usize
            && second.chars().all(|c| c.is_ascii_digit())
        {
            Ok(PostalCode(s.to_string()))
        } else {
            Err(DomainError::DomainParseError)
        }
    }
}

impl TryFrom<String> for PostalCode {
    type Error = DomainError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        value.parse()
    }
}

// -------------------------------------------------------------------------------------------------
// Date

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct Date(time::Date);

impl FromStr for Date {
    type Err = DomainError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let date = time::Date::parse(s, format_description!("[year]-[month]-[day]"))
            .map_err(|_| DomainError::DomainParseError)?;
        Ok(Date(date))
    }
}

impl TryFrom<String> for Date {
    type Error = DomainError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        value.parse()
    }
}
