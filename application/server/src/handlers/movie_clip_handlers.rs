use crate::commands::movie_clip_commands;
use crate::usecases::movie_clip_usecases;

use common::AppCommonError;
use domain::movie_clip::{MovieClip, MovieClipId};
use domain::Date;

#[cfg(not(test))]
use infrastructure::MovieClipPgDBRepository as MovieClipRepositoryImpl;

#[cfg(test)] // モック
use infrastructure::MockMovieClipRepository as MovieClipRepositoryImpl;

use axum::{
    extract::rejection::{JsonRejection, PathRejection, QueryRejection},
    extract::{Json, Path, Query, State},
};

use serde::Deserialize;
use std::sync::Arc;

pub async fn save_movie_clip_handler(
    State(movie_clip_repo): State<Arc<MovieClipRepositoryImpl>>,
    movie_clip_res: Result<Json<MovieClip>, JsonRejection>,
) -> Result<(), AppCommonError> {
    let movie_clip = movie_clip_res?.0;

    let cmd = movie_clip_commands::SaveMovieClipCommand::new(movie_clip);
    movie_clip_usecases::save_movie_clip_usecase(movie_clip_repo, cmd).await?;
    Ok(())
}

pub async fn edit_movie_clip_handler(
    State(movie_clip_repo): State<Arc<MovieClipRepositoryImpl>>,
    movie_clip_res: Result<Json<MovieClip>, JsonRejection>,
) -> Result<(), AppCommonError> {
    let movie_clip = movie_clip_res?.0;

    let cmd = movie_clip_commands::EditMovieClipCommand::new(movie_clip);
    movie_clip_usecases::edit_movie_clip_usecase(movie_clip_repo, cmd).await?;

    Ok(())
}

pub async fn all_movie_clips_handler(
    State(movie_clip_repo): State<Arc<MovieClipRepositoryImpl>>,
) -> Result<Json<Vec<MovieClip>>, AppCommonError> {
    let cmd = movie_clip_commands::AllMovieClipCommand;
    let movie_clips = movie_clip_usecases::all_movie_clips_usecase(movie_clip_repo, cmd).await?;
    Ok(Json(movie_clips))
}

#[derive(Deserialize)]
pub struct Limit {
    length: usize,
}

pub async fn order_by_like_limit_movie_clips_handler(
    limit_res: Result<Query<Limit>, QueryRejection>,
    State(movie_clip_repo): State<Arc<MovieClipRepositoryImpl>>,
) -> Result<Json<Vec<MovieClip>>, AppCommonError> {
    let limit = limit_res.map_err(Into::<AppCommonError>::into)?.0;
    let cmd = movie_clip_commands::OrderByLikeLimitMovieClipCommand::new(limit.length);
    let movie_clips =
        movie_clip_usecases::order_by_like_limit_movie_clips_usecase(movie_clip_repo, cmd).await?;
    Ok(Json(movie_clips))
}

#[derive(Deserialize)]
pub struct DateRange {
    start: Date,
    end: Date,
}

pub async fn order_by_create_date_range_movie_clips_handler(
    date_range_res: Result<Query<DateRange>, QueryRejection>,
    State(movie_clip_repo): State<Arc<MovieClipRepositoryImpl>>,
) -> Result<Json<Vec<MovieClip>>, AppCommonError> {
    let date_range = date_range_res?.0;
    let cmd = movie_clip_commands::OrderByCreateDateRangeMovieClipCommand::new(
        date_range.start,
        date_range.end,
    );
    let movie_clips =
        movie_clip_usecases::order_by_create_date_range_movie_clips_usecase(movie_clip_repo, cmd)
            .await?;
    Ok(Json(movie_clips))
}

pub async fn remove_by_id_movie_clip_handler(
    id: Result<Path<MovieClipId>, PathRejection>,
    State(movie_clip_repo): State<Arc<MovieClipRepositoryImpl>>,
) -> Result<(), AppCommonError> {
    let id = id?.0;
    let cmd = movie_clip_commands::RemoveByIdMovieClipCommand::new(id);
    movie_clip_usecases::remove_by_id_movie_clip_usecase(movie_clip_repo, cmd).await?;
    Ok(())
}

#[cfg(test)]
mod test {
    use super::MovieClipRepositoryImpl;
    use assert_matches::assert_matches;
    use axum::{
        body::Body,
        http::{self, Request, StatusCode},
        routing::{delete, get, put},
        Router,
    };
    use common::AppCommonError;
    use domain::movie_clip::{MovieClip, MovieClipId};
    use domain::Date;
    use pretty_assertions::{assert_eq, assert_ne};
    use rstest::{fixture, rstest};
    use std::sync::Arc;
    use tower::{Service, ServiceExt};

    #[fixture]
    fn router() -> Router {
        let movie_clip_repo = Arc::new(MovieClipRepositoryImpl::new());

        Router::new()
            .route(
                "/movie_clip",
                put(super::save_movie_clip_handler)
                    .patch(super::edit_movie_clip_handler)
                    .get(super::all_movie_clips_handler),
            )
            .route(
                "/movie_clip/order_like",
                get(super::order_by_like_limit_movie_clips_handler),
            )
            .route(
                "/movie_clip/order_create_date",
                get(super::order_by_create_date_range_movie_clips_handler),
            )
            .route(
                "/movie_clip/:id",
                delete(super::remove_by_id_movie_clip_handler),
            )
            .with_state(Arc::clone(&movie_clip_repo))
    }

    #[fixture]
    async fn movie_clips_and_router(mut router: Router) -> (Vec<MovieClip>, Router) {
        let mut movie_clips = vec![
            MovieClip::new(
                "MovieClip 1".to_string(),
                "https://www.youtube.com/watch?v=B7OPlsdBuVc".to_string(),
                100,
                200,
                (2022, 11, 21),
            )
            .unwrap(),
            MovieClip::new(
                "MovieClip 2".to_string(),
                "https://www.youtube.com/watch?v=NHpILI4NpCI".to_string(),
                200,
                300,
                (2022, 11, 22),
            )
            .unwrap(),
            MovieClip::new(
                "MovieClip 3".to_string(),
                "https://www.youtube.com/watch?v=6LAn0lbMpZ8".to_string(),
                400,
                500,
                (2022, 11, 19),
            )
            .unwrap(),
        ];

        for (i, movie_clip) in movie_clips.iter_mut().enumerate() {
            for _ in 0..i {
                movie_clip.like_increment();
            }
        }

        for movie_clip in movie_clips.iter() {
            let request = Request::builder()
                .method(http::Method::PUT)
                .uri("/movie_clip")
                .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                .body(Body::from(serde_json::to_vec(movie_clip).unwrap()))
                .unwrap();

            let response = router.ready().await.unwrap().call(request).await.unwrap();
            assert_eq!(response.status(), StatusCode::OK);

            let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
            assert!(body.is_empty());
        }

        (movie_clips, router)
    }

    #[rstest]
    #[tokio::test]
    async fn test_save_and_all_movie_clips(
        #[future] movie_clips_and_router: (Vec<MovieClip>, Router),
    ) {
        let (mut movie_clips, mut router) = movie_clips_and_router.await;

        let request = Request::builder()
            .method(http::Method::GET)
            .uri("/movie_clip")
            .body(Body::empty())
            .unwrap();

        let response = router.ready().await.unwrap().call(request).await.unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
        let mut all_movie_clips: Vec<MovieClip> = serde_json::from_slice(&body).unwrap();

        all_movie_clips.sort_by_key(|movie_clip| movie_clip.id());
        movie_clips.sort_by_key(|movie_clip| movie_clip.id());

        assert_eq!(all_movie_clips, movie_clips);
    }

    #[rstest]
    #[tokio::test]
    async fn test_save_and_edit_and_all_movie_clips(
        #[future] movie_clips_and_router: (Vec<MovieClip>, Router),
    ) {
        let (mut movie_clips, mut router) = movie_clips_and_router.await;

        let mut edited_clip = movie_clips[1].clone(); // 三番目の値を変更
        edited_clip.edit_title("New MovieClip".to_string()).unwrap();
        movie_clips[1] = edited_clip.clone();

        let edit_request = Request::builder()
            .method(http::Method::PATCH)
            .uri("/movie_clip")
            .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
            .body(Body::from(serde_json::to_vec(&edited_clip).unwrap()))
            .unwrap();

        let edit_response = router
            .ready()
            .await
            .unwrap()
            .call(edit_request)
            .await
            .unwrap();
        assert_eq!(edit_response.status(), StatusCode::OK);

        let edit_body = hyper::body::to_bytes(edit_response.into_body())
            .await
            .unwrap();
        assert!(edit_body.is_empty());

        let all_request = Request::builder()
            .method(http::Method::GET)
            .uri("/movie_clip")
            .body(Body::empty())
            .unwrap();

        let all_response = router.oneshot(all_request).await.unwrap();

        assert_eq!(all_response.status(), StatusCode::OK);

        let all_body = hyper::body::to_bytes(all_response.into_body())
            .await
            .unwrap();
        let mut all_movie_clips: Vec<MovieClip> = serde_json::from_slice(&all_body).unwrap();

        all_movie_clips.sort_by_key(|movie_clip| movie_clip.id());
        movie_clips.sort_by_key(|movie_clip| movie_clip.id());

        assert_eq!(all_movie_clips, movie_clips);
    }

    #[rstest]
    #[tokio::test]
    async fn test_save_and_order_by_like_limit_movie_clips(
        #[future] movie_clips_and_router: (Vec<MovieClip>, Router),
    ) {
        let (mut movie_clips, router) = movie_clips_and_router.await;

        let request = Request::builder()
            .method(http::Method::GET)
            .uri("/movie_clip/order_like?length=2")
            .body(Body::empty())
            .unwrap();

        let response = router.oneshot(request).await.unwrap();
        assert_eq!(response.status(), StatusCode::OK);

        let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
        let ordered_movie_clips: Vec<MovieClip> = serde_json::from_slice(&body).unwrap();

        movie_clips.sort_by_key(|movie_clip| u32::MAX - movie_clip.like());
        let movie_clips = movie_clips.into_iter().take(2).collect::<Vec<_>>();

        assert_eq!(ordered_movie_clips, movie_clips);
    }

    #[rstest]
    #[tokio::test]
    async fn test_save_and_order_by_create_date_range_movie_clips(
        #[future] movie_clips_and_router: (Vec<MovieClip>, Router),
    ) {
        let (mut movie_clips, router) = movie_clips_and_router.await;

        let request = Request::builder()
            .method(http::Method::GET)
            .uri("/movie_clip/order_create_date?start=2022-11-19&end=2022-11-22")
            .body(Body::empty())
            .unwrap();

        let response = router.oneshot(request).await.unwrap();
        assert_eq!(response.status(), StatusCode::OK);

        let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
        let ordered_movie_clips: Vec<MovieClip> = serde_json::from_slice(&body).unwrap();

        movie_clips.sort_by_key(|movie_clip| movie_clip.create_date());
        let movie_clips = movie_clips
            .into_iter()
            .filter(|movie_clip| {
                Date::from_ymd(2022, 11, 19).unwrap() <= movie_clip.create_date()
                    && movie_clip.create_date() < Date::from_ymd(2022, 11, 22).unwrap()
            })
            .collect::<Vec<_>>();

        assert_eq!(ordered_movie_clips, movie_clips);
    }

    #[rstest]
    #[tokio::test]
    async fn test_save_and_remove_by_id_movie_clips(
        #[future] movie_clips_and_router: (Vec<MovieClip>, Router),
    ) {
        let (mut movie_clips, mut router) = movie_clips_and_router.await;

        let removed_movie_clip = movie_clips.remove(1);
        let movie_clip_id = removed_movie_clip.id();

        let remove_request = Request::builder()
            .method(http::Method::DELETE)
            .uri(format!("/movie_clip/{}", movie_clip_id))
            .body(Body::empty())
            .unwrap();

        let remove_response = router
            .ready()
            .await
            .unwrap()
            .call(remove_request)
            .await
            .unwrap();
        assert_eq!(remove_response.status(), StatusCode::OK);

        let remove_body = hyper::body::to_bytes(remove_response.into_body())
            .await
            .unwrap();
        assert!(remove_body.is_empty());

        let all_request = Request::builder()
            .method(http::Method::GET)
            .uri("/movie_clip")
            .body(Body::empty())
            .unwrap();

        let all_response = router.oneshot(all_request).await.unwrap();
        assert_eq!(all_response.status(), StatusCode::OK);

        let all_body = hyper::body::to_bytes(all_response.into_body())
            .await
            .unwrap();
        let mut all_movie_clips: Vec<MovieClip> = serde_json::from_slice(&all_body).unwrap();

        all_movie_clips.sort_by_key(|movie_clip| movie_clip.id());
        movie_clips.sort_by_key(|movie_clip| movie_clip.id());

        assert_eq!(all_movie_clips, movie_clips);
    }

    #[rstest]
    #[tokio::test]
    async fn test_edit_movie_clip_not_exists(router: Router) {
        let movie_clip = MovieClip::new(
            "MovieClip 1".to_string(),
            "https://www.youtube.com/watch?v=B7OPlsdBuVc".to_string(),
            100,
            200,
            (2022, 11, 21),
        )
        .unwrap();

        let request = Request::builder()
            .method(http::Method::PATCH)
            .uri("/movie_clip")
            .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
            .body(Body::from(serde_json::to_vec(&movie_clip).unwrap()))
            .unwrap();

        let response = router.oneshot(request).await.unwrap();

        assert_ne!(response.status(), StatusCode::OK);

        let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
        let err: AppCommonError = serde_json::from_slice(&body).unwrap();

        assert_matches!(err, AppCommonError::RemovedRecordError);
    }

    #[rstest]
    #[tokio::test]
    async fn test_remove_by_id_movie_clip_not_exists(router: Router) {
        let id = MovieClipId::generate();

        let request = Request::builder()
            .method(http::Method::DELETE)
            .uri(format!("/movie_clip/{}", id))
            .body(Body::empty())
            .unwrap();

        let response = router.oneshot(request).await.unwrap();

        assert_ne!(response.status(), StatusCode::OK);

        let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
        let err: AppCommonError = serde_json::from_slice(&body).unwrap();

        assert_matches!(err, AppCommonError::RemovedRecordError);
    }
}
