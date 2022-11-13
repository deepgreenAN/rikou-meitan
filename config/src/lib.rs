pub struct Config {
    pub url_string_lim: usize,
    pub episode_string_lim: usize,
    pub url_allow_pattern: [&'static str; 2],
}

impl Config {
    const fn default() -> Self {
        Self {
            url_string_lim: 100,
            episode_string_lim: 300,
            url_allow_pattern: ["https://www.youtube.com/", "https://youtu.be/"],
        }
    }
}

pub const CONFIG: Config = Config::default();
