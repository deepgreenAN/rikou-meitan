use config::CONFIG;

pub enum Platform {
    Youtube, // いまのところyoutubeのみ
}

pub struct Url {
    row_url: String,
    video_id: String,
    platform: Platform,
}
