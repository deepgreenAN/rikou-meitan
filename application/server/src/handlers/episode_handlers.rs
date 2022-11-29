use crate::commands::episode_commands;
use crate::usecases::episode_usecases;

use common::AppCommonError;
use domain::episode::{Episode, EpisodeId};
use domain::Date;

#[cfg(not(test))]
use infrastructure::EpisodePgDBRepository as EpisodeRepositoryImpl;

#[cfg(test)]
use infrastructure::MockEpisodeRepository as EpisodeRepositoryImpl;

use axum::{
    extract::rejection::{JsonRejection, PathRejection, QueryRejection},
    extract::{Json, Path, Query, State},
};

use serde::Deserialize;
use std::sync::Arc;

pub async fn save_episode_handler(
    State(episode_repo): State<Arc<EpisodeRepositoryImpl>>,
    episode_res: Result<Json<Episode>, JsonRejection>,
) -> Result<(), AppCommonError> {
    let episode = episode_res?.0;
    let cmd = episode_commands::SaveEpisodeCommand::new(episode);
    episode_usecases::save_episode_usecase(episode_repo, cmd).await?;
    Ok(())
}

pub async fn edit_episode_handler(
    State(episode_repo): State<Arc<EpisodeRepositoryImpl>>,
    episode_res: Result<Json<Episode>, JsonRejection>,
) -> Result<(), AppCommonError> {
    let episode = episode_res?.0;
    let cmd = episode_commands::EditEpisodeCommand::new(episode);
    episode_usecases::edit_episode_usecase(episode_repo, cmd).await?;
    Ok(())
}

pub async fn all_episodes_handler(
    State(episode_repo): State<Arc<EpisodeRepositoryImpl>>,
) -> Result<Json<Vec<Episode>>, AppCommonError> {
    let cmd = episode_commands::AllEpisodeCommand;
    let episodes = episode_usecases::all_episodes_usecase(episode_repo, cmd).await?;
    Ok(Json(episodes))
}

#[derive(Deserialize)]
pub struct DateRange {
    start: Date,
    end: Date,
}

pub async fn order_by_date_range_episodes_handler(
    date_range_res: Result<Query<DateRange>, QueryRejection>,
    State(episode_repo): State<Arc<EpisodeRepositoryImpl>>,
) -> Result<Json<Vec<Episode>>, AppCommonError> {
    let date_range = date_range_res?.0;
    let cmd =
        episode_commands::OrderByDateRangeEpisodeCommand::new(date_range.start, date_range.end);
    let episodes =
        episode_usecases::order_by_date_range_episodes_usecase(episode_repo, cmd).await?;
    Ok(Json(episodes))
}

pub async fn remove_by_id_episode_handler(
    id: Result<Path<EpisodeId>, PathRejection>,
    State(episode_repo): State<Arc<EpisodeRepositoryImpl>>,
) -> Result<(), AppCommonError> {
    let id = id?.0;
    let cmd = episode_commands::RemoveByIdEpisodeCommand::new(id);
    episode_usecases::remove_by_id_episode_usecase(episode_repo, cmd).await?;
    Ok(())
}

#[cfg(test)]
mod test {
    use super::EpisodeRepositoryImpl;
    use assert_matches::assert_matches;
    use axum::{
        body::Body,
        http::{self, Request, StatusCode},
        routing::{delete, get, put},
        Router,
    };
    use common::AppCommonError;
    use domain::episode::{Episode, EpisodeId};
    use domain::Date;
    use pretty_assertions::{assert_eq, assert_ne};
    use rstest::{fixture, rstest};
    use std::sync::Arc;
    use tower::{Service, ServiceExt};

    #[fixture]
    fn router() -> Router {
        let episode_repo = Arc::new(EpisodeRepositoryImpl::new());

        Router::new()
            .route(
                "/episode",
                put(super::save_episode_handler)
                    .patch(super::edit_episode_handler)
                    .get(super::all_episodes_handler),
            )
            .route(
                "/episode/order_date",
                get(super::order_by_date_range_episodes_handler),
            )
            .route("/episode/:id", delete(super::remove_by_id_episode_handler))
            .with_state(episode_repo)
    }

    #[fixture]
    async fn episodes_and_saved_repo(mut router: Router) -> (Vec<Episode>, Router) {
        let episodes = vec![
            Episode::new((2022, 11, 21), "Some Episode Content1".to_string()).unwrap(),
            Episode::new((2022, 11, 19), "Some Episode Content2".to_string()).unwrap(),
            Episode::new((2022, 11, 22), "Some Episode Content3".to_string()).unwrap(),
        ];

        for episode in episodes.iter() {
            let request = Request::builder()
                .method(http::Method::PUT)
                .uri("/episode")
                .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                .body(Body::from(serde_json::to_vec(episode).unwrap()))
                .unwrap();

            let response = router.ready().await.unwrap().call(request).await.unwrap();
            assert_eq!(response.status(), StatusCode::OK);

            let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
            assert!(body.is_empty());
        }
        (episodes, router)
    }

    #[rstest]
    #[tokio::test]
    async fn test_save_and_all_episodes(#[future] episodes_and_saved_repo: (Vec<Episode>, Router)) {
        let (mut episodes, router) = episodes_and_saved_repo.await;

        let request = Request::builder()
            .method(http::Method::GET)
            .uri("/episode")
            .body(Body::empty())
            .unwrap();

        let response = router.oneshot(request).await.unwrap();
        assert_eq!(response.status(), StatusCode::OK);

        let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
        let mut all_episodes: Vec<Episode> = serde_json::from_slice(&body).unwrap();

        all_episodes.sort_by_key(|episode| episode.id());
        episodes.sort_by_key(|episode| episode.id());

        assert_eq!(episodes, all_episodes);
    }

    #[rstest]
    #[tokio::test]
    async fn test_save_and_edit_and_all_episodes(
        #[future] episodes_and_saved_repo: (Vec<Episode>, Router),
    ) {
        let (mut episodes, mut router) = episodes_and_saved_repo.await;

        let mut edited_episode = episodes[1].clone();
        edited_episode
            .edit_content("New Episode Content".to_string())
            .unwrap();
        episodes[1] = edited_episode.clone();

        let edit_request = Request::builder()
            .method(http::Method::PATCH)
            .uri("/episode")
            .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
            .body(Body::from(serde_json::to_vec(&edited_episode).unwrap()))
            .unwrap();

        let edit_response = router
            .ready()
            .await
            .unwrap()
            .call(edit_request)
            .await
            .unwrap();
        assert_eq!(edit_response.status(), StatusCode::OK);

        let body = hyper::body::to_bytes(edit_response.into_body())
            .await
            .unwrap();
        assert!(body.is_empty());

        let all_request = Request::builder()
            .method(http::Method::GET)
            .uri("/episode")
            .body(Body::empty())
            .unwrap();

        let all_response = router.oneshot(all_request).await.unwrap();
        assert_eq!(all_response.status(), StatusCode::OK);

        let body = hyper::body::to_bytes(all_response.into_body())
            .await
            .unwrap();
        let mut all_episodes: Vec<Episode> = serde_json::from_slice(&body).unwrap();

        all_episodes.sort_by_key(|episode| episode.id());
        episodes.sort_by_key(|episode| episode.id());

        assert_eq!(episodes, all_episodes);
    }

    #[rstest]
    #[tokio::test]
    async fn test_save_and_order_by_date_range_episodes(
        #[future] episodes_and_saved_repo: (Vec<Episode>, Router),
    ) {
        let (mut episodes, router) = episodes_and_saved_repo.await;

        let request = Request::builder()
            .uri("/episode/order_date?start=2022-11-19&end=2022-11-22")
            .body(Body::empty())
            .unwrap();

        let response = router.oneshot(request).await.unwrap();
        assert_eq!(response.status(), StatusCode::OK);

        let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
        let order_episodes: Vec<Episode> = serde_json::from_slice(&body).unwrap();

        episodes.sort_by_key(|episode| episode.date());
        let episodes = episodes
            .into_iter()
            .filter(|episode| {
                Date::from_ymd(2022, 11, 19).unwrap() <= episode.date()
                    && episode.date() < Date::from_ymd(2022, 11, 22).unwrap()
            })
            .collect::<Vec<_>>();

        assert_eq!(episodes, order_episodes);
    }

    #[rstest]
    #[tokio::test]
    async fn test_save_and_remove_by_id_episode(
        #[future] episodes_and_saved_repo: (Vec<Episode>, Router),
    ) {
        let (mut episodes, mut router) = episodes_and_saved_repo.await;

        let removed_episode = episodes.remove(1);

        let remove_request = Request::builder()
            .method(http::Method::DELETE)
            .uri(format!("/episode/{}", removed_episode.id()))
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
            .uri("/episode")
            .body(Body::empty())
            .unwrap();

        let all_response = router.oneshot(all_request).await.unwrap();
        assert_eq!(all_response.status(), StatusCode::OK);

        let all_body = hyper::body::to_bytes(all_response.into_body())
            .await
            .unwrap();
        let mut all_episodes: Vec<Episode> = serde_json::from_slice(&all_body).unwrap();

        all_episodes.sort_by_key(|episode| episode.id());
        episodes.sort_by_key(|episode| episode.id());

        assert_eq!(episodes, all_episodes);
    }

    #[rstest]
    #[tokio::test]
    async fn test_edit_episode_not_exists(router: Router) {
        let episode = Episode::new((2022, 11, 21), "Some Episode Content1".to_string()).unwrap();
        let request = Request::builder()
            .method(http::Method::PATCH)
            .uri("/episode")
            .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
            .body(Body::from(serde_json::to_vec(&episode).unwrap()))
            .unwrap();

        let response = router.oneshot(request).await.unwrap();
        assert_ne!(response.status(), StatusCode::OK);

        let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
        let err: AppCommonError = serde_json::from_slice(&body).unwrap();

        assert_matches!(err, AppCommonError::RemovedRecordError);
    }

    #[rstest]
    #[tokio::test]
    async fn test_remove_by_id_not_exists(router: Router) {
        let id = EpisodeId::generate();
        let request = Request::builder()
            .method(http::Method::DELETE)
            .uri(format!("/episode/{}", id))
            .body(Body::empty())
            .unwrap();

        let response = router.oneshot(request).await.unwrap();
        assert_ne!(response.status(), StatusCode::OK);

        let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
        let err: AppCommonError = serde_json::from_slice(&body).unwrap();

        assert_matches!(err, AppCommonError::RemovedRecordError);
    }
}
