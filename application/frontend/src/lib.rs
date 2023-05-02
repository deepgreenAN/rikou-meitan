pub mod commands;
mod error;
pub mod usecases;
pub mod utils;

pub use error::AppFrontError;

use config::CONFIG;
use once_cell::sync::OnceCell;

pub(crate) static API_BASE_URL: OnceCell<String> = OnceCell::new();

pub(crate) fn api_base_url() -> String {
    if cfg!(test) || cfg!(feature = "test_api") {
        CONFIG.test_api_domain.to_string()
    } else {
        #[cfg(target_arch = "wasm32")]
        let origin = gloo_utils::window()
            .location()
            .origin()
            .expect("Cannot get origin string.");

        #[cfg(not(target_arch = "wasm32"))]
        let origin = "".to_string(); // おそらく失敗する
        format!("{}{}", origin, CONFIG.api_domain)
    }
}
