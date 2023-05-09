use crate::commands::video_commands;
use common::{AppCommonError, QueryInfo};
use domain::video::{Video, VideoId, VideoType};

// video_usecaseのモック化
#[cfg(not(test))]
use crate::usecases::video_usecases;

#[cfg(test)]
use crate::usecases::mock_video_usecases as video_usecases;

// VideoRepoのモック化
#[cfg(all(not(test), feature = "inmemory"))]
use infrastructure::video_repository_impl::InMemoryVideoRepository as VideoRepositoryImpl;

#[cfg(all(not(test), not(feature = "inmemory")))]
use infrastructure::video_repository_impl::VideoPgDbRepository as VideoRepositoryImpl;

#[cfg(test)]
use infrastructure::video_repository_impl::InMemoryVideoRepository as VideoRepositoryImpl; // 実際には利用しない．

use axum::{
    extract::rejection::{JsonRejection, PathRejection, QueryRejection},
    extract::{Json, Path, Query, State},
};
use serde::Deserialize;
use std::str::FromStr;
use std::sync::Arc;
use tracing_attributes::instrument;

#[instrument(skip(video_repo), err(Display))]
pub async fn save_video<T: VideoType + 'static>(
    State(video_repo): State<Arc<VideoRepositoryImpl<T>>>,
    video_res: Result<Json<Video<T>>, JsonRejection>,
) -> Result<(), AppCommonError> {
    let video = video_res?.0;
    let cmd = video_commands::SaveVideoCommand::<T>::new(video);
    video_usecases::save_video(video_repo, cmd).await?;
    Ok(())
}

#[instrument(skip(video_repo), err(Display))]
pub async fn edit_video<T: VideoType + 'static>(
    State(video_repo): State<Arc<VideoRepositoryImpl<T>>>,
    video_res: Result<Json<Video<T>>, JsonRejection>,
) -> Result<(), AppCommonError> {
    let video = video_res?.0;
    let cmd = video_commands::EditVideoCommand::<T>::new(video);
    video_usecases::edit_video(video_repo, cmd).await?;
    Ok(())
}

#[instrument(skip(video_repo), err(Display))]
pub async fn increment_like_video<T: VideoType + 'static>(
    id: Result<Path<VideoId>, PathRejection>,
    State(video_repo): State<Arc<VideoRepositoryImpl<T>>>,
) -> Result<(), AppCommonError> {
    let id = id?.0;
    let cmd = video_commands::IncrementLikeVideoCommand::new(id);
    video_usecases::increment_like_video::<VideoRepositoryImpl<T>, _>(video_repo, cmd).await?;
    Ok(())
}

#[instrument(skip(video_repo), err(Display))]
pub async fn all_videos<T: VideoType + 'static>(
    State(video_repo): State<Arc<VideoRepositoryImpl<T>>>,
) -> Result<Json<Vec<Video<T>>>, AppCommonError> {
    let cmd = video_commands::AllVideosCommand;
    let videos = video_usecases::all_videos::<VideoRepositoryImpl<T>, _>(video_repo, cmd).await?;
    Ok(Json(videos))
}

#[derive(Deserialize, strum_macros::EnumString, Debug)]
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

#[derive(Deserialize, Debug)]
pub struct VideoQuery {
    sort_type: SortType,
    length: Option<usize>,
}

#[instrument(skip(video_repo), err(Display))]
pub async fn get_videos_with_query<T: VideoType + 'static>(
    path_query_res: Result<Query<VideoQuery>, QueryRejection>,
    State(video_repo): State<Arc<VideoRepositoryImpl<T>>>,
    query_info_res: Result<Json<QueryInfo<Video<T>>>, JsonRejection>,
) -> Result<Json<Vec<Video<T>>>, AppCommonError> {
    let path_query = path_query_res?.0;
    let reference_video = match query_info_res {
        // リクエストに正しいjsonが与えられた場合
        Ok(query_info) => query_info.0.reference,
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
                        video_usecases::order_by_like_later_videos(video_repo, cmd).await?;
                    Ok(Json(videos))
                }
                // QueryInfoにreference_originalが与えられなかった場合
                None => {
                    let cmd = video_commands::OrderByLikeVideosCommand::new(length);
                    let videos = video_usecases::order_by_like_videos(video_repo, cmd).await?;
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
                        video_usecases::order_by_date_later_videos(video_repo, cmd).await?;
                    Ok(Json(videos))
                }
                // QueryInfoにreference_originalが与えられなかった場合
                None => {
                    let cmd = video_commands::OrderByDateVideosCommand::new(length);
                    let videos = video_usecases::order_by_date_videos(video_repo, cmd).await?;
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

#[instrument(skip(video_repo), err(Display))]
pub async fn remove_video<T: VideoType + 'static>(
    id: Result<Path<VideoId>, PathRejection>,
    State(video_repo): State<Arc<VideoRepositoryImpl<T>>>,
) -> Result<(), AppCommonError> {
    let id = id?.0;
    let cmd = video_commands::RemoveVideoCommand::new(id);
    video_usecases::remove_video(video_repo, cmd).await?;
    Ok(())
}

#[cfg(test)]
mod test {
    use crate::usecases::mock_video_usecases;
    use common::{AppCommonError, QueryInfoRef};
    use domain::video::{Original, Video, VideoId};
    use infrastructure::video_repository_impl::InMemoryVideoRepository;

    use axum::{
        body::Body,
        http::{self, Request, StatusCode},
        routing::{delete, get, patch, put},
        Router,
    };
    use fake::{Fake, Faker};
    use pretty_assertions::{assert_eq, assert_ne};
    use rstest::{fixture, rstest};
    use serial_test::serial;
    use std::borrow::Cow;
    use std::sync::Arc;
    use tower::{Service, ServiceExt};

    #[fixture]
    fn router() -> Router {
        let video_repo = Arc::new(InMemoryVideoRepository::<Original>::new());

        Router::new()
            .route(
                "/original",
                put(super::save_video::<Original>)
                    .patch(super::edit_video::<Original>)
                    .get(super::all_videos::<Original>),
            )
            .route(
                "/original/query",
                get(super::get_videos_with_query::<Original>),
            )
            .route("/original/:id", delete(super::remove_video::<Original>))
            .route(
                "/original/increment_like/:id",
                patch(super::increment_like_video::<Original>),
            )
            .with_state(video_repo)
    }

    #[fixture]
    fn videos() -> Vec<Video<Original>> {
        (0..100)
            .map(|_| Faker.fake::<Video<Original>>())
            .collect::<Vec<_>>()
    }

    #[rstest]
    #[tokio::test]
    #[serial("mock_video")]
    async fn test_save_video(mut router: Router) {
        let video = Faker.fake::<Video<Original>>();

        {
            let mock_ctx_ok = mock_video_usecases::save_video_context();
            mock_ctx_ok
                .expect::<InMemoryVideoRepository<Original>, Original>()
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
                .expect::<InMemoryVideoRepository<Original>, Original>()
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

    #[rstest]
    #[tokio::test]
    #[serial("mock_video")]
    async fn test_edit_video(mut router: Router) {
        let video = Faker.fake::<Video<Original>>();

        {
            let mock_ctx_ok = mock_video_usecases::edit_video_context();
            mock_ctx_ok
                .expect::<InMemoryVideoRepository<Original>, Original>()
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
                .expect::<InMemoryVideoRepository<Original>, Original>()
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

    #[rstest]
    #[tokio::test]
    #[serial("mock_video")]
    async fn test_increment_like_video(mut router: Router) {
        let video_id = VideoId::generate();

        {
            let mock_ctx_ok = mock_video_usecases::increment_like_video_context();
            mock_ctx_ok
                .expect::<InMemoryVideoRepository<Original>, Original>()
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
                .expect::<InMemoryVideoRepository<Original>, Original>()
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

    #[rstest]
    #[tokio::test]
    #[serial("mock_video")]
    async fn test_all_movie_clips(mut router: Router, videos: Vec<Video<Original>>) {
        let mock_ctx = mock_video_usecases::all_videos_context();
        mock_ctx
            .expect::<InMemoryVideoRepository<Original>, Original>()
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

    #[rstest]
    #[tokio::test]
    #[serial("mock_video")]
    async fn test_order_by_like_videos(mut router: Router, videos: Vec<Video<Original>>) {
        let length = 100_usize;

        let mock_ctx = mock_video_usecases::order_by_like_videos_context();
        mock_ctx
            .expect::<InMemoryVideoRepository<Original>, Original>()
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

    #[rstest]
    #[tokio::test]
    #[serial("mock_video")]
    async fn test_order_by_like_later_videos(mut router: Router, videos: Vec<Video<Original>>) {
        let reference = Faker.fake::<Video<Original>>();
        let length = 100_usize;

        let mock_ctx = mock_video_usecases::order_by_like_later_videos_context();
        mock_ctx
            .expect::<InMemoryVideoRepository<Original>, Original>()
            .withf({
                let reference = reference.clone();
                move |_, cmd| cmd.reference == reference && cmd.length == length
            })
            .times(1)
            .return_const(Ok(videos.clone()));

        let query_info = QueryInfoRef::builder()
            .reference(Cow::Borrowed(&reference))
            .build();

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

    #[rstest]
    #[tokio::test]
    #[serial("mock_video")]
    async fn test_order_by_date_videos(mut router: Router, videos: Vec<Video<Original>>) {
        let length = 100_usize;

        let mock_ctx = mock_video_usecases::order_by_date_videos_context();
        mock_ctx
            .expect::<InMemoryVideoRepository<Original>, Original>()
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

    #[rstest]
    #[tokio::test]
    #[serial("mock_video")]
    async fn test_order_by_date_later_videos(mut router: Router, videos: Vec<Video<Original>>) {
        let reference = Faker.fake::<Video<Original>>();
        let length = 100_usize;

        let mock_ctx = mock_video_usecases::order_by_date_later_videos_context();
        mock_ctx
            .expect::<InMemoryVideoRepository<Original>, Original>()
            .withf({
                let reference = reference.clone();
                move |_, cmd| cmd.reference == reference && cmd.length == length
            })
            .times(1)
            .return_const(Ok(videos.clone()));

        let query_info = QueryInfoRef::builder()
            .reference(Cow::Borrowed(&reference))
            .build();

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

    #[rstest]
    #[tokio::test]
    #[serial("mock_video")]
    async fn test_remove_video(mut router: Router) {
        let video_id = VideoId::generate();

        {
            let mock_ctx_ok = mock_video_usecases::remove_video_context();
            mock_ctx_ok
                .expect::<InMemoryVideoRepository<Original>, Original>()
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
                .expect::<InMemoryVideoRepository<Original>, Original>()
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
