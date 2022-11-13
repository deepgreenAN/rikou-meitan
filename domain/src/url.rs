use config::Config;

pub enum Platform {
    Youtube, // いまのところyoutubeのみ
}

pub struct Url {
    video_id: String,
    start_time: u32,
    end_time: u32,
}
