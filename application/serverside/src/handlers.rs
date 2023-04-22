pub mod episode_handlers;
pub mod movie_clip_handlers;
pub mod video_handlers;

#[cfg(test)]
pub mod global {
    use once_cell::sync::Lazy;
    use std::sync::Mutex;
    /// 同期テスト用のミューテックス
    pub static MTX: Lazy<Mutex<()>> = Lazy::new(|| Mutex::new(()));
}
