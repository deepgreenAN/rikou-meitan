#![cfg(feature = "server")]

use axum::{
    extract::{Path, Json, Query},
    extract::rejection::{PathRejection, JsonRejection, QueryRejection},
    routing::{delete, get, put},
    Router,
};
use tower_http::cors::{Any, CorsLayer};



mod movie_clip_handlers {
    async fn save_movie_clip()
}

#[tokio::main]
async fn main() {


}
