use std::{fmt::Display, str::FromStr};

use crate::DomainError::{self, DomainLogicError, DomainParseError};
use crate::GenericParseError;

#[cfg(feature = "server")]
use chrono::{Datelike, NaiveDate};

/// Date型のYear
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Year(u32);

impl Year {
    fn new(year: u32) -> Result<Self, DomainError> {
        if (2018..=2050).contains(&year) {
            Ok(Self(year))
        } else {
            Err(DomainLogicError("Year is too small".to_string()))
        }
    }
    fn to_u32(self) -> u32 {
        self.0
    }
}

impl TryFrom<u32> for Year {
    type Error = DomainError;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Year::new(value)
    }
}

/// Date型のMonth
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Month(u32);

impl Month {
    fn new(month: u32) -> Result<Self, DomainError> {
        if (1..=12).contains(&month) {
            Ok(Self(month))
        } else {
            Err(DomainLogicError("Month must be in [1, 12]".to_string()))
        }
    }
    fn to_u32(self) -> u32 {
        self.0
    }
}

impl TryFrom<u32> for Month {
    type Error = DomainError;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Month::new(value)
    }
}

/// Date型のDay
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Day(u32);

impl Day {
    fn new(day: u32) -> Result<Self, DomainError> {
        if (1..=31).contains(&day) {
            Ok(Self(day))
        } else {
            Err(DomainLogicError("Day must be in [1, 31]".to_string()))
        }
    }
    fn to_u32(self) -> u32 {
        self.0
    }
}

impl TryFrom<u32> for Day {
    type Error = DomainError;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Day::new(value)
    }
}

/// 軽量なDate
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Date {
    year: Year,
    month: Month,
    day: Day,
}

impl Date {
    pub fn from_ymd(year: u32, month: u32, day: u32) -> Result<Self, DomainError> {
        Ok(Self {
            year: Year::new(year)?,
            month: Month::new(month)?,
            day: Day::new(day)?,
        })
    }
    pub fn to_ymd(&self) -> (u32, u32, u32) {
        (self.year.to_u32(), self.month.to_u32(), self.day.to_u32())
    }
    #[cfg(feature = "server")]
    pub fn to_chrono(&self) -> Result<NaiveDate, DomainError> {
        NaiveDate::from_ymd_opt(
            self.year.to_u32() as i32,
            self.month.to_u32(),
            self.day.to_u32(),
        )
        .ok_or_else(|| DomainLogicError("Invalid Date".to_string()))
    }
}

impl TryFrom<(u32, u32, u32)> for Date {
    type Error = DomainError;
    fn try_from(value: (u32, u32, u32)) -> Result<Self, Self::Error> {
        Self::from_ymd(value.0, value.1, value.2)
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
        Self::from_ymd(
            str_list[0]
                .parse()
                .map_err(Into::<GenericParseError>::into)?,
            str_list[1]
                .parse()
                .map_err(Into::<GenericParseError>::into)?,
            str_list[2]
                .parse()
                .map_err(Into::<GenericParseError>::into)?,
        )
    }
}

impl TryFrom<String> for Date {
    type Error = DomainError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        value.parse()
    }
}

#[cfg(feature = "server")]
impl TryFrom<NaiveDate> for Date {
    type Error = DomainError;
    fn try_from(value: NaiveDate) -> Result<Self, Self::Error> {
        Self::from_ymd(value.year() as u32, value.month(), value.day())
    }
}

impl Display for Date {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{:>04}-{:>02}-{:>02}",
            self.year.to_u32(),
            self.month.to_u32(),
            self.day.to_u32()
        )
    }
}

#[cfg(test)]
mod test {
    use super::Date;
    use crate::DomainError;
    use assert_matches::assert_matches;

    #[test]
    fn test_constructor() {
        let date_ok = Date::from_ymd(2022, 12, 1);
        assert_matches!(date_ok, Ok(_));
        let date_err = Date::from_ymd(2020, 13, 1);
        assert_matches!(date_err, Err(DomainError::DomainLogicError(_)));
    }

    #[test]
    fn test_parse() {
        let parsed_date_ok = "2022-12-01".parse::<Date>();
        assert_eq!(
            parsed_date_ok.unwrap(),
            Date::from_ymd(2022, 12, 1).unwrap()
        );

        let parsed_date_err = "2022-12-1".parse::<Date>();
        assert_matches!(parsed_date_err, Err(DomainError::DomainParseError(_)));

        let parsed_date_err = "2022 12 1".parse::<Date>();
        assert_matches!(parsed_date_err, Err(DomainError::DomainParseError(_)));

        let parsed_date_err = "12-01-2022".parse::<Date>();
        assert_matches!(parsed_date_err, Err(DomainError::DomainLogicError(_)));

        let parsed_date_err = "2022-12-aa".parse::<Date>();
        assert_matches!(parsed_date_err, Err(DomainError::GenericParseError(_)));
    }

    #[test]
    fn to_string() {
        assert_eq!(
            "2021-12-01",
            Date::from_ymd(2021, 12, 1).unwrap().to_string()
        );
        assert_eq!(
            "2021-09-05",
            Date::from_ymd(2021, 9, 5).unwrap().to_string()
        )
    }
}
