use std::{fmt::Display, marker::PhantomData};
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Id<T>(Uuid, PhantomData<T>);

impl<T> Id<T> {
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

impl<T> From<Uuid> for Id<T> {
    fn from(uuid: Uuid) -> Id<T> {
        Id(uuid, PhantomData)
    }
}

impl<T> Display for Id<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.0)
    }
}

#[cfg(test)]
mod test {
    use super::Id;

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
}
