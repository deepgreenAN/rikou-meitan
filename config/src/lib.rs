pub struct Config {
    pub url_string_lim: usize,
    pub episode_string_lim: usize,
}

impl Config {
    const fn default() -> Self {
        Self {
            url_string_lim: 100,
            episode_string_lim: 300,
        }
    }
}

pub const CONFIG: Config = Config::default();
