use crate::commands::movie_clip_commands::{
    AllMovieClipCommand, EditMovieClipCommand, OrderByCreateDateRangeMovieClipCommand,
    OrderByLikeLimitMovieClipCommand, RemoveMovieClipCommand, SaveMovieClipCommand,
};
use crate::AppFrontError;
use crate::{API_BASE_URL, CORS_MODE};

use common::AppCommonError;
use domain::movie_clip::MovieClip;

use gloo_net::http::Request;

pub async fn save_movie_clip(cmd: SaveMovieClipCommand) -> Result<(), AppFrontError> {
    let response = Request::put(&format!("{}{}", API_BASE_URL, "/movie_clip"))
        .mode(CORS_MODE)
        .json(&cmd.movie_clip)?
        .send()
        .await?;

    if response.ok() {
        Ok(())
    } else {
        let err = response.json::<AppCommonError>().await?;
        Err(err.into())
    }
}

pub async fn edit_movie_clip(cmd: EditMovieClipCommand) -> Result<(), AppFrontError> {
    let response = Request::patch(&format!("{}{}", API_BASE_URL, "/movie_clip"))
        .mode(CORS_MODE)
        .json(&cmd.movie_clip)?
        .send()
        .await?;

    if response.ok() {
        Ok(())
    } else {
        let err = response.json::<AppCommonError>().await?;
        Err(err.into())
    }
}

pub async fn all_movie_clips(_cmd: AllMovieClipCommand) -> Result<Vec<MovieClip>, AppFrontError> {
    let response = Request::get(&format!("{}{}", API_BASE_URL, "/movie_clip"))
        .mode(CORS_MODE)
        .send()
        .await?;

    if response.ok() {
        let movies = response.json::<Vec<MovieClip>>().await?;
        Ok(movies)
    } else {
        let err = response.json::<AppCommonError>().await?;
        Err(err.into())
    }
}

pub async fn order_movie_clips_by_like_limit(
    cmd: OrderByLikeLimitMovieClipCommand,
) -> Result<Vec<MovieClip>, AppFrontError> {
    let query_string = format!("?length={}", cmd.length);
    let response = Request::get(&format!(
        "{}{}{}",
        API_BASE_URL, "/movie_clip/order_like", query_string
    ))
    .mode(CORS_MODE)
    .send()
    .await?;

    if response.ok() {
        let ordered_movies = response.json::<Vec<MovieClip>>().await?;
        Ok(ordered_movies)
    } else {
        let err = response.json::<AppCommonError>().await?;
        Err(err.into())
    }
}

pub async fn order_movie_clips_by_create_date_range(
    cmd: OrderByCreateDateRangeMovieClipCommand,
) -> Result<Vec<MovieClip>, AppFrontError> {
    let query_string = format!("?start={}&end={}", cmd.start, cmd.end);
    let response = Request::get(&format!(
        "{}{}{}",
        API_BASE_URL, "/movie_clip/order_create_date", query_string
    ))
    .mode(CORS_MODE)
    .send()
    .await?;

    if response.ok() {
        let ordered_movies = response.json::<Vec<MovieClip>>().await?;
        Ok(ordered_movies)
    } else {
        let err = response.json::<AppCommonError>().await?;
        Err(err.into())
    }
}

pub async fn remove_movie_clip(cmd: RemoveMovieClipCommand) -> Result<(), AppFrontError> {
    let response = Request::delete(&format!("{}{}{}", API_BASE_URL, "/movie_clip/", cmd.id))
        .mode(CORS_MODE)
        .send()
        .await?;
    if response.ok() {
        Ok(())
    } else {
        let err = response.json::<AppCommonError>().await?;
        Err(err.into())
    }
}

#[cfg(test)]
pub mod test {
    use crate::commands::movie_clip_commands;
    use crate::AppFrontError;
    use assert_matches::assert_matches;
    use domain::movie_clip::{MovieClip, MovieClipId};
    use domain::Date;
    use wasm_bindgen_test::*;

    wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen_test]
    async fn test_save_movie_clip() {
        let movie_clip_ok = MovieClip::new(
            "Some Title".to_string(),
            "https://www.youtube.com/watch?v=gPkvkFiG8vE".to_string(),
            300,
            400,
            (2022, 12, 6),
        )
        .unwrap();

        let cmd = movie_clip_commands::SaveMovieClipCommand::new(movie_clip_ok);
        let res_ok = super::save_movie_clip(cmd).await;
        assert_matches!(res_ok, Ok(_));

        // CommonError::ConflictError
        let movie_clip_err = MovieClip::new(
            "ConflictError".to_string(),
            "https://www.youtube.com/watch?v=gPkvkFiG8vE".to_string(),
            300,
            400,
            (2022, 12, 6),
        )
        .unwrap();

        let cmd = movie_clip_commands::SaveMovieClipCommand::new(movie_clip_err);
        let res_err = super::save_movie_clip(cmd).await;
        assert_matches!(res_err, Err(AppFrontError::CommonError(_)));
    }

    #[wasm_bindgen_test]
    async fn test_edit_movie_clip() {
        let movie_clip_ok = MovieClip::new(
            "Some Title".to_string(),
            "https://www.youtube.com/watch?v=gPkvkFiG8vE".to_string(),
            300,
            400,
            (2022, 12, 6),
        )
        .unwrap();

        let cmd = movie_clip_commands::EditMovieClipCommand::new(movie_clip_ok);
        let res_ok = super::edit_movie_clip(cmd).await;
        assert_matches!(res_ok, Ok(_));

        // CommonError::NoRecorderror
        let movie_clip_err = MovieClip::new(
            "NoRecordError".to_string(),
            "https://www.youtube.com/watch?v=gPkvkFiG8vE".to_string(),
            300,
            400,
            (2022, 12, 6),
        )
        .unwrap();

        let cmd = movie_clip_commands::EditMovieClipCommand::new(movie_clip_err);
        let res_err = super::edit_movie_clip(cmd).await;
        assert_matches!(res_err, Err(AppFrontError::CommonError(_)));
    }

    #[wasm_bindgen_test]
    async fn test_all_movie_clips() {
        let cmd = movie_clip_commands::AllMovieClipCommand;
        let res_vec = super::all_movie_clips(cmd).await.unwrap();
        assert_eq!(res_vec, Vec::new());
    }

    #[wasm_bindgen_test]
    async fn test_order_by_like_limit_movie_clips() {
        let length = 0;
        let cmd = movie_clip_commands::OrderByLikeLimitMovieClipCommand::new(length);
        let res_vec = super::order_movie_clips_by_like_limit(cmd).await.unwrap();
        assert_eq!(res_vec, Vec::new());
    }

    #[wasm_bindgen_test]
    async fn test_order_by_create_date_range_movie_clips() {
        let start = Date::from_ymd(2022, 12, 4).unwrap();
        let end = Date::from_ymd(2022, 12, 6).unwrap();

        let cmd = movie_clip_commands::OrderByCreateDateRangeMovieClipCommand::new(start, end);
        let res_vec = super::order_movie_clips_by_create_date_range(cmd)
            .await
            .unwrap();
        assert_eq!(res_vec, Vec::new());
    }

    #[wasm_bindgen_test]
    async fn test_remove_by_id_movie_clip() {
        let id_ok: MovieClipId = "67e55044-10b1-426f-9247-bb680e5fe0c8".parse().unwrap();
        let cmd = movie_clip_commands::RemoveMovieClipCommand::new(id_ok);
        let res_ok = super::remove_movie_clip(cmd).await;
        assert_matches!(res_ok, Ok(_));

        // CommonError::NoRecordError
        let id_err = MovieClipId::generate();
        let cmd = movie_clip_commands::RemoveMovieClipCommand::new(id_err);
        let res_err = super::remove_movie_clip(cmd).await;
        assert_matches!(res_err, Err(AppFrontError::CommonError(_)));
    }
}
