use anyhow::Context;
use std::str::FromStr;

use crate::DomainError::{self, DomainLogicError, DomainParseError};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Year(u32);

impl Year {
    fn new(year: u32) -> Self {
        Self(year)
    }
}

impl From<u32> for Year {
    fn from(value: u32) -> Self {
        Year::new(value)
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Month(u32);

impl Month {
    fn new(month: u32) -> Result<Self, DomainError> {
        if (1..=12).contains(&month) {
            Ok(Self(month))
        } else {
            Err(DomainLogicError("Month must be in [1, 12]".to_string()))
        }
    }
}

impl TryFrom<u32> for Month {
    type Error = DomainError;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Month::new(value)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Day(u32);

impl Day {
    fn new(day: u32) -> Result<Self, DomainError> {
        if (1..=31).contains(&day) {
            Ok(Self(day))
        } else {
            Err(DomainLogicError("Day must be in [1, 31]".to_string()))
        }
    }
}

impl TryFrom<u32> for Day {
    type Error = DomainError;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Day::new(value)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Date {
    year: Year,
    month: Month,
    day: Day,
}

impl Date {
    pub fn new(year: u32, month: u32, day: u32) -> Result<Self, DomainError> {
        Ok(Self {
            year: Year::new(year),
            month: Month::new(month)?,
            day: Day::new(day)?,
        })
    }
    pub fn year(&self) -> &Year {
        &self.year
    }
    pub fn month(&self) -> &Month {
        &self.month
    }
    pub fn day(&self) -> &Day {
        &self.day
    }
}

impl TryFrom<(u32, u32, u32)> for Date {
    type Error = DomainError;
    fn try_from(value: (u32, u32, u32)) -> Result<Self, Self::Error> {
        Self::new(value.0, value.1, value.2)
    }
}

impl FromStr for Date {
    type Err = DomainError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 10 {
            return Err(DomainParseError("invalid date string".to_string()));
        }
        let str_list: Vec<&str> = s.split('-').collect();
        if str_list.len() != 3 {
            return Err(DomainParseError("invalid date string".to_string()));
        }
        Self::new(
            str_list[0].parse().context("Int Parse Error")?,
            str_list[1].parse().context("Int Parse Error")?,
            str_list[2].parse().context("Int Parse Error")?,
        )
    }
}

impl TryFrom<String> for Date {
    type Error = DomainError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        value.parse()
    }
}

#[cfg(test)]
mod test {
    use super::Date;
    use assert_matches::assert_matches;

    #[test]
    fn test_constructor() {
        let date_ok = Date::new(2022, 12, 1);
        assert_matches!(date_ok, Ok(_));
    }
}
