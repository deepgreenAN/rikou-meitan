use crate::{DomainError, GenericParseError};
use serde::{Deserialize, Serialize};
use std::{fmt::Display, marker::PhantomData, str::FromStr};
use uuid::Uuid;

/// ジェネリックなUUID
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(try_from = "String", into = "String")]
pub struct Id<T: Clone>(Uuid, PhantomData<T>);

impl<T: Clone> Id<T> {
    pub fn generate() -> Id<T> {
        Id(Uuid::new_v4(), PhantomData)
    }
    pub fn from_uuid(id: Uuid) -> Id<T> {
        id.into()
    }
    pub fn to_uuid(&self) -> Uuid {
        self.0
    }
}

impl<T: Clone> From<Uuid> for Id<T> {
    fn from(id: Uuid) -> Id<T> {
        Id(id, PhantomData)
    }
}

impl<T: Clone> FromStr for Id<T> {
    type Err = DomainError;
    fn from_str(s: &str) -> Result<Id<T>, Self::Err> {
        let id = Uuid::try_parse(s).map_err(Into::<GenericParseError>::into)?;
        Ok(Id(id, PhantomData))
    }
}

impl<T: Clone> TryFrom<String> for Id<T> {
    type Error = DomainError;
    fn try_from(value: String) -> Result<Id<T>, Self::Error> {
        value.parse()
    }
}

impl<T: Clone> Display for Id<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0.hyphenated())
    }
}

impl<T: Clone> From<Id<T>> for String {
    fn from(id: Id<T>) -> Self {
        id.to_string()
    }
}

#[cfg(test)]
mod test {
    use super::Id;
    use pretty_assertions::assert_eq;
    use uuid::Uuid;

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct FakeIdType;

    pub type FakeId = Id<FakeIdType>;

    #[test]
    fn id_eq() {
        let id_1: FakeId = Id::generate();
        let id_2 = FakeId::from_uuid(id_1.to_uuid());
        assert_eq!(id_1, id_2);
    }

    #[test]
    fn id_neq() {
        let id_1 = FakeId::generate();
        let id_2 = FakeId::generate();
        assert_ne!(id_1, id_2);
    }

    #[test]
    fn serialize_and_deserialize() {
        let id: FakeId = "67e55044-10b1-426f-9247-bb680e5fe0c8".parse().unwrap();
        let json_str = serde_json::to_string(&id).unwrap();

        assert_eq!(
            r#""67e55044-10b1-426f-9247-bb680e5fe0c8""#.to_string(),
            json_str
        );

        let json_str = r#""936da01f-9abd-4d9d-80c7-02af85c822a8""#.to_string();
        let id = serde_json::from_str::<FakeId>(&json_str).unwrap();

        assert_eq!(
            id,
            FakeId::from_uuid(Uuid::try_parse("936da01f-9abd-4d9d-80c7-02af85c822a8").unwrap())
        );
    }
}
