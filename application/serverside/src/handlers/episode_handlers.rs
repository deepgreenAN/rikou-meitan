use crate::commands::episode_commands;
use common::AppCommonError;
use domain::episode::{Episode, EpisodeId};
use domain::Date;

/// usecaseについてのダブル
#[cfg(not(test))]
use crate::usecases::episode_usecases;

#[cfg(test)]
use crate::usecases::mock_episode_usecases as episode_usecases;

// EpisodeRepositoryについてのタプル
#[cfg(all(not(test), feature = "inmemory"))]
use infrastructure::episode_repository_impl::InMemoryEpisodeRepository as EpisodeRepositoryImpl;

#[cfg(all(not(test), not(feature = "inmemory")))]
use infrastructure::episode_repository_impl::EpisodePgDBRepository as EpisodeRepositoryImpl;

#[cfg(test)]
use infrastructure::episode_repository_impl::MockEpisodeRepository as EpisodeRepositoryImpl;

use axum::{
    extract::rejection::{JsonRejection, PathRejection, QueryRejection},
    extract::{Json, Path, Query, State},
};

use tracing_attributes::instrument;

use serde::Deserialize;
use std::str::FromStr;
use std::sync::Arc;

#[instrument(skip(episode_repo), err(Display))]
pub async fn save_episode(
    State(episode_repo): State<Arc<EpisodeRepositoryImpl>>,
    episode_res: Result<Json<Episode>, JsonRejection>,
) -> Result<(), AppCommonError> {
    let episode = episode_res?.0;
    let cmd = episode_commands::SaveEpisodeCommand::new(episode);
    episode_usecases::save_episode(episode_repo, cmd).await?;
    Ok(())
}

#[instrument(skip(episode_repo), err(Display))]
pub async fn edit_episode(
    State(episode_repo): State<Arc<EpisodeRepositoryImpl>>,
    episode_res: Result<Json<Episode>, JsonRejection>,
) -> Result<(), AppCommonError> {
    let episode = episode_res?.0;
    let cmd = episode_commands::EditEpisodeCommand::new(episode);
    episode_usecases::edit_episode(episode_repo, cmd).await?;
    Ok(())
}

#[instrument(skip(episode_repo), err(Display))]
pub async fn all_episodes(
    State(episode_repo): State<Arc<EpisodeRepositoryImpl>>,
) -> Result<Json<Vec<Episode>>, AppCommonError> {
    let cmd = episode_commands::AllEpisodeCommand;
    let episodes = episode_usecases::all_episodes(episode_repo, cmd).await?;
    Ok(Json(episodes))
}

#[derive(Deserialize, strum_macros::EnumString, Debug)]
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
#[derive(Deserialize, Debug)]
pub struct EpisodeQuery {
    /// クエリのstart日時
    start: Option<Date>,
    /// クエリのend日時
    end: Option<Date>,
    /// クエリのソートタイプ
    sort_type: SortType,
}

#[instrument(skip(episode_repo), err(Display))]
pub async fn get_episodes_with_query(
    query_res: Result<Query<EpisodeQuery>, QueryRejection>,
    State(episode_repo): State<Arc<EpisodeRepositoryImpl>>,
) -> Result<Json<Vec<Episode>>, AppCommonError> {
    let query = query_res?.0;

    match (query.sort_type, query.start, query.end) {
        (SortType::Date, Some(start), Some(end)) => {
            let cmd = episode_commands::OrderByDateRangeEpisodeCommand::new(start, end);
            let episodes =
                episode_usecases::order_by_date_range_episodes(episode_repo, cmd).await?;
            Ok(Json(episodes))
        }
        _ => Err(AppCommonError::QueryStringRejectionError(
            "Invalid query parameter combination.".to_string(),
        )),
    }
}

#[instrument(skip(episode_repo), err(Display))]
pub async fn remove_episode(
    id: Result<Path<EpisodeId>, PathRejection>,
    State(episode_repo): State<Arc<EpisodeRepositoryImpl>>,
) -> Result<(), AppCommonError> {
    let id = id?.0;
    let cmd = episode_commands::RemoveEpisodeCommand::new(id);
    episode_usecases::remove_episode(episode_repo, cmd).await?;
    Ok(())
}

#[cfg(test)]
mod test {
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

    use std::sync::Arc;

    use fake::{Fake, Faker};
    use pretty_assertions::{assert_eq, assert_ne};
    use rstest::{fixture, rstest};
    use serial_test::serial;
    use tower::{Service, ServiceExt};

    #[fixture]
    fn router() -> Router {
        let episode_repo = Arc::new(MockEpisodeRepository::new());

        Router::new()
            .route(
                "/episode",
                put(super::save_episode)
                    .patch(super::edit_episode)
                    .get(super::all_episodes),
            )
            .route("/episode/query", get(super::get_episodes_with_query))
            .route("/episode/:id", delete(super::remove_episode))
            .with_state(episode_repo)
    }

    #[fixture]
    fn episodes() -> Vec<Episode> {
        (0..100)
            .map(|_| Faker.fake::<Episode>())
            .collect::<Vec<_>>()
    }

    #[rstest]
    #[tokio::test]
    #[serial("mock_episode")]
    async fn test_save_episode(mut router: Router) {
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

    #[rstest]
    #[tokio::test]
    #[serial("mock_episode")]
    async fn test_edit_episode(mut router: Router) {
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

    #[rstest]
    #[tokio::test]
    #[serial("mock_episode")]
    async fn test_all_episodes(mut router: Router, episodes: Vec<Episode>) {
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

    #[rstest]
    #[tokio::test]
    #[serial("mock_episode")]
    async fn test_order_by_date_episodes(mut router: Router, episodes: Vec<Episode>) {
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

    #[rstest]
    #[tokio::test]
    #[serial("mock_episode")]
    async fn test_remove_by_id_episode(mut router: Router) {
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
