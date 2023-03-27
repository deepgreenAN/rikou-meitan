#[cfg(not(feature = "fake"))]
pub use self::product::*;

#[cfg(feature = "fake")]
pub use self::fake::*;

#[cfg(not(feature = "fake"))]
mod product {
    use crate::commands::movie_clip_commands;
    use crate::{
        utils::{deserialize_response, deserialize_response_null},
        AppFrontError,
    };
    use crate::{API_BASE_URL, CORS_MODE};
    use common::QueryInfoRef;
    use domain::movie_clip::MovieClip;

    use gloo_net::http::Request;
    use std::borrow::Cow::Borrowed;

    pub async fn save_movie_clip<'a>(
        cmd: movie_clip_commands::SaveMovieClipCommand<'_>,
    ) -> Result<(), AppFrontError> {
        let response = Request::put(&format!("{}{}", API_BASE_URL, "/movie_clip"))
            .mode(CORS_MODE)
            .json(&cmd.movie_clip)?
            .send()
            .await?;

        deserialize_response_null(response).await
    }

    pub async fn edit_movie_clip<'a>(
        cmd: movie_clip_commands::EditMovieClipCommand<'_>,
    ) -> Result<(), AppFrontError> {
        let response = Request::patch(&format!("{}{}", API_BASE_URL, "/movie_clip"))
            .mode(CORS_MODE)
            .json(&cmd.movie_clip)?
            .send()
            .await?;

        deserialize_response_null(response).await
    }

    pub async fn increment_like(
        cmd: movie_clip_commands::IncrementLikeMovieClipCommand,
    ) -> Result<(), AppFrontError> {
        let response = Request::patch(&format!(
            "{}{}{}",
            API_BASE_URL, "/movie_clip/increment_like/", cmd.id
        ))
        .mode(CORS_MODE)
        .send()
        .await?;

        deserialize_response_null(response).await
    }

    pub async fn all_movie_clips(
        _cmd: movie_clip_commands::AllMovieClipsCommand,
    ) -> Result<Vec<MovieClip>, AppFrontError> {
        let response = Request::get(&format!("{}{}", API_BASE_URL, "/movie_clip"))
            .mode(CORS_MODE)
            .send()
            .await?;

        deserialize_response(response).await
    }

    pub async fn order_by_like_movie_clips(
        cmd: movie_clip_commands::OrderByLikeMovieClipsCommand,
    ) -> Result<Vec<MovieClip>, AppFrontError> {
        let query_string = format!("?sort_type=like&length={}", cmd.length);
        let response = Request::get(&format!(
            "{}{}{}",
            API_BASE_URL, "/movie_clip/query", query_string
        ))
        .mode(CORS_MODE)
        .send()
        .await?;

        deserialize_response(response).await
    }

    pub async fn order_by_like_later_movie_clips<'a>(
        cmd: movie_clip_commands::OrderByLikeLaterMovieClipsCommand<'_>,
    ) -> Result<Vec<MovieClip>, AppFrontError> {
        let query_string = format!("?sort_type=like&length={}", cmd.length);
        let query_info = QueryInfoRef::builder()
            .reference(Borrowed(cmd.reference))
            .build();

        let response = Request::post(&format!(
            "{}{}{}",
            API_BASE_URL, "/movie_clip/query", query_string
        ))
        .mode(CORS_MODE)
        .json(&query_info)?
        .send()
        .await?;

        deserialize_response(response).await
    }

    pub async fn order_by_create_date_range_movie_clips(
        cmd: movie_clip_commands::OrderByCreateDateRangeMovieClipsCommand,
    ) -> Result<Vec<MovieClip>, AppFrontError> {
        let query_string = format!("?sort_type=create_date&start={}&end={}", cmd.start, cmd.end);
        let response = Request::get(&format!(
            "{}{}{}",
            API_BASE_URL, "/movie_clip/query", query_string
        ))
        .mode(CORS_MODE)
        .send()
        .await?;

        deserialize_response(response).await
    }

    pub async fn order_by_create_date_movie_clips(
        cmd: movie_clip_commands::OrderByCreateDateMovieClipsCommand,
    ) -> Result<Vec<MovieClip>, AppFrontError> {
        let query_string = format!("?sort_type=create_date&length={}", cmd.length);
        let response = Request::get(&format!(
            "{}{}{}",
            API_BASE_URL, "/movie_clip/query", query_string
        ))
        .mode(CORS_MODE)
        .send()
        .await?;

        deserialize_response(response).await
    }

    pub async fn order_by_create_date_later_movie_clips<'a>(
        cmd: movie_clip_commands::OrderByCreateDateLaterMovieClipsCommand<'_>,
    ) -> Result<Vec<MovieClip>, AppFrontError> {
        let query_string = format!("?sort_type=create_date&length={}", cmd.length);
        let query_info = QueryInfoRef::builder()
            .reference(Borrowed(cmd.reference))
            .build();

        let response = Request::post(&format!(
            "{}{}{}",
            API_BASE_URL, "/movie_clip/query", query_string
        ))
        .mode(CORS_MODE)
        .json(&query_info)?
        .send()
        .await?;

        deserialize_response(response).await
    }

    pub async fn remove_movie_clip(
        cmd: movie_clip_commands::RemoveMovieClipCommand,
    ) -> Result<(), AppFrontError> {
        let response = Request::delete(&format!("{}{}{}", API_BASE_URL, "/movie_clip/", cmd.id))
            .mode(CORS_MODE)
            .send()
            .await?;

        deserialize_response_null(response).await
    }
}

#[cfg(feature = "fake")]
mod fake {
    use crate::commands::movie_clip_commands;
    use crate::AppFrontError;
    use domain::movie_clip::MovieClip;

    use fake::{Fake, Faker};

    pub async fn save_movie_clip<'a>(
        _cmd: movie_clip_commands::SaveMovieClipCommand<'_>,
    ) -> Result<(), AppFrontError> {
        Ok(())
    }

    pub async fn edit_movie_clip<'a>(
        _cmd: movie_clip_commands::EditMovieClipCommand<'_>,
    ) -> Result<(), AppFrontError> {
        Ok(())
    }

    pub async fn increment_like(
        _cmd: movie_clip_commands::IncrementLikeMovieClipCommand,
    ) -> Result<(), AppFrontError> {
        Ok(())
    }

    pub async fn all_movie_clips(
        _cmd: movie_clip_commands::AllMovieClipsCommand,
    ) -> Result<Vec<MovieClip>, AppFrontError> {
        Ok((0..100)
            .map(|_| Faker.fake::<MovieClip>())
            .collect::<Vec<_>>())
    }

    pub async fn order_by_like_movie_clips(
        cmd: movie_clip_commands::OrderByLikeMovieClipsCommand,
    ) -> Result<Vec<MovieClip>, AppFrontError> {
        Ok((0..cmd.length)
            .map(|_| Faker.fake::<MovieClip>())
            .collect::<Vec<_>>())
    }

    pub async fn order_by_like_later_movie_clips<'a>(
        cmd: movie_clip_commands::OrderByLikeLaterMovieClipsCommand<'_>,
    ) -> Result<Vec<MovieClip>, AppFrontError> {
        Ok((0..cmd.length)
            .map(|_| Faker.fake::<MovieClip>())
            .collect::<Vec<_>>())
    }

    pub async fn order_by_create_date_range_movie_clips(
        _cmd: movie_clip_commands::OrderByCreateDateRangeMovieClipsCommand,
    ) -> Result<Vec<MovieClip>, AppFrontError> {
        Ok((0..50)
            .map(|_| Faker.fake::<MovieClip>())
            .collect::<Vec<_>>())
    }

    pub async fn order_by_create_date_movie_clips(
        cmd: movie_clip_commands::OrderByCreateDateMovieClipsCommand,
    ) -> Result<Vec<MovieClip>, AppFrontError> {
        Ok((0..cmd.length)
            .map(|_| Faker.fake::<MovieClip>())
            .collect::<Vec<_>>())
    }

    pub async fn order_by_create_date_later_movie_clips<'a>(
        cmd: movie_clip_commands::OrderByCreateDateLaterMovieClipsCommand<'_>,
    ) -> Result<Vec<MovieClip>, AppFrontError> {
        Ok((0..cmd.length)
            .map(|_| Faker.fake::<MovieClip>())
            .collect::<Vec<_>>())
    }

    pub async fn remove_movie_clip(
        _cmd: movie_clip_commands::RemoveMovieClipCommand,
    ) -> Result<(), AppFrontError> {
        Ok(())
    }
}

#[cfg(all(test, not(feature = "integration_test")))]
pub mod test {
    use crate::commands::movie_clip_commands;
    use crate::AppFrontError;
    use domain::movie_clip::{MovieClip, MovieClipId};
    use domain::Date;
    use fake::{Fake, Faker};
    use wasm_bindgen_test::*;

    wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen_test]
    async fn test_save_movie_clip() {
        let movie_clip = Faker.fake::<MovieClip>();

        let cmd = movie_clip_commands::SaveMovieClipCommand::new(&movie_clip);
        let _res: Result<(), AppFrontError> = super::save_movie_clip(cmd).await;
    }

    #[wasm_bindgen_test]
    async fn test_edit_movie_clip() {
        let movie_clip = Faker.fake::<MovieClip>();

        let cmd = movie_clip_commands::EditMovieClipCommand::new(&movie_clip);
        let _res: Result<(), AppFrontError> = super::edit_movie_clip(cmd).await;
    }

    #[wasm_bindgen_test]
    async fn test_increment_like_movie_clip() {
        let movie_clip = Faker.fake::<MovieClip>();

        let cmd = movie_clip_commands::IncrementLikeMovieClipCommand::new(movie_clip.id());
        let _res: Result<(), AppFrontError> = super::increment_like(cmd).await;
    }

    #[wasm_bindgen_test]
    async fn test_all_movie_clips() {
        let cmd = movie_clip_commands::AllMovieClipsCommand;
        let _res: Result<Vec<MovieClip>, AppFrontError> = super::all_movie_clips(cmd).await;
    }

    #[wasm_bindgen_test]
    async fn test_order_by_like_movie_clips() {
        let length = 0;
        let cmd = movie_clip_commands::OrderByLikeMovieClipsCommand::new(length);
        let _res_vec: Result<Vec<MovieClip>, AppFrontError> =
            super::order_by_like_movie_clips(cmd).await;
    }

    #[wasm_bindgen_test]
    async fn test_order_by_like_later_movie_clips() {
        let length = 0;
        let reference = Faker.fake::<MovieClip>();
        let cmd = movie_clip_commands::OrderByLikeLaterMovieClipsCommand::new(&reference, length);
        let _res_vec: Result<Vec<MovieClip>, AppFrontError> =
            super::order_by_like_later_movie_clips(cmd).await;
    }

    #[wasm_bindgen_test]
    async fn test_order_by_create_date_range_movie_clips() {
        let start = Faker.fake::<Date>();
        let end = Faker.fake::<Date>();

        let cmd = movie_clip_commands::OrderByCreateDateRangeMovieClipsCommand::new(start, end);
        let _res_vec: Result<Vec<MovieClip>, AppFrontError> =
            super::order_by_create_date_range_movie_clips(cmd).await;
    }

    #[wasm_bindgen_test]
    async fn test_order_by_create_date_movie_clips() {
        let length = 0;
        let cmd = movie_clip_commands::OrderByCreateDateMovieClipsCommand::new(length);
        let _res_vec: Result<Vec<MovieClip>, AppFrontError> =
            super::order_by_create_date_movie_clips(cmd).await;
    }

    #[wasm_bindgen_test]
    async fn test_order_by_create_date_later_movie_clips() {
        let length = 0;
        let reference = Faker.fake::<MovieClip>();
        let cmd =
            movie_clip_commands::OrderByCreateDateLaterMovieClipsCommand::new(&reference, length);
        let _res_vec: Result<Vec<MovieClip>, AppFrontError> =
            super::order_by_create_date_later_movie_clips(cmd).await;
    }

    #[wasm_bindgen_test]
    async fn test_remove_by_id_movie_clip() {
        let id = MovieClipId::generate();
        let cmd = movie_clip_commands::RemoveMovieClipCommand::new(id);
        let _res: Result<(), AppFrontError> = super::remove_movie_clip(cmd).await;
    }
}
