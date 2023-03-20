use crate::commands::video_commands;
use common::{AppCommonError, QueryInfo};
use domain::video::{Original, Video, VideoId};

// video_usecaseのモック化
#[cfg(not(test))]
use crate::usecases::video_usecases;

#[cfg(test)]
use crate::usecases::mock_video_usecases as video_usecases;

// AppStateのモック化
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
use std::str::FromStr;

pub async fn save_original(
    State(app_state): State<AppState>,
    video_res: Result<Json<Video<Original>>, JsonRejection>,
) -> Result<(), AppCommonError> {
    let video = video_res?.0;
    let cmd = video_commands::SaveVideoCommand::new(video);
    video_usecases::save_video(app_state.original_repo, cmd).await?;
    Ok(())
}

pub async fn edit_original(
    State(app_state): State<AppState>,
    video_res: Result<Json<Video<Original>>, JsonRejection>,
) -> Result<(), AppCommonError> {
    let video = video_res?.0;
    let cmd = video_commands::EditVideoCommand::new(video);
    video_usecases::edit_video(app_state.original_repo, cmd).await?;
    Ok(())
}

pub async fn increment_like_original(
    id: Result<Path<VideoId>, PathRejection>,
    State(app_state): State<AppState>,
) -> Result<(), AppCommonError> {
    let id = id?.0;
    let cmd = video_commands::IncrementLikeVideoCommand::new(id);
    video_usecases::increment_like_video(app_state.original_repo, cmd).await?;
    Ok(())
}

pub async fn all_originals(
    State(app_state): State<AppState>,
) -> Result<Json<Vec<Video<Original>>>, AppCommonError> {
    let cmd = video_commands::AllVideosCommand;
    let videos = video_usecases::all_videos(app_state.original_repo, cmd).await?;
    Ok(Json(videos))
}

#[derive(Deserialize, strum_macros::EnumString)]
#[serde(try_from = "String")]
#[strum(serialize_all = "snake_case")]
pub enum SortType {
    Date,
    Like,
}

impl TryFrom<String> for SortType {
    type Error = <SortType as FromStr>::Err;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        value.parse()
    }
}

#[derive(Deserialize)]
pub struct OriginalQuery {
    sort_type: SortType,
    length: Option<usize>,
}

pub async fn get_originals_with_query(
    path_query_res: Result<Query<OriginalQuery>, QueryRejection>,
    State(app_state): State<AppState>,
    query_info_res: Result<Json<QueryInfo>, JsonRejection>,
) -> Result<Json<Vec<Video<Original>>>, AppCommonError> {
    let path_query = path_query_res?.0;
    let reference_video = match query_info_res {
        // リクエストに正しいjsonが与えられた場合
        Ok(query_info) => query_info.0.reference_original,
        // Jsonが与えら無かった場合
        Err(JsonRejection::MissingJsonContentType(_)) => None,
        // その他のJsonRejection
        Err(e) => return Err(e.into()),
    };

    match (path_query.sort_type, path_query.length) {
        // ソートタイプがLikeの場合
        (SortType::Like, Some(length)) => {
            match reference_video {
                // QueryInfoにreference_originalが与えられた場合
                Some(reference_video) => {
                    let cmd =
                        video_commands::OrderByLikeLaterVideosCommand::new(reference_video, length);
                    let videos =
                        video_usecases::order_by_like_later_videos(app_state.original_repo, cmd)
                            .await?;
                    Ok(Json(videos))
                }
                // QueryInfoにreference_originalが与えられなかった場合
                None => {
                    let cmd = video_commands::OrderByLikeVideosCommand::new(length);
                    let videos =
                        video_usecases::order_by_like_videos(app_state.original_repo, cmd).await?;
                    Ok(Json(videos))
                }
            }
        }
        // ソートタイプがDateの場合
        (SortType::Date, Some(length)) => {
            match reference_video {
                // QueryInfoにreference_originalが与えられた場合
                Some(reference_video) => {
                    let cmd =
                        video_commands::OrderByDateLaterVideosCommand::new(reference_video, length);
                    let videos =
                        video_usecases::order_by_date_later_videos(app_state.original_repo, cmd)
                            .await?;
                    Ok(Json(videos))
                }
                // QueryInfoにreference_originalが与えられなかった場合
                None => {
                    let cmd = video_commands::OrderByDateVideosCommand::new(length);
                    let videos =
                        video_usecases::order_by_date_videos(app_state.original_repo, cmd).await?;
                    Ok(Json(videos))
                }
            }
        }
        // 無効なクエリの場合
        _ => Err(AppCommonError::QueryStringRejectionError(
            "Invalid query parameter combination.".to_string(),
        )),
    }
}

pub async fn remove_original(
    id: Result<Path<VideoId>, PathRejection>,
    State(app_state): State<AppState>,
) -> Result<(), AppCommonError> {
    let id = id?.0;
    let cmd = video_commands::RemoveVideoCommand::new(id);
    video_usecases::remove_video(app_state.original_repo, cmd).await?;
    Ok(())
}

#[cfg(test)]
mod test {
    use super::AppState;
    use crate::handlers::global::MTX;
    use crate::usecases::mock_video_usecases;
    use common::{AppCommonError, QueryInfo};
    use domain::video::{Original, Video, VideoId};
    use infrastructure::video_repository_impl::MockVideoOriginalRepository;

    use axum::{
        body::Body,
        http::{self, Request, StatusCode},
        routing::{delete, get, patch, put},
        Router,
    };
    use fake::{Fake, Faker};
    use pretty_assertions::{assert_eq, assert_ne};
    use rstest::{fixture, rstest};
    use tower::{Service, ServiceExt};

    #[fixture]
    fn router() -> Router {
        let app_state = AppState::default();

        Router::new()
            .route(
                "/original",
                put(super::save_original)
                    .patch(super::edit_original)
                    .get(super::all_originals),
            )
            .route("/original/query", get(super::get_originals_with_query))
            .route("/original/:id", delete(super::remove_original))
            .route(
                "/original/increment_like/:id",
                patch(super::increment_like_original),
            )
            .with_state(app_state)
    }

    #[allow(clippy::await_holding_lock)]
    #[rstest]
    #[tokio::test]
    async fn test_save_video(mut router: Router) {
        let _m = MTX.lock().unwrap_or_else(|p_err| p_err.into_inner());

        let video = Faker.fake::<Video<Original>>();

        {
            let mock_ctx_ok = mock_video_usecases::save_video_context();
            mock_ctx_ok
                .expect::<MockVideoOriginalRepository, Original>()
                .withf({
                    let video = video.clone();
                    move |_, cmd| cmd.video == video
                })
                .times(1)
                .return_const(Ok(()));

            let request = Request::builder()
                .method(http::Method::PUT)
                .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                .uri("/original")
                .body(Body::from(serde_json::to_vec(&video).unwrap()))
                .unwrap();

            let response = router.ready().await.unwrap().call(request).await.unwrap();
            assert_eq!(response.status(), StatusCode::OK);
            let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
            assert!(body.is_empty());
        }
        {
            let mock_ctx_err = mock_video_usecases::save_video_context();
            mock_ctx_err
                .expect::<MockVideoOriginalRepository, Original>()
                .withf({
                    let video = video.clone();
                    move |_, cmd| cmd.video == video
                })
                .times(1)
                .return_const(Err(AppCommonError::ConflictError));

            let request = Request::builder()
                .method(http::Method::PUT)
                .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                .uri("/original")
                .body(Body::from(serde_json::to_vec(&video).unwrap()))
                .unwrap();

            let response = router.ready().await.unwrap().call(request).await.unwrap();
            assert_ne!(response.status(), StatusCode::OK);

            let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
            let err: AppCommonError = serde_json::from_slice(&body).unwrap();

            assert!(matches!(err, AppCommonError::ConflictError));
        }
    }

    #[allow(clippy::await_holding_lock)]
    #[rstest]
    #[tokio::test]
    async fn test_edit_video(mut router: Router) {
        let _m = MTX.lock().unwrap_or_else(|p_err| p_err.into_inner());

        let video = Faker.fake::<Video<Original>>();

        {
            let mock_ctx_ok = mock_video_usecases::edit_video_context();
            mock_ctx_ok
                .expect::<MockVideoOriginalRepository, Original>()
                .withf({
                    let video = video.clone();
                    move |_, cmd| cmd.video == video
                })
                .times(1)
                .return_const(Ok(()));

            let request = Request::builder()
                .method(http::Method::PATCH)
                .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                .uri("/original")
                .body(Body::from(serde_json::to_vec(&video).unwrap()))
                .unwrap();

            let response = router.ready().await.unwrap().call(request).await.unwrap();
            assert_eq!(response.status(), StatusCode::OK);
            let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
            assert!(body.is_empty());
        }
        {
            let mock_ctx_err = mock_video_usecases::edit_video_context();
            mock_ctx_err
                .expect::<MockVideoOriginalRepository, Original>()
                .withf({
                    let video = video.clone();
                    move |_, cmd| cmd.video == video
                })
                .times(1)
                .return_const(Err(AppCommonError::NoRecordError));

            let request = Request::builder()
                .method(http::Method::PATCH)
                .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                .uri("/original")
                .body(Body::from(serde_json::to_vec(&video).unwrap()))
                .unwrap();

            let response = router.ready().await.unwrap().call(request).await.unwrap();
            assert_ne!(response.status(), StatusCode::OK);

            let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
            let err: AppCommonError = serde_json::from_slice(&body).unwrap();

            assert!(matches!(err, AppCommonError::NoRecordError));
        }
    }

    #[allow(clippy::await_holding_lock)]
    #[rstest]
    #[tokio::test]
    async fn test_increment_like_video(mut router: Router) {
        let _m = MTX.lock().unwrap_or_else(|p_err| p_err.into_inner());

        let video_id = VideoId::generate();

        {
            let mock_ctx_ok = mock_video_usecases::increment_like_video_context();
            mock_ctx_ok
                .expect::<MockVideoOriginalRepository, Original>()
                .withf(move |_, cmd| cmd.id == video_id)
                .times(1)
                .return_const(Ok(()));

            let request = Request::builder()
                .method(http::Method::PATCH)
                .uri(&format!("/original/increment_like/{video_id}"))
                .body(Body::empty())
                .unwrap();

            let response = router.ready().await.unwrap().call(request).await.unwrap();
            assert_eq!(response.status(), StatusCode::OK);
            let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
            assert!(body.is_empty());
        }
        {
            let mock_ctx_err = mock_video_usecases::increment_like_video_context();
            mock_ctx_err
                .expect::<MockVideoOriginalRepository, Original>()
                .withf(move |_, cmd| cmd.id == video_id)
                .times(1)
                .return_const(Err(AppCommonError::NoRecordError));

            let request = Request::builder()
                .method(http::Method::PATCH)
                .uri(&format!("/original/increment_like/{video_id}"))
                .body(Body::empty())
                .unwrap();

            let response = router.ready().await.unwrap().call(request).await.unwrap();
            assert_ne!(response.status(), StatusCode::OK);

            let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
            let err: AppCommonError = serde_json::from_slice(&body).unwrap();

            assert!(matches!(err, AppCommonError::NoRecordError));
        }
    }

    #[allow(clippy::await_holding_lock)]
    #[rstest]
    #[tokio::test]
    async fn test_all_movie_clips(mut router: Router) {
        let _m = MTX.lock().unwrap_or_else(|p_err| p_err.into_inner());
        let videos = vec![Faker.fake::<Video<Original>>(); 100];

        let mock_ctx = mock_video_usecases::all_videos_context();
        mock_ctx
            .expect::<MockVideoOriginalRepository, Original>()
            .times(1)
            .return_const(Ok(videos.clone()));

        let request = Request::builder()
            .method(http::Method::GET)
            .uri("/original")
            .body(Body::empty())
            .unwrap();

        let response = router.ready().await.unwrap().call(request).await.unwrap();
        assert_eq!(response.status(), StatusCode::OK);

        let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
        let res_vec: Vec<Video<Original>> = serde_json::from_slice(&body).unwrap();

        assert_eq!(res_vec, videos);
    }

    #[allow(clippy::await_holding_lock)]
    #[rstest]
    #[tokio::test]
    async fn test_order_by_like_videos(mut router: Router) {
        let _m = MTX.lock().unwrap_or_else(|p_err| p_err.into_inner());
        let videos = vec![Faker.fake::<Video<Original>>(); 100];

        let length = 100_usize;

        let mock_ctx = mock_video_usecases::order_by_like_videos_context();
        mock_ctx
            .expect::<MockVideoOriginalRepository, Original>()
            .withf(move |_, cmd| cmd.length == length)
            .times(1)
            .return_const(Ok(videos.clone()));

        let request = Request::builder()
            .method(http::Method::GET)
            .uri(&format!("/original/query?sort_type=like&length={length}"))
            .body(Body::empty())
            .unwrap();

        let response = router.ready().await.unwrap().call(request).await.unwrap();
        assert_eq!(response.status(), StatusCode::OK);

        let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
        let res_vec: Vec<Video<Original>> = serde_json::from_slice(&body).unwrap();

        assert_eq!(res_vec, videos);
    }

    #[allow(clippy::await_holding_lock)]
    #[rstest]
    #[tokio::test]
    async fn test_order_by_like_later_videos(mut router: Router) {
        let _m = MTX.lock().unwrap_or_else(|p_err| p_err.into_inner());
        let videos = vec![Faker.fake::<Video<Original>>(); 100];

        let reference = Faker.fake::<Video<Original>>();
        let length = 100_usize;

        let mock_ctx = mock_video_usecases::order_by_like_later_videos_context();
        mock_ctx
            .expect::<MockVideoOriginalRepository, Original>()
            .withf({
                let reference = reference.clone();
                move |_, cmd| cmd.reference == reference && cmd.length == length
            })
            .times(1)
            .return_const(Ok(videos.clone()));

        let query_info = QueryInfo::builder().reference_original(reference).build();

        let request = Request::builder()
            .method(http::Method::GET)
            .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
            .uri(&format!("/original/query?sort_type=like&length={length}"))
            .body(Body::from(serde_json::to_vec(&query_info).unwrap()))
            .unwrap();

        let response = router.ready().await.unwrap().call(request).await.unwrap();
        assert_eq!(response.status(), StatusCode::OK);

        let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
        let res_vec: Vec<Video<Original>> = serde_json::from_slice(&body).unwrap();

        assert_eq!(res_vec, videos);
    }

    #[allow(clippy::await_holding_lock)]
    #[rstest]
    #[tokio::test]
    async fn test_order_by_date_videos(mut router: Router) {
        let _m = MTX.lock().unwrap_or_else(|p_err| p_err.into_inner());
        let videos = vec![Faker.fake::<Video<Original>>(); 100];

        let length = 100_usize;

        let mock_ctx = mock_video_usecases::order_by_date_videos_context();
        mock_ctx
            .expect::<MockVideoOriginalRepository, Original>()
            .withf(move |_, cmd| cmd.length == length)
            .times(1)
            .return_const(Ok(videos.clone()));

        let request = Request::builder()
            .method(http::Method::GET)
            .uri(&format!("/original/query?sort_type=date&length={length}"))
            .body(Body::empty())
            .unwrap();

        let response = router.ready().await.unwrap().call(request).await.unwrap();
        assert_eq!(response.status(), StatusCode::OK);

        let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
        let res_vec: Vec<Video<Original>> = serde_json::from_slice(&body).unwrap();

        assert_eq!(res_vec, videos);
    }

    #[allow(clippy::await_holding_lock)]
    #[rstest]
    #[tokio::test]
    async fn test_order_by_date_later_videos(mut router: Router) {
        let _m = MTX.lock().unwrap_or_else(|p_err| p_err.into_inner());
        let videos = vec![Faker.fake::<Video<Original>>(); 100];

        let reference = Faker.fake::<Video<Original>>();
        let length = 100_usize;

        let mock_ctx = mock_video_usecases::order_by_date_later_videos_context();
        mock_ctx
            .expect::<MockVideoOriginalRepository, Original>()
            .withf({
                let reference = reference.clone();
                move |_, cmd| cmd.reference == reference && cmd.length == length
            })
            .times(1)
            .return_const(Ok(videos.clone()));

        let query_info = QueryInfo::builder().reference_original(reference).build();

        let request = Request::builder()
            .method(http::Method::GET)
            .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
            .uri(&format!("/original/query?sort_type=date&length={length}"))
            .body(Body::from(serde_json::to_vec(&query_info).unwrap()))
            .unwrap();

        let response = router.ready().await.unwrap().call(request).await.unwrap();
        assert_eq!(response.status(), StatusCode::OK);

        let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
        let res_vec: Vec<Video<Original>> = serde_json::from_slice(&body).unwrap();

        assert_eq!(res_vec, videos);
    }

    #[allow(clippy::await_holding_lock)]
    #[rstest]
    #[tokio::test]
    async fn test_remove_video(mut router: Router) {
        let _m = MTX.lock().unwrap_or_else(|p_err| p_err.into_inner());

        let video_id = VideoId::generate();

        {
            let mock_ctx_ok = mock_video_usecases::remove_video_context();
            mock_ctx_ok
                .expect::<MockVideoOriginalRepository, Original>()
                .withf(move |_, cmd| cmd.id == video_id)
                .times(1)
                .return_const(Ok(()));

            let request = Request::builder()
                .method(http::Method::DELETE)
                .uri(&format!("/original/{video_id}"))
                .body(Body::empty())
                .unwrap();

            let response = router.ready().await.unwrap().call(request).await.unwrap();
            assert_eq!(response.status(), StatusCode::OK);
            let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
            assert!(body.is_empty());
        }
        {
            let mock_ctx_err = mock_video_usecases::remove_video_context();
            mock_ctx_err
                .expect::<MockVideoOriginalRepository, Original>()
                .withf(move |_, cmd| cmd.id == video_id)
                .times(1)
                .return_const(Err(AppCommonError::NoRecordError));

            let request = Request::builder()
                .method(http::Method::DELETE)
                .uri(&format!("/original/{video_id}"))
                .body(Body::empty())
                .unwrap();

            let response = router.ready().await.unwrap().call(request).await.unwrap();
            assert_ne!(response.status(), StatusCode::OK);

            let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
            let err: AppCommonError = serde_json::from_slice(&body).unwrap();

            assert!(matches!(err, AppCommonError::NoRecordError));
        }
    }
}
