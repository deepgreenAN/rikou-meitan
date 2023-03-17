use crate::commands::movie_clip_commands;
use common::AppCommonError;
use domain::movie_clip::{MovieClip, MovieClipId};
use domain::Date;

/// movie_clip_usecaseのモック化
#[cfg(not(test))]
use crate::usecases::movie_clip_usecases;

#[cfg(test)]
use crate::usecases::mock_movie_clip_usecases as movie_clip_usecases;

/// AppStateのモック化
#[cfg(all(not(test), feature = "inmemory"))]
use crate::app_state::InMemoryAppState as AppState;

#[cfg(all(not(test), not(feature = "inmemory")))]
use crate::app_state::AppState;

#[cfg(test)]
use crate::app_state::MockAppState as AppState;

use axum::{
    extract::rejection::{JsonRejection, PathRejection, QueryRejection},
    extract::{Json, Path, Query, State},
};

use serde::Deserialize;

pub async fn save_movie_clip(
    State(app_state): State<AppState>,
    movie_clip_res: Result<Json<MovieClip>, JsonRejection>,
) -> Result<(), AppCommonError> {
    let movie_clip = movie_clip_res?.0;

    let cmd = movie_clip_commands::SaveMovieClipCommand::new(movie_clip);
    movie_clip_usecases::save_movie_clip(app_state.movie_clip_repo, cmd).await?;
    Ok(())
}

pub async fn edit_movie_clip(
    State(app_state): State<AppState>,
    movie_clip_res: Result<Json<MovieClip>, JsonRejection>,
) -> Result<(), AppCommonError> {
    let movie_clip = movie_clip_res?.0;

    let cmd = movie_clip_commands::EditMovieClipCommand::new(movie_clip);
    movie_clip_usecases::edit_movie_clip(app_state.movie_clip_repo, cmd).await?;

    Ok(())
}

pub async fn all_movie_clips(
    State(app_state): State<AppState>,
) -> Result<Json<Vec<MovieClip>>, AppCommonError> {
    let cmd = movie_clip_commands::AllMovieClipCommand;
    let movie_clips = movie_clip_usecases::all_movie_clips(app_state.movie_clip_repo, cmd).await?;
    Ok(Json(movie_clips))
}

#[derive(Deserialize)]
pub struct Limit {
    length: usize,
}

pub async fn order_by_like_limit_movie_clips(
    limit_res: Result<Query<Limit>, QueryRejection>,
    State(app_state): State<AppState>,
) -> Result<Json<Vec<MovieClip>>, AppCommonError> {
    let limit = limit_res.map_err(Into::<AppCommonError>::into)?.0;
    let cmd = movie_clip_commands::OrderByLikeLimitMovieClipCommand::new(limit.length);
    let movie_clips =
        movie_clip_usecases::order_by_like_movie_clips(app_state.movie_clip_repo, cmd).await?;
    Ok(Json(movie_clips))
}

#[derive(Deserialize)]
pub struct DateRange {
    start: Date,
    end: Date,
}

pub async fn order_by_create_date_range_movie_clips(
    date_range_res: Result<Query<DateRange>, QueryRejection>,
    State(app_state): State<AppState>,
) -> Result<Json<Vec<MovieClip>>, AppCommonError> {
    let date_range = date_range_res?.0;
    let cmd = movie_clip_commands::OrderByCreateDateRangeMovieClipCommand::new(
        date_range.start,
        date_range.end,
    );
    let movie_clips =
        movie_clip_usecases::order_by_create_date_range_movie_clips(app_state.movie_clip_repo, cmd)
            .await?;
    Ok(Json(movie_clips))
}

pub async fn remove_by_id_movie_clip(
    id: Result<Path<MovieClipId>, PathRejection>,
    State(app_state): State<AppState>,
) -> Result<(), AppCommonError> {
    let id = id?.0;
    let cmd = movie_clip_commands::RemoveByIdMovieClipCommand::new(id);
    movie_clip_usecases::remove_movie_clip(app_state.movie_clip_repo, cmd).await?;
    Ok(())
}

#[cfg(test)]
mod test {
    use super::AppState;
    use crate::usecases::mock_movie_clip_usecases;
    use common::AppCommonError;
    use domain::movie_clip::{MovieClip, MovieClipId};
    use domain::Date;
    use infrastructure::movie_clip_repository_impl::MockMovieClipRepositoryImpl;

    use assert_matches::assert_matches;
    use axum::{
        body::Body,
        http::{self, Request, StatusCode},
        routing::{delete, get, put},
        Router,
    };
    use once_cell::sync::Lazy;
    use pretty_assertions::{assert_eq, assert_ne};
    use rstest::{fixture, rstest};
    use std::sync::Mutex;
    use tower::{Service, ServiceExt};

    static MTX: Lazy<Mutex<()>> = Lazy::new(|| Mutex::new(()));

    #[fixture]
    fn router() -> Router {
        let app_state = AppState::default();

        Router::new()
            .route(
                "/movie_clip",
                put(super::save_movie_clip)
                    .patch(super::edit_movie_clip)
                    .get(super::all_movie_clips),
            )
            .route(
                "/movie_clip/order_like",
                get(super::order_by_like_limit_movie_clips),
            )
            .route(
                "/movie_clip/order_create_date",
                get(super::order_by_create_date_range_movie_clips),
            )
            .route("/movie_clip/:id", delete(super::remove_by_id_movie_clip))
            .with_state(app_state)
    }
    #[allow(clippy::await_holding_lock)]
    #[rstest]
    #[tokio::test]
    async fn test_save_movie_clip(mut router: Router) {
        let _m = MTX.lock().unwrap_or_else(|p_err| p_err.into_inner());

        let movie_clip = MovieClip::new(
            "MovieClip 1".to_string(),
            "https://www.youtube.com/watch?v=B7OPlsdBuVc".to_string(),
            100,
            200,
            (2022, 11, 21),
        )
        .unwrap();

        {
            let cloned_movie_clip = movie_clip.clone();
            let mock_ctx_ok = mock_movie_clip_usecases::save_movie_clip_context();
            mock_ctx_ok
                .expect::<MockMovieClipRepositoryImpl>()
                .withf(move |_, cmd| cmd.movie_clip == cloned_movie_clip)
                .times(1)
                .return_const(Ok(()));

            let request = Request::builder()
                .method(http::Method::PUT)
                .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                .uri("/movie_clip")
                .body(Body::from(serde_json::to_vec(&movie_clip).unwrap()))
                .unwrap();

            let response = router.ready().await.unwrap().call(request).await.unwrap();
            assert_eq!(response.status(), StatusCode::OK);
            let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
            assert!(body.is_empty());
        }
        {
            let cloned_movie_clip = movie_clip.clone();
            let mock_ctx_err = mock_movie_clip_usecases::save_movie_clip_context();
            mock_ctx_err
                .expect::<MockMovieClipRepositoryImpl>()
                .withf(move |_, cmd| cmd.movie_clip == cloned_movie_clip)
                .times(1)
                .return_const(Err(AppCommonError::ConflictError));

            let request = Request::builder()
                .method(http::Method::PUT)
                .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                .uri("/movie_clip")
                .body(Body::from(serde_json::to_vec(&movie_clip).unwrap()))
                .unwrap();

            let response = router.ready().await.unwrap().call(request).await.unwrap();
            assert_ne!(response.status(), StatusCode::OK);

            let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
            let err: AppCommonError = serde_json::from_slice(&body).unwrap();

            assert_matches!(err, AppCommonError::ConflictError);
        }
    }
    #[allow(clippy::await_holding_lock)]
    #[rstest]
    #[tokio::test]
    async fn test_edit_movie_clip(mut router: Router) {
        let _m = MTX.lock().unwrap_or_else(|p_err| p_err.into_inner());

        let movie_clip = MovieClip::new(
            "MovieClip 1".to_string(),
            "https://www.youtube.com/watch?v=B7OPlsdBuVc".to_string(),
            100,
            200,
            (2022, 11, 21),
        )
        .unwrap();

        {
            let cloned_movie_clip = movie_clip.clone();
            let mock_ctx_ok = mock_movie_clip_usecases::edit_movie_clip_context();
            mock_ctx_ok
                .expect::<MockMovieClipRepositoryImpl>()
                .withf(move |_, cmd| cmd.movie_clip == cloned_movie_clip)
                .times(1)
                .return_const(Ok(()));

            let request = Request::builder()
                .method(http::Method::PATCH)
                .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                .uri("/movie_clip")
                .body(Body::from(serde_json::to_vec(&movie_clip).unwrap()))
                .unwrap();

            let response = router.ready().await.unwrap().call(request).await.unwrap();
            assert_eq!(response.status(), StatusCode::OK);
            let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
            assert!(body.is_empty());
        }
        {
            let cloned_movie_clip = movie_clip.clone();
            let mock_ctx_err = mock_movie_clip_usecases::edit_movie_clip_context();
            mock_ctx_err
                .expect::<MockMovieClipRepositoryImpl>()
                .withf(move |_, cmd| cmd.movie_clip == cloned_movie_clip)
                .times(1)
                .return_const(Err(AppCommonError::NoRecordError));

            let request = Request::builder()
                .method(http::Method::PATCH)
                .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                .uri("/movie_clip")
                .body(Body::from(serde_json::to_vec(&movie_clip).unwrap()))
                .unwrap();

            let response = router.ready().await.unwrap().call(request).await.unwrap();
            assert_ne!(response.status(), StatusCode::OK);

            let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
            let err: AppCommonError = serde_json::from_slice(&body).unwrap();

            assert_matches!(err, AppCommonError::NoRecordError);
        }
    }
    #[allow(clippy::await_holding_lock)]
    #[rstest]
    #[tokio::test]
    async fn test_all_movie_clips(mut router: Router) {
        let _m = MTX.lock().unwrap_or_else(|p_err| p_err.into_inner());
        let movie_clips = vec![MovieClip::new(
            "MovieClip 1".to_string(),
            "https://www.youtube.com/watch?v=B7OPlsdBuVc".to_string(),
            100,
            200,
            (2022, 11, 21),
        )
        .unwrap()];

        let mock_ctx = mock_movie_clip_usecases::all_movie_clips_context();
        mock_ctx
            .expect::<MockMovieClipRepositoryImpl>()
            .times(1)
            .return_const(Ok(movie_clips.clone()));

        let request = Request::builder()
            .method(http::Method::GET)
            .uri("/movie_clip")
            .body(Body::empty())
            .unwrap();

        let response = router.ready().await.unwrap().call(request).await.unwrap();
        assert_eq!(response.status(), StatusCode::OK);

        let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
        let res_vec: Vec<MovieClip> = serde_json::from_slice(&body).unwrap();

        assert_eq!(res_vec, movie_clips);
    }
    #[allow(clippy::await_holding_lock)]
    #[rstest]
    #[tokio::test]
    async fn test_order_by_like_limit_movie_clips(mut router: Router) {
        let _m = MTX.lock().unwrap_or_else(|p_err| p_err.into_inner());
        let movie_clips = vec![MovieClip::new(
            "MovieClip 1".to_string(),
            "https://www.youtube.com/watch?v=B7OPlsdBuVc".to_string(),
            100,
            200,
            (2022, 11, 21),
        )
        .unwrap()];

        let length = 1_usize;

        let mock_ctx = mock_movie_clip_usecases::order_by_like_movie_clips_context();
        mock_ctx
            .expect::<MockMovieClipRepositoryImpl>()
            .withf(move |_, cmd| cmd.length == length)
            .times(1)
            .return_const(Ok(movie_clips.clone()));

        let request = Request::builder()
            .method(http::Method::GET)
            .uri(&format!("/movie_clip/order_like?length={length}"))
            .body(Body::empty())
            .unwrap();

        let response = router.ready().await.unwrap().call(request).await.unwrap();
        assert_eq!(response.status(), StatusCode::OK);

        let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
        let res_vec: Vec<MovieClip> = serde_json::from_slice(&body).unwrap();

        assert_eq!(res_vec, movie_clips);
    }
    #[allow(clippy::await_holding_lock)]
    #[rstest]
    #[tokio::test]
    async fn test_order_by_crate_date_range_movie_clips(mut router: Router) {
        let _m = MTX.lock().unwrap_or_else(|p_err| p_err.into_inner());
        let movie_clips = vec![MovieClip::new(
            "MovieClip 1".to_string(),
            "https://www.youtube.com/watch?v=B7OPlsdBuVc".to_string(),
            100,
            200,
            (2022, 11, 21),
        )
        .unwrap()];

        let start = Date::from_ymd(2022, 11, 21).unwrap();
        let end = Date::from_ymd(2022, 11, 23).unwrap();

        let mock_ctx = mock_movie_clip_usecases::order_by_create_date_range_movie_clips_context();
        mock_ctx
            .expect::<MockMovieClipRepositoryImpl>()
            .withf(move |_, cmd| cmd.start == start && cmd.end == end)
            .times(1)
            .return_const(Ok(movie_clips.clone()));

        let request = Request::builder()
            .method(http::Method::GET)
            .uri(&format!(
                "/movie_clip/order_create_date?start={start}&end={end}"
            ))
            .body(Body::empty())
            .unwrap();

        let response = router.ready().await.unwrap().call(request).await.unwrap();
        assert_eq!(response.status(), StatusCode::OK);

        let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
        let res_vec: Vec<MovieClip> = serde_json::from_slice(&body).unwrap();

        assert_eq!(res_vec, movie_clips);
    }
    #[allow(clippy::await_holding_lock)]
    #[rstest]
    #[tokio::test]
    async fn test_remove_by_id_movie_clip(mut router: Router) {
        let _m = MTX.lock().unwrap_or_else(|p_err| p_err.into_inner());

        let movie_clip_id = MovieClipId::generate();

        {
            let mock_ctx_ok = mock_movie_clip_usecases::remove_movie_clip_context();
            mock_ctx_ok
                .expect::<MockMovieClipRepositoryImpl>()
                .withf(move |_, cmd| cmd.id == movie_clip_id)
                .times(1)
                .return_const(Ok(()));

            let request = Request::builder()
                .method(http::Method::DELETE)
                .uri(&format!("/movie_clip/{movie_clip_id}"))
                .body(Body::empty())
                .unwrap();

            let response = router.ready().await.unwrap().call(request).await.unwrap();
            assert_eq!(response.status(), StatusCode::OK);
            let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
            assert!(body.is_empty());
        }
        {
            let mock_ctx_err = mock_movie_clip_usecases::remove_movie_clip_context();
            mock_ctx_err
                .expect::<MockMovieClipRepositoryImpl>()
                .withf(move |_, cmd| cmd.id == movie_clip_id)
                .times(1)
                .return_const(Err(AppCommonError::NoRecordError));

            let request = Request::builder()
                .method(http::Method::DELETE)
                .uri(&format!("/movie_clip/{movie_clip_id}"))
                .body(Body::empty())
                .unwrap();

            let response = router.ready().await.unwrap().call(request).await.unwrap();
            assert_ne!(response.status(), StatusCode::OK);

            let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
            let err: AppCommonError = serde_json::from_slice(&body).unwrap();

            assert_matches!(err, AppCommonError::NoRecordError);
        }
    }
}
