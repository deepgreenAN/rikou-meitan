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
use std::str::FromStr;

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

#[derive(Deserialize, strum_macros::EnumString)]
#[serde(try_from = "String")]
#[strum(serialize_all = "snake_case")]
pub enum SortType {
    Date,
}

impl TryFrom<String> for SortType {
    type Error = <SortType as FromStr>::Err;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        value.parse()
    }
}

#[allow(dead_code)]
#[derive(Deserialize)]
pub struct EpisodeQuery {
    /// クエリのstart日時
    start: Option<Date>,
    /// クエリのend日時
    end: Option<Date>,
    /// クエリのソートタイプ
    sort_type: SortType,
}

pub async fn get_episodes_with_query(
    query_res: Result<Query<EpisodeQuery>, QueryRejection>,
    State(app_state): State<AppState>,
) -> Result<Json<Vec<Episode>>, AppCommonError> {
    let query = query_res?.0;

    match (query.sort_type, query.start, query.end) {
        (SortType::Date, Some(start), Some(end)) => {
            let cmd = episode_commands::OrderByDateRangeEpisodeCommand::new(start, end);
            let episodes =
                episode_usecases::order_by_date_range_episodes(app_state.episode_repo, cmd).await?;
            Ok(Json(episodes))
        }
        _ => Err(AppCommonError::QueryStringRejectionError(
            "Invalid query parameter combination.".to_string(),
        )),
    }
}

pub async fn remove_episode(
    id: Result<Path<EpisodeId>, PathRejection>,
    State(app_state): State<AppState>,
) -> Result<(), AppCommonError> {
    let id = id?.0;
    let cmd = episode_commands::RemoveEpisodeCommand::new(id);
    episode_usecases::remove_episode(app_state.episode_repo, cmd).await?;
    Ok(())
}

#[cfg(test)]
mod test {
    use super::AppState;
    use crate::usecases::mock_episode_usecases;
    use common::AppCommonError;
    use domain::episode::{Episode, EpisodeId};
    use domain::Date;
    use infrastructure::episode_repository_impl::MockEpisodeRepository;

    use axum::{
        body::Body,
        http::{self, Request, StatusCode},
        routing::{delete, get, put},
        Router,
    };

    use fake::{Fake, Faker};
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
            .route("/episode/query", get(super::get_episodes_with_query))
            .route("/episode/:id", delete(super::remove_episode))
            .with_state(app_state)
    }

    #[fixture]
    fn episodes() -> Vec<Episode> {
        (0..100)
            .map(|_| Faker.fake::<Episode>())
            .collect::<Vec<_>>()
    }

    #[allow(clippy::await_holding_lock)]
    #[rstest]
    #[tokio::test]
    async fn test_save_episode(mut router: Router) {
        let _m = MTX.lock().unwrap_or_else(|p_err| p_err.into_inner());

        let episode = Faker.fake::<Episode>();

        {
            let mock_ctx_ok = mock_episode_usecases::save_episode_context();
            mock_ctx_ok
                .expect::<MockEpisodeRepository>()
                .withf({
                    let episode = episode.clone();
                    move |_, cmd| cmd.episode == episode
                })
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
            let mock_ctx_err = mock_episode_usecases::save_episode_context();
            mock_ctx_err
                .expect::<MockEpisodeRepository>()
                .withf({
                    let episode = episode.clone();
                    move |_, cmd| cmd.episode == episode
                })
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

            assert!(matches!(err, AppCommonError::ConflictError));
        }
    }

    #[allow(clippy::await_holding_lock)]
    #[rstest]
    #[tokio::test]
    async fn test_edit_episode(mut router: Router) {
        let _m = MTX.lock().unwrap_or_else(|p_err| p_err.into_inner());

        let episode = Faker.fake::<Episode>();

        {
            let mock_ctx_ok = mock_episode_usecases::edit_episode_context();
            mock_ctx_ok
                .expect::<MockEpisodeRepository>()
                .withf({
                    let episode = episode.clone();
                    move |_, cmd| cmd.episode == episode
                })
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
            let mock_ctx_err = mock_episode_usecases::edit_episode_context();
            mock_ctx_err
                .expect::<MockEpisodeRepository>()
                .withf({
                    let episode = episode.clone();
                    move |_, cmd| cmd.episode == episode
                })
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

            assert!(matches!(err, AppCommonError::NoRecordError));
        }
    }

    #[allow(clippy::await_holding_lock)]
    #[rstest]
    #[tokio::test]
    async fn test_all_episodes(mut router: Router, episodes: Vec<Episode>) {
        let _m = MTX.lock().unwrap_or_else(|p_err| p_err.into_inner());

        let mock_ctx = mock_episode_usecases::all_episodes_context();
        mock_ctx
            .expect::<MockEpisodeRepository>()
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
    async fn test_order_by_date_episodes(mut router: Router, episodes: Vec<Episode>) {
        let _m = MTX.lock().unwrap_or_else(|p_err| p_err.into_inner());

        let start = Faker.fake::<Date>();
        let end = Faker.fake::<Date>();

        let mock_ctx = mock_episode_usecases::order_by_date_range_episodes_context();
        mock_ctx
            .expect::<MockEpisodeRepository>()
            .withf(move |_, cmd| start == cmd.start && end == cmd.end)
            .return_const(Ok(episodes.clone()));

        let request = Request::builder()
            .method(http::Method::GET)
            .uri(&format!(
                "/episode/query?start={start}&end={end}&sort_type=date"
            ))
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
            let mock_ctx_ok = mock_episode_usecases::remove_episode_context();
            mock_ctx_ok
                .expect::<MockEpisodeRepository>()
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
            let mock_ctx_err = mock_episode_usecases::remove_episode_context();
            mock_ctx_err
                .expect::<MockEpisodeRepository>()
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

            assert!(matches!(err, AppCommonError::NoRecordError));
        }
    }
}
