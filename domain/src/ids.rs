use crate::{DomainError, GenericParseError};
use std::{fmt::Display, marker::PhantomData, str::FromStr};
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Id<T>(Uuid, PhantomData<T>);

#[derive(Debug, Clone, Copy)]
pub struct ClipId;

impl<T> Id<T> {
    pub fn generate<U>() -> Id<U> {
        let id = Uuid::new_v4();
        Id(id, PhantomData)
    }
}

impl<T> From<Uuid> for Id<T> {
    fn from(id: Uuid) -> Self {
        Id(id, PhantomData)
    }
}

impl<T> FromStr for Id<T> {
    type Err = DomainError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let id = Uuid::parse_str(s).map_err(Into::<GenericParseError>::into)?;
        Ok(id.into())
    }
}

impl<T> Display for Id<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0.hyphenated())
    }
}
