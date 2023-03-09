use domain::video::{Kirinuki, Original};

pub trait Caption {
    fn caption() -> String;
}

impl Caption for Kirinuki {
    fn caption() -> String {
        "切り抜き".to_string()
    }
}

impl Caption for Original {
    fn caption() -> String {
        "コラボ配信".to_string()
    }
}
