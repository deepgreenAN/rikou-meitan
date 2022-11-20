pub struct Config {
    pub url_string_lim: usize,
}

impl Config {
    const fn default() -> Self {
        Self {
            url_string_lim: 100,
        }
    }
}

pub const CONFIG: Config = Config::default();
