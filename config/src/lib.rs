pub struct Config {
    pub url_string_lim: usize,              // 文字数
    pub episode_string_lim: usize,          // 文字数
    pub video_clip_title_string_lim: usize, // 文字数
}

impl Config {
    const fn default() -> Self {
        Self {
            url_string_lim: 100,
            episode_string_lim: 300,
            video_clip_title_string_lim: 100,
        }
    }
}

pub const CONFIG: Config = Config::default();
