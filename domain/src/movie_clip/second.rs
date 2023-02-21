use crate::DomainError;
use serde::{Deserialize, Serialize};
use std::ops::Range;

#[cfg(any(test, feature = "fake"))]
use fake::{Dummy, Fake, Faker};

/// MovieClipで用いる秒指定のための秒型
#[cfg_attr(any(test, feature = "fake"), derive(Dummy))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Second(u32);

impl Second {
    pub fn from_u32(second: u32) -> Self {
        Self(second)
    }
    pub fn from_hms(hours: u32, minutes: u32, seconds: u32) -> Self {
        Self(hours * 60 * 60 + minutes * 60 + seconds)
    }
    pub fn to_u32(self) -> u32 {
        self.0
    }
    pub fn to_hms(self) -> (u32, u32, u32) {
        let mut all_seconds = self.0;
        let hours = all_seconds / (60 * 60);
        all_seconds -= hours * (60 * 60);
        let minutes = all_seconds / 60;
        all_seconds -= minutes * 60;
        (hours, minutes, all_seconds)
    }
}

impl From<u32> for Second {
    fn from(second: u32) -> Self {
        Second::from_u32(second)
    }
}

impl From<i32> for Second {
    fn from(second: i32) -> Self {
        (second as u32).into()
    }
}

impl From<Second> for u32 {
    fn from(second: Second) -> Self {
        second.to_u32()
    }
}

impl From<Second> for i32 {
    fn from(second: Second) -> Self {
        second.to_u32() as i32
    }
}

// -------------------------------------------------------------------------------------------------
// SecondRange

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct SecondRange {
    start: Second,
    end: Second,
}

impl SecondRange {
    pub fn start(&self) -> Second {
        self.start
    }
    pub fn end(&self) -> Second {
        self.end
    }
}

impl TryFrom<Range<Second>> for SecondRange {
    type Error = DomainError;
    fn try_from(value: Range<Second>) -> Result<Self, Self::Error> {
        (value.start <= value.end)
            .then_some(())
            .ok_or(DomainError::DomainLogicError(
                "It is needed start < end".to_string(),
            ))?;
        Ok(Self {
            start: value.start,
            end: value.end,
        })
    }
}

impl TryFrom<Range<u32>> for SecondRange {
    type Error = DomainError;
    fn try_from(value: Range<u32>) -> Result<Self, Self::Error> {
        let start: Second = value.start.into();
        let end: Second = value.end.into();

        (start..end).try_into()
    }
}

#[cfg(any(test, feature = "fake"))]
impl Dummy<Faker> for SecondRange {
    fn dummy_with_rng<R: rand::Rng + ?Sized>(_config: &Faker, rng: &mut R) -> Self {
        let start = Faker.fake_with_rng::<u32, R>(rng);
        let end = start
            .checked_add(Faker.fake_with_rng(rng))
            .unwrap_or(u32::MAX);
        (start..end)
            .try_into()
            .expect("Generate fake SecondRange Error")
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use fake::Faker;
    use pretty_assertions::assert_eq;

    #[test]
    fn second_from_to_u32() {
        assert_eq!(100_u32, Second::from_u32(100).to_u32());
    }

    #[test]
    fn second_from_to_hms() {
        assert_eq!((1, 40, 30), Second::from_hms(1, 40, 30).to_hms());
    }

    #[test]
    fn second_range_try_from() {
        let start = Faker.fake::<Second>();
        let end: Second = start.to_u32().saturating_sub(Faker.fake()).into();
        let res: Result<SecondRange, DomainError> = (start..end).try_into();
        assert!(matches!(res, Err(DomainError::DomainLogicError(_))));

        let start = Faker.fake::<u32>();
        let end = start.saturating_add(Faker.fake());
        let res: Result<SecondRange, DomainError> = (start..end).try_into();
        assert!(matches!(res, Ok(_)));
    }

    #[test]
    fn serialize_and_deserialize() {
        let second = Second::from_u32(100);
        let json_str = serde_json::to_string(&second).unwrap();
        assert_eq!(json_str, r#"100"#);

        let json_str = r#"200"#.to_string();
        let second = serde_json::from_str::<Second>(&json_str).unwrap();
        assert_eq!(second, Second::from_u32(200));
    }

    #[test]
    fn generate_fake() {
        let _ = (0..10000)
            .map(|_| Faker.fake::<Second>())
            .collect::<Vec<_>>();

        let _ = (0..10000)
            .map(|_| Faker.fake::<SecondRange>())
            .collect::<Vec<_>>();
    }
}
