use crate::commands::movie_clip_commands::{
    AllMovieClipCommand, EditMovieClipCommand, OrderByCreateDateRangeMovieClipCommand,
    OrderByLikeLimitMovieClipCommand, RemoveMovieClipCommand, SaveMovieClipCommand,
};
use crate::AppCommonError;
use domain::movie_clip::MovieClip;

use gloo_net::http::{Request, RequestMode};

const API_BASE_URL: &str = "http://127.0.0.1:8000";

#[cfg(test)]
const CORS_MODE: RequestMode = RequestMode::Cors;

#[cfg(not(test))]
const CORS_MODE: RequestMode = RequestMode::SameOrigin;

pub async fn save_movie_clip_usecase(cmd: SaveMovieClipCommand) -> Result<(), AppCommonError> {
    Request::put(&format!("{}{}", API_BASE_URL, "/movie_clip"))
        .mode(CORS_MODE)
        .json(&cmd.movie_clip)?
        .send()
        .await?;

    Ok(())
}

pub async fn edit_movie_clip_usecase(cmd: EditMovieClipCommand) -> Result<(), AppCommonError> {
    Request::patch(&format!("{}{}", API_BASE_URL, "/movie_clip"))
        .mode(CORS_MODE)
        .json(&cmd.movie_clip)?
        .send()
        .await?;

    Ok(())
}

pub async fn all_movie_clips_usecase(
    _cmd: AllMovieClipCommand,
) -> Result<Vec<MovieClip>, AppCommonError> {
    let movies = Request::get(&format!("{}{}", API_BASE_URL, "/movie_clip"))
        .mode(CORS_MODE)
        .send()
        .await?
        .json::<Vec<MovieClip>>()
        .await?;

    Ok(movies)
}

pub async fn order_movie_clips_by_like_limit_usecase(
    cmd: OrderByLikeLimitMovieClipCommand,
) -> Result<Vec<MovieClip>, AppCommonError> {
    let query_string = format!("?length={}", cmd.length);
    let ordered_movies = Request::get(&format!(
        "{}{}{}",
        API_BASE_URL, "/movie_clip/order_like", query_string
    ))
    .mode(CORS_MODE)
    .send()
    .await?
    .json::<Vec<MovieClip>>()
    .await?;

    Ok(ordered_movies)
}

pub async fn order_movie_clips_by_create_date_range_usecase(
    cmd: OrderByCreateDateRangeMovieClipCommand,
) -> Result<Vec<MovieClip>, AppCommonError> {
    let query_string = format!("?start={}&end={}", cmd.start, cmd.end);
    let ordered_movies = Request::get(&format!(
        "{}{}{}",
        API_BASE_URL, "/movie_clip/order_create_date", query_string
    ))
    .mode(CORS_MODE)
    .send()
    .await?
    .json::<Vec<MovieClip>>()
    .await?;

    Ok(ordered_movies)
}

pub async fn remove_movie_clip_usecase(cmd: RemoveMovieClipCommand) -> Result<(), AppCommonError> {
    Request::delete(&format!("{}{}{}", API_BASE_URL, "/movie_clip/", cmd.id))
        .mode(CORS_MODE)
        .send()
        .await?;

    Ok(())
}

#[cfg(test)]
pub mod test {
    use crate::commands::movie_clip_commands;
    use crate::AppCommonError;
    use domain::movie_clip::MovieClip;
    use wasm_bindgen_test::*;

    wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);
}
