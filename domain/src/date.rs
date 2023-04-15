use crate::DomainError::{self, DomainLogicError};
use crate::GenericParseError;

use chrono::{Datelike, NaiveDate};
use serde::{Deserialize, Serialize};
use std::{fmt::Display, str::FromStr};

#[cfg(any(test, feature = "fake"))]
use fake::{Dummy, Fake, Faker};

#[cfg(any(test, feature = "fake"))]
use rand::Rng;

/// Date
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(try_from = "String", into = "String")]
pub struct Date(NaiveDate);

impl Date {
    /// year, month, dayを指定してDateを作成．
    pub fn from_ymd(year: u32, month: u32, day: u32) -> Result<Self, DomainError> {
        let chrono_date = NaiveDate::from_ymd_opt(year as i32, month, day)
            .ok_or(DomainLogicError("Invalid Date".to_string()))?;
        Ok(Self(chrono_date))
    }
    /// year, month, dayを取得
    pub fn to_ymd(&self) -> (u32, u32, u32) {
        (self.0.year() as u32, self.0.month(), self.0.day())
    }
    /// chrono::NaiveDateへ変換．
    pub fn to_chrono(&self) -> Result<NaiveDate, DomainError> {
        Ok(self.0)
    }
    /// chrono::NaiveDateからの変換．
    pub fn from_chrono(chrono_date: NaiveDate) -> Result<Self, DomainError> {
        Ok(Self(chrono_date))
    }
}

impl TryFrom<(u32, u32, u32)> for Date {
    type Error = DomainError;
    fn try_from(value: (u32, u32, u32)) -> Result<Self, Self::Error> {
        Self::from_ymd(value.0, value.1, value.2)
    }
}

// String <-> Dae

impl FromStr for Date {
    type Err = DomainError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let chrono_date =
            NaiveDate::parse_from_str(s, "%Y-%m-%d").map_err(Into::<GenericParseError>::into)?;
        Date::from_chrono(chrono_date)
    }
}

impl TryFrom<String> for Date {
    type Error = DomainError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        value.parse()
    }
}

impl Display for Date {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0.format("%Y-%m-%d"))
    }
}

impl From<Date> for String {
    fn from(date: Date) -> Self {
        date.to_string()
    }
}

// NaiveDate <-> Date

impl TryFrom<NaiveDate> for Date {
    type Error = DomainError;
    fn try_from(value: NaiveDate) -> Result<Self, Self::Error> {
        Self::from_chrono(value)
    }
}

impl TryFrom<Date> for NaiveDate {
    type Error = DomainError;
    fn try_from(value: Date) -> Result<Self, Self::Error> {
        value.to_chrono()
    }
}

// -------------------------------------------------------------------------------------------------
// Dummy trait

#[cfg(any(feature = "fake", test))]
impl Dummy<Faker> for Date {
    fn dummy_with_rng<R: Rng + ?Sized>(_config: &Faker, rng: &mut R) -> Self {
        let start = Date::from_ymd(2018, 12, 27).unwrap();
        let end = Date::from_ymd(2023, 12, 31).unwrap();

        (start..end).fake_with_rng(rng)
    }
}

#[cfg(any(feature = "fake", test))]
impl Dummy<std::ops::Range<Date>> for Date {
    fn dummy_with_rng<R: Rng + ?Sized>(config: &std::ops::Range<Date>, rng: &mut R) -> Self {
        let chrono_start = config.start.to_chrono().expect("Generate fake Date Error");
        let chrono_end = config.end.to_chrono().expect("Generate fake Date Error");
        let days = (0..(chrono_end - chrono_start).num_days()).fake_with_rng::<i64, R>(rng);

        let chrono_date = chrono_start + chrono::Duration::days(days);
        chrono_date.try_into().expect("Generate fake Date Error")
    }
}

#[cfg(test)]
mod test {
    use super::Date;
    use crate::DomainError;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_constructor() {
        let date_ok = Date::from_ymd(2022, 12, 1);
        assert!(matches!(date_ok, Ok(_)));
        let date_err = Date::from_ymd(2020, 13, 1);
        assert!(matches!(date_err, Err(DomainError::DomainLogicError(_))));
    }

    #[test]
    fn test_parse() {
        let parsed_date_ok = "2022-12-01".parse::<Date>();
        assert_eq!(
            parsed_date_ok.unwrap(),
            Date::from_ymd(2022, 12, 1).unwrap()
        );

        let parsed_date_err: Result<Date, DomainError> = "2022-12-aa".parse::<Date>();
        assert!(matches!(parsed_date_err, Err(_)));
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

    #[test]
    fn serialize_and_deserialize() {
        let date = Date::from_ymd(2022, 11, 23).unwrap();
        let json_str = serde_json::to_string(&date).unwrap();

        assert_eq!(r#""2022-11-23""#.to_string(), json_str);

        let json_str = r#""2022-11-24""#.to_string();
        let date = serde_json::from_str::<Date>(&json_str).unwrap();

        assert_eq!(date, Date::from_ymd(2022, 11, 24).unwrap())
    }

    #[cfg(feature = "fake")]
    #[test]
    fn generate_fake() {
        use fake::{Fake, Faker};

        let _ = (0..10000).map(|_| Faker.fake::<Date>()).collect::<Vec<_>>();

        let start = Date::from_ymd(2018, 12, 27).unwrap();
        let end = Date::from_ymd(2023, 1, 3).unwrap();
        let range_dates = (0..1000)
            .map(|_| (start..end).fake::<Date>())
            .collect::<Vec<_>>();
        range_dates.into_iter().for_each(|date| {
            assert!(start <= date && date < end);
        });
    }
}
