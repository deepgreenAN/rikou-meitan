use crate::commands::movie_clip_commands;
use common::{AppCommonError, QueryInfo};
use domain::movie_clip::{MovieClip, MovieClipId};
use domain::Date;

// movie_clip_usecaseのモック化
#[cfg(not(test))]
use crate::usecases::movie_clip_usecases;

#[cfg(test)]
use crate::usecases::mock_movie_clip_usecases as movie_clip_usecases;

// MovieClipRepositoryのモック化
#[cfg(all(not(test), feature = "inmemory"))]
use infrastructure::movie_clip_repository_impl::InMemoryMovieClipRepository as MovieClipRepositoryImpl;

#[cfg(all(not(test), not(feature = "inmemory")))]
use infrastructure::movie_clip_repository_impl::MovieClipPgDBRepository as MovieClipRepositoryImpl;

#[cfg(test)]
use infrastructure::movie_clip_repository_impl::MockMovieClipRepository as MovieClipRepositoryImpl;

use axum::{
    extract::rejection::{JsonRejection, PathRejection, QueryRejection},
    extract::{Json, Path, Query, State},
};
use serde::Deserialize;
use std::str::FromStr;
use std::sync::Arc;
use tracing_attributes::instrument;

#[instrument(skip(movie_clip_repo), err(Display))]
pub async fn save_movie_clip(
    State(movie_clip_repo): State<Arc<MovieClipRepositoryImpl>>,
    movie_clip_res: Result<Json<MovieClip>, JsonRejection>,
) -> Result<(), AppCommonError> {
    let movie_clip = movie_clip_res?.0;

    let cmd = movie_clip_commands::SaveMovieClipCommand::new(movie_clip);
    movie_clip_usecases::save_movie_clip(movie_clip_repo, cmd).await?;
    Ok(())
}

#[instrument(skip(movie_clip_repo), err(Display))]
pub async fn edit_movie_clip(
    State(movie_clip_repo): State<Arc<MovieClipRepositoryImpl>>,
    movie_clip_res: Result<Json<MovieClip>, JsonRejection>,
) -> Result<(), AppCommonError> {
    let movie_clip = movie_clip_res?.0;

    let cmd = movie_clip_commands::EditMovieClipCommand::new(movie_clip);
    movie_clip_usecases::edit_movie_clip(movie_clip_repo, cmd).await?;

    Ok(())
}

#[instrument(skip(movie_clip_repo), err(Display))]
pub async fn increment_like_movie_clip(
    id: Result<Path<MovieClipId>, PathRejection>,
    State(movie_clip_repo): State<Arc<MovieClipRepositoryImpl>>,
) -> Result<(), AppCommonError> {
    let id = id?.0;
    let cmd = movie_clip_commands::IncrementLikeMovieClipCommand::new(id);
    movie_clip_usecases::increment_like_movie_clip(movie_clip_repo, cmd).await?;
    Ok(())
}

#[instrument(skip(movie_clip_repo), err(Display))]
pub async fn all_movie_clips(
    State(movie_clip_repo): State<Arc<MovieClipRepositoryImpl>>,
) -> Result<Json<Vec<MovieClip>>, AppCommonError> {
    let cmd = movie_clip_commands::AllMovieClipCommand;
    let movie_clips = movie_clip_usecases::all_movie_clips(movie_clip_repo, cmd).await?;
    Ok(Json(movie_clips))
}

#[derive(Deserialize, strum_macros::EnumString, Debug)]
#[serde(try_from = "String")]
#[strum(serialize_all = "snake_case")]
pub enum SortType {
    CreateDate,
    Like,
}

impl TryFrom<String> for SortType {
    type Error = <SortType as FromStr>::Err;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        value.parse()
    }
}

#[derive(Deserialize, Debug)]
pub struct MovieClipQuery {
    sort_type: SortType,
    length: Option<usize>,
    start: Option<Date>,
    end: Option<Date>,
}

#[instrument(skip(movie_clip_repo), err(Display))]
pub async fn get_movie_clips_with_query(
    query_res: Result<Query<MovieClipQuery>, QueryRejection>,
    State(movie_clip_repo): State<Arc<MovieClipRepositoryImpl>>,
    query_info_res: Result<Json<QueryInfo<MovieClip>>, JsonRejection>,
) -> Result<Json<Vec<MovieClip>>, AppCommonError> {
    let query = query_res?.0;
    let movie_clip_reference = match query_info_res {
        // リクエストにjsonが与えられた場合
        Ok(reference) => reference.0.reference,
        // // リクエストにjsonが与えられない場合
        Err(JsonRejection::MissingJsonContentType(_)) => None,
        // その他のJsonRejection
        Err(e) => return Err(e.into()),
    };

    match (query.sort_type, query.length, query.start, query.end) {
        // Likeでソートする場合
        (SortType::Like, Some(length), None, None) => {
            match movie_clip_reference {
                // referenceが存在する場合
                Some(reference) => {
                    let cmd = movie_clip_commands::OrderByLikeLaterMovieClipCommand::new(
                        reference, length,
                    );
                    let clips =
                        movie_clip_usecases::order_by_like_later_movie_clips(movie_clip_repo, cmd)
                            .await?;
                    Ok(Json(clips))
                }
                // referenceが存在しない場合
                None => {
                    let cmd = movie_clip_commands::OrderByLikeMovieClipCommand::new(length);
                    let clips =
                        movie_clip_usecases::order_by_like_movie_clips(movie_clip_repo, cmd)
                            .await?;
                    Ok(Json(clips))
                }
            }
        }
        // CreateDateでソートしlengthを指定する場合
        (SortType::CreateDate, Some(length), None, None) => {
            match movie_clip_reference {
                // referenceが存在する場合
                Some(reference) => {
                    let cmd = movie_clip_commands::OrderByCreateDateLaterMovieClipCommand::new(
                        reference, length,
                    );
                    let clips = movie_clip_usecases::order_by_create_date_later_movie_clips(
                        movie_clip_repo,
                        cmd,
                    )
                    .await?;
                    Ok(Json(clips))
                }
                // referenceが存在しない場合
                None => {
                    let cmd = movie_clip_commands::OrderByCreateDateMovieClipCommand::new(length);
                    let clips =
                        movie_clip_usecases::order_by_create_date_movie_clips(movie_clip_repo, cmd)
                            .await?;
                    Ok(Json(clips))
                }
            }
        }
        // CreateDateでソートしstartとendを指定する場合
        (SortType::CreateDate, None, Some(start), Some(end)) => {
            let cmd = movie_clip_commands::OrderByCreateDateRangeMovieClipCommand::new(start, end);
            let clips =
                movie_clip_usecases::order_by_create_date_range_movie_clips(movie_clip_repo, cmd)
                    .await?;
            Ok(Json(clips))
        }
        // 無効なクエリの場合
        _ => Err(AppCommonError::QueryStringRejectionError(
            "Invalid query parameter combination.".to_string(),
        )),
    }
}

#[instrument(skip(movie_clip_repo), err(Display))]
pub async fn remove_movie_clip(
    id: Result<Path<MovieClipId>, PathRejection>,
    State(movie_clip_repo): State<Arc<MovieClipRepositoryImpl>>,
) -> Result<(), AppCommonError> {
    let id = id?.0;
    let cmd = movie_clip_commands::RemoveMovieClipCommand::new(id);
    movie_clip_usecases::remove_movie_clip(movie_clip_repo, cmd).await?;
    Ok(())
}

#[cfg(test)]
mod test {
    use crate::usecases::mock_movie_clip_usecases;
    use common::{AppCommonError, QueryInfoRef};
    use domain::movie_clip::{MovieClip, MovieClipId};
    use domain::Date;
    use infrastructure::movie_clip_repository_impl::MockMovieClipRepository;

    use axum::{
        body::Body,
        http::{self, Request, StatusCode},
        routing::{delete, get, patch, put},
        Router,
    };
    use fake::{Fake, Faker};
    use once_cell::sync::Lazy;
    use pretty_assertions::{assert_eq, assert_ne};
    use rstest::{fixture, rstest};
    use std::borrow::Cow;
    use std::sync::{Arc, Mutex};
    use tower::{Service, ServiceExt};

    static MTX: Lazy<Mutex<()>> = Lazy::new(|| Mutex::new(()));

    #[fixture]
    fn router() -> Router {
        let movie_clip_repo = Arc::new(MockMovieClipRepository::new());

        Router::new()
            .route(
                "/movie_clip",
                put(super::save_movie_clip)
                    .patch(super::edit_movie_clip)
                    .get(super::all_movie_clips),
            )
            .route("/movie_clip/query", get(super::get_movie_clips_with_query))
            .route("/movie_clip/:id", delete(super::remove_movie_clip))
            .route(
                "/movie_clip/increment_like/:id",
                patch(super::increment_like_movie_clip),
            )
            .with_state(movie_clip_repo)
    }

    #[fixture]
    fn movie_clips() -> Vec<MovieClip> {
        (0..100)
            .map(|_| Faker.fake::<MovieClip>())
            .collect::<Vec<_>>()
    }

    #[allow(clippy::await_holding_lock)]
    #[rstest]
    #[tokio::test]
    async fn test_save_movie_clip(mut router: Router) {
        let _m = MTX.lock().unwrap_or_else(|p_err| p_err.into_inner());

        let movie_clip = Faker.fake::<MovieClip>();

        {
            let mock_ctx_ok = mock_movie_clip_usecases::save_movie_clip_context();
            mock_ctx_ok
                .expect::<MockMovieClipRepository>()
                .withf({
                    let movie_clip = movie_clip.clone();
                    move |_, cmd| cmd.movie_clip == movie_clip
                })
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
            let mock_ctx_err = mock_movie_clip_usecases::save_movie_clip_context();
            mock_ctx_err
                .expect::<MockMovieClipRepository>()
                .withf({
                    let movie_clip = movie_clip.clone();
                    move |_, cmd| cmd.movie_clip == movie_clip
                })
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

            assert!(matches!(err, AppCommonError::ConflictError));
        }
    }

    #[allow(clippy::await_holding_lock)]
    #[rstest]
    #[tokio::test]
    async fn test_edit_movie_clip(mut router: Router) {
        let _m = MTX.lock().unwrap_or_else(|p_err| p_err.into_inner());

        let movie_clip = Faker.fake::<MovieClip>();

        {
            let mock_ctx_ok = mock_movie_clip_usecases::edit_movie_clip_context();
            mock_ctx_ok
                .expect::<MockMovieClipRepository>()
                .withf({
                    let movie_clip = movie_clip.clone();
                    move |_, cmd| cmd.movie_clip == movie_clip
                })
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
            let mock_ctx_err = mock_movie_clip_usecases::edit_movie_clip_context();
            mock_ctx_err
                .expect::<MockMovieClipRepository>()
                .withf({
                    let movie_clip = movie_clip.clone();
                    move |_, cmd| cmd.movie_clip == movie_clip
                })
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

            assert!(matches!(err, AppCommonError::NoRecordError));
        }
    }

    #[allow(clippy::await_holding_lock)]
    #[rstest]
    #[tokio::test]
    async fn test_increment_like_movie_clip(mut router: Router) {
        let _m = MTX.lock().unwrap_or_else(|p_err| p_err.into_inner());

        let movie_clip_id = MovieClipId::generate();

        {
            let mock_ctx_ok = mock_movie_clip_usecases::increment_like_movie_clip_context();
            mock_ctx_ok
                .expect::<MockMovieClipRepository>()
                .withf(move |_, cmd| cmd.id == movie_clip_id)
                .times(1)
                .return_const(Ok(()));

            let request = Request::builder()
                .method(http::Method::PATCH)
                .uri(&format!("/movie_clip/increment_like/{movie_clip_id}"))
                .body(Body::empty())
                .unwrap();

            let response = router.ready().await.unwrap().call(request).await.unwrap();
            assert_eq!(response.status(), StatusCode::OK);
            let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
            assert!(body.is_empty());
        }
        {
            let mock_ctx_err = mock_movie_clip_usecases::increment_like_movie_clip_context();
            mock_ctx_err
                .expect::<MockMovieClipRepository>()
                .withf(move |_, cmd| cmd.id == movie_clip_id)
                .times(1)
                .return_const(Err(AppCommonError::NoRecordError));

            let request = Request::builder()
                .method(http::Method::PATCH)
                .uri(&format!("/movie_clip/increment_like/{movie_clip_id}"))
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
    async fn test_all_movie_clips(mut router: Router, movie_clips: Vec<MovieClip>) {
        let _m = MTX.lock().unwrap_or_else(|p_err| p_err.into_inner());

        let mock_ctx = mock_movie_clip_usecases::all_movie_clips_context();
        mock_ctx
            .expect::<MockMovieClipRepository>()
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
    async fn test_order_by_like_movie_clips(mut router: Router, movie_clips: Vec<MovieClip>) {
        let _m = MTX.lock().unwrap_or_else(|p_err| p_err.into_inner());

        let length = 100_usize;

        let mock_ctx = mock_movie_clip_usecases::order_by_like_movie_clips_context();
        mock_ctx
            .expect::<MockMovieClipRepository>()
            .withf(move |_, cmd| cmd.length == length)
            .times(1)
            .return_const(Ok(movie_clips.clone()));

        let request = Request::builder()
            .method(http::Method::GET)
            .uri(&format!("/movie_clip/query?sort_type=like&length={length}"))
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
    async fn test_order_by_like_later_movie_clips(mut router: Router, movie_clips: Vec<MovieClip>) {
        let _m = MTX.lock().unwrap_or_else(|p_err| p_err.into_inner());

        let reference = Faker.fake::<MovieClip>();
        let length = 100_usize;

        let mock_ctx = mock_movie_clip_usecases::order_by_like_later_movie_clips_context();
        mock_ctx
            .expect::<MockMovieClipRepository>()
            .withf({
                let reference = reference.clone();
                move |_, cmd| cmd.reference == reference && cmd.length == length
            })
            .times(1)
            .return_const(Ok(movie_clips.clone()));

        let query_info = QueryInfoRef::builder()
            .reference(Cow::Borrowed(&reference))
            .build();

        let request = Request::builder()
            .method(http::Method::GET)
            .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
            .uri(&format!("/movie_clip/query?sort_type=like&length={length}"))
            .body(Body::from(serde_json::to_vec(&query_info).unwrap()))
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
    async fn test_order_by_crate_date_range_movie_clips(
        mut router: Router,
        movie_clips: Vec<MovieClip>,
    ) {
        let _m = MTX.lock().unwrap_or_else(|p_err| p_err.into_inner());

        let start = Faker.fake::<Date>();
        let end = Faker.fake::<Date>();

        let mock_ctx = mock_movie_clip_usecases::order_by_create_date_range_movie_clips_context();
        mock_ctx
            .expect::<MockMovieClipRepository>()
            .withf(move |_, cmd| cmd.start == start && cmd.end == end)
            .times(1)
            .return_const(Ok(movie_clips.clone()));

        let request = Request::builder()
            .method(http::Method::GET)
            .uri(&format!(
                "/movie_clip/query?sort_type=create_date&start={start}&end={end}"
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
    async fn test_order_by_create_date_movie_clips(
        mut router: Router,
        movie_clips: Vec<MovieClip>,
    ) {
        let _m = MTX.lock().unwrap_or_else(|p_err| p_err.into_inner());

        let length = 100_usize;

        let mock_ctx = mock_movie_clip_usecases::order_by_create_date_movie_clips_context();
        mock_ctx
            .expect::<MockMovieClipRepository>()
            .withf(move |_, cmd| cmd.length == length)
            .times(1)
            .return_const(Ok(movie_clips.clone()));

        let request = Request::builder()
            .method(http::Method::GET)
            .uri(&format!(
                "/movie_clip/query?sort_type=create_date&length={length}"
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
    async fn test_order_by_create_date_later_movie_clips(
        mut router: Router,
        movie_clips: Vec<MovieClip>,
    ) {
        let _m = MTX.lock().unwrap_or_else(|p_err| p_err.into_inner());

        let reference = Faker.fake::<MovieClip>();
        let length = 100_usize;

        let mock_ctx = mock_movie_clip_usecases::order_by_create_date_later_movie_clips_context();
        mock_ctx
            .expect::<MockMovieClipRepository>()
            .withf({
                let reference = reference.clone();
                move |_, cmd| cmd.reference == reference && cmd.length == length
            })
            .times(1)
            .return_const(Ok(movie_clips.clone()));

        let query_info = QueryInfoRef::builder()
            .reference(Cow::Borrowed(&reference))
            .build();

        let request = Request::builder()
            .method(http::Method::GET)
            .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
            .uri(&format!(
                "/movie_clip/query?sort_type=create_date&length={length}"
            ))
            .body(Body::from(serde_json::to_vec(&query_info).unwrap()))
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
    async fn test_remove_movie_clip(mut router: Router) {
        let _m = MTX.lock().unwrap_or_else(|p_err| p_err.into_inner());

        let movie_clip_id = MovieClipId::generate();

        {
            let mock_ctx_ok = mock_movie_clip_usecases::remove_movie_clip_context();
            mock_ctx_ok
                .expect::<MockMovieClipRepository>()
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
                .expect::<MockMovieClipRepository>()
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

            assert!(matches!(err, AppCommonError::NoRecordError));
        }
    }
}
