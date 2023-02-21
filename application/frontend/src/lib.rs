pub mod commands;
mod error;
pub mod usecases;

pub use error::AppFrontError;

use config::CONFIG;
use gloo_net::http::RequestMode;

pub const API_BASE_URL: &str = if cfg!(test) || cfg!(feature = "test_api") {
    CONFIG.test_api_domain
} else {
    CONFIG.api_domain
};

pub const CORS_MODE: RequestMode = if cfg!(test) || cfg!(feature = "test_api") {
    RequestMode::Cors
} else {
    RequestMode::SameOrigin
};
