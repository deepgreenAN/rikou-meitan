use std::{fmt::Display, marker::PhantomData};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Id<T>([u8; 16], PhantomData<T>);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ClipIdType;

pub type ClipId = Id<ClipIdType>;

impl<T> Id<T> {
    pub fn from_bytes(id_u8: [u8; 16]) -> Id<T> {
        Id(id_u8, PhantomData)
    }
    pub fn to_bytes(&self) -> [u8; 16] {
        self.0
    }
}

impl<T> Display for Id<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let id_u8_str = self
            .0
            .iter()
            .map(|i| i.to_string())
            .collect::<Vec<_>>()
            .join("");
        write!(f, "{:?}", id_u8_str)
    }
}

#[cfg(test)]
mod test {
    use super::{ClipId, Id};

    #[test]
    fn id_eq() {
        let bytes = [
            0xa1, 0xa2, 0xa3, 0xa4, 0xb1, 0xb2, 0xc1, 0xc2, 0xd1, 0xd2, 0xd3, 0xd4, 0xd5, 0xd6,
            0xd7, 0xd8,
        ];

        let id_1: ClipId = Id::from_bytes(bytes);
        let id_2 = ClipId::from_bytes(bytes);
        assert_eq!(id_1, id_2);
    }

    #[test]
    fn id_neq() {
        let bytes_1 = [
            0xa1, 0xa2, 0xa3, 0xa4, 0xb1, 0xb2, 0xc1, 0xc2, 0xd1, 0xd2, 0xd3, 0xd4, 0xd5, 0xd6,
            0xd7, 0xd8,
        ];

        let bytes_2 = [
            0xd8, 0xd7, 0xd6, 0xd5, 0xd4, 0xd3, 0xd2, 0xd1, 0xc2, 0xc1, 0xb2, 0xb1, 0xa4, 0xa3,
            0xa2, 0xa1,
        ];

        let id_1 = ClipId::from_bytes(bytes_1);
        let id_2 = ClipId::from_bytes(bytes_2);
        assert_ne!(id_1, id_2);
    }
}
