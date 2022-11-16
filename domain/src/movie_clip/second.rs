#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn from_to_u32() {
        assert_eq!(100_u32, Second::from_u32(100).to_u32());
    }
    #[test]
    fn from_to_hms() {
        assert_eq!((1, 40, 30), Second::from_hms(1, 40, 30).to_hms());
    }
}
