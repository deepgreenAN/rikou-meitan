use crate::commands::episode_commands;
use common::AppCommonError;
use domain::episode::{Episode, EpisodeId};
use domain::Date;

/// usecaseについてのダブル
#[cfg(not(test))]
use crate::usecases::episode_usecases;

#[cfg(test)]
use crate::usecases::mock_episode_usecases as episode_usecases;

/// AppStateについてのダブル
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

pub async fn save_episode(
    State(app_state): State<AppState>,
    episode_res: Result<Json<Episode>, JsonRejection>,
) -> Result<(), AppCommonError> {
    let episode = episode_res?.0;
    let cmd = episode_commands::SaveEpisodeCommand::new(episode);
    episode_usecases::save_episode(app_state.episode_repo, cmd).await?;
    Ok(())
}

pub async fn edit_episode(
    State(app_state): State<AppState>,
    episode_res: Result<Json<Episode>, JsonRejection>,
) -> Result<(), AppCommonError> {
    let episode = episode_res?.0;
    let cmd = episode_commands::EditEpisodeCommand::new(episode);
    episode_usecases::edit_episode(app_state.episode_repo, cmd).await?;
    Ok(())
}

pub async fn all_episodes(
    State(app_state): State<AppState>,
) -> Result<Json<Vec<Episode>>, AppCommonError> {
    let cmd = episode_commands::AllEpisodeCommand;
    let episodes = episode_usecases::all_episodes(app_state.episode_repo, cmd).await?;
    Ok(Json(episodes))
}

#[derive(Deserialize)]
pub struct DateRange {
    start: Date,
    end: Date,
}

pub async fn order_by_date_range_episodes(
    date_range_res: Result<Query<DateRange>, QueryRejection>,
    State(app_state): State<AppState>,
) -> Result<Json<Vec<Episode>>, AppCommonError> {
    let date_range = date_range_res?.0;
    let cmd =
        episode_commands::OrderByDateRangeEpisodeCommand::new(date_range.start, date_range.end);
    let episodes =
        episode_usecases::order_by_date_range_episodes(app_state.episode_repo, cmd).await?;
    Ok(Json(episodes))
}

pub async fn remove_by_id_episode(
    id: Result<Path<EpisodeId>, PathRejection>,
    State(app_state): State<AppState>,
) -> Result<(), AppCommonError> {
    let id = id?.0;
    let cmd = episode_commands::RemoveByIdEpisodeCommand::new(id);
    episode_usecases::remove_by_id_episode(app_state.episode_repo, cmd).await?;
    Ok(())
}

#[cfg(test)]
mod test {
    use super::AppState;
    use crate::usecases::mock_episode_usecases;
    use common::AppCommonError;
    use domain::episode::{Episode, EpisodeId};
    use domain::Date;
    use infrastructure::episode_repository_impl::MockEpisodeRepositoryImpl;

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
        let app_state = AppState::new();

        Router::new()
            .route(
                "/episode",
                put(super::save_episode)
                    .patch(super::edit_episode)
                    .get(super::all_episodes),
            )
            .route(
                "/episode/order_date",
                get(super::order_by_date_range_episodes),
            )
            .route("/episode/:id", delete(super::remove_by_id_episode))
            .with_state(app_state)
    }

    #[allow(clippy::await_holding_lock)]
    #[rstest]
    #[tokio::test]
    async fn test_save_episode(mut router: Router) {
        let _m = MTX.lock().unwrap_or_else(|p_err| p_err.into_inner());

        let episode = Episode::new((2022, 12, 4), "Some Contents".to_string()).unwrap();

        {
            let cloned_episode = episode.clone();
            let mock_ctx_ok = mock_episode_usecases::save_episode_context();
            mock_ctx_ok
                .expect::<MockEpisodeRepositoryImpl>()
                .withf(move |_, cmd| cmd.episode == cloned_episode)
                .return_const(Ok(()));

            let request = Request::builder()
                .method(http::Method::PUT)
                .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                .uri("/episode")
                .body(Body::from(serde_json::to_vec(&episode).unwrap()))
                .unwrap();

            let response = router.ready().await.unwrap().call(request).await.unwrap();
            assert_eq!(response.status(), StatusCode::OK);
            let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
            assert!(body.is_empty());
        }
        {
            let cloned_episode = episode.clone();
            let mock_ctx_err = mock_episode_usecases::save_episode_context();
            mock_ctx_err
                .expect::<MockEpisodeRepositoryImpl>()
                .withf(move |_, cmd| cmd.episode == cloned_episode)
                .return_const(Err(AppCommonError::ConflictError));

            let request = Request::builder()
                .method(http::Method::PUT)
                .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                .uri("/episode")
                .body(Body::from(serde_json::to_vec(&episode).unwrap()))
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
    async fn test_edit_episode(mut router: Router) {
        let _m = MTX.lock().unwrap_or_else(|p_err| p_err.into_inner());

        let episode = Episode::new((2022, 12, 4), "Some Contents".to_string()).unwrap();

        {
            let cloned_episode = episode.clone();
            let mock_ctx_ok = mock_episode_usecases::edit_episode_context();
            mock_ctx_ok
                .expect::<MockEpisodeRepositoryImpl>()
                .withf(move |_, cmd| cmd.episode == cloned_episode)
                .return_const(Ok(()));

            let request = Request::builder()
                .method(http::Method::PATCH)
                .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                .uri("/episode")
                .body(Body::from(serde_json::to_vec(&episode).unwrap()))
                .unwrap();

            let response = router.ready().await.unwrap().call(request).await.unwrap();
            assert_eq!(response.status(), StatusCode::OK);
            let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
            assert!(body.is_empty());
        }
        {
            let cloned_episode = episode.clone();
            let mock_ctx_err = mock_episode_usecases::edit_episode_context();
            mock_ctx_err
                .expect::<MockEpisodeRepositoryImpl>()
                .withf(move |_, cmd| cmd.episode == cloned_episode)
                .return_const(Err(AppCommonError::NoRecordError));

            let request = Request::builder()
                .method(http::Method::PATCH)
                .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                .uri("/episode")
                .body(Body::from(serde_json::to_vec(&episode).unwrap()))
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
    async fn test_all_episodes(mut router: Router) {
        let _m = MTX.lock().unwrap_or_else(|p_err| p_err.into_inner());

        let episodes = vec![Episode::new((2022, 12, 4), "Some Contents".to_string()).unwrap()];

        let mock_ctx = mock_episode_usecases::all_episodes_context();
        mock_ctx
            .expect::<MockEpisodeRepositoryImpl>()
            .return_const(Ok(episodes.clone()));

        let request = Request::builder()
            .method(http::Method::GET)
            .uri("/episode")
            .body(Body::empty())
            .unwrap();

        let response = router.ready().await.unwrap().call(request).await.unwrap();
        assert_eq!(response.status(), StatusCode::OK);

        let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
        let res_vec: Vec<Episode> = serde_json::from_slice(&body).unwrap();

        assert_eq!(res_vec, episodes);
    }

    #[allow(clippy::await_holding_lock)]
    #[rstest]
    #[tokio::test]
    async fn test_order_by_date_episodes(mut router: Router) {
        let _m = MTX.lock().unwrap_or_else(|p_err| p_err.into_inner());

        let episodes = vec![Episode::new((2022, 12, 4), "Some Contents".to_string()).unwrap()];

        let start = Date::from_ymd(2022, 12, 4).unwrap();
        let end = Date::from_ymd(2022, 12, 6).unwrap();

        let mock_ctx = mock_episode_usecases::order_by_date_range_episodes_context();
        mock_ctx
            .expect::<MockEpisodeRepositoryImpl>()
            .withf(move |_, cmd| start == cmd.start && end == cmd.end)
            .return_const(Ok(episodes.clone()));

        let request = Request::builder()
            .method(http::Method::GET)
            .uri(&format!("/episode/order_date?start={start}&end={end}"))
            .body(Body::empty())
            .unwrap();

        let response = router.ready().await.unwrap().call(request).await.unwrap();
        assert_eq!(response.status(), StatusCode::OK);

        let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
        let res_vec: Vec<Episode> = serde_json::from_slice(&body).unwrap();

        assert_eq!(res_vec, episodes);
    }

    #[allow(clippy::await_holding_lock)]
    #[rstest]
    #[tokio::test]
    async fn test_remove_by_id_episode(mut router: Router) {
        let _m = MTX.lock().unwrap_or_else(|p_err| p_err.into_inner());

        let episode_id = EpisodeId::generate();

        {
            let mock_ctx_ok = mock_episode_usecases::remove_by_id_episode_context();
            mock_ctx_ok
                .expect::<MockEpisodeRepositoryImpl>()
                .withf(move |_, cmd| cmd.id == episode_id)
                .return_const(Ok(()));

            let request = Request::builder()
                .method(http::Method::DELETE)
                .uri(&format!("/episode/{episode_id}"))
                .body(Body::empty())
                .unwrap();

            let response = router.ready().await.unwrap().call(request).await.unwrap();
            assert_eq!(response.status(), StatusCode::OK);
            let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
            assert!(body.is_empty());
        }
        {
            let mock_ctx_err = mock_episode_usecases::remove_by_id_episode_context();
            mock_ctx_err
                .expect::<MockEpisodeRepositoryImpl>()
                .withf(move |_, cmd| cmd.id == episode_id)
                .return_const(Err(AppCommonError::NoRecordError));

            let request = Request::builder()
                .method(http::Method::DELETE)
                .uri(&format!("/episode/{episode_id}"))
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
