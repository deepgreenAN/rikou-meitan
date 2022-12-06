#![cfg(feature = "server")]

use config::CONFIG;

mod movie_clip_handlers {
    use common::AppCommonError;
    use domain::movie_clip::{MovieClip, MovieClipId};
    use domain::Date;

    use axum::{
        extract::rejection::{JsonRejection, PathRejection, QueryRejection},
        extract::{Json, Path, Query},
    };
    use serde::Deserialize;

    pub async fn save_movie_clip(
        movie_clip_res: Result<Json<MovieClip>, JsonRejection>,
    ) -> Result<(), AppCommonError> {
        let movie_clip = movie_clip_res?.0;
        if movie_clip.title() == "ConflictError" {
            Err(AppCommonError::ConflictError)
        } else {
            Ok(())
        }
    }

    pub async fn edit_movie_clip(
        movie_clip_res: Result<Json<MovieClip>, JsonRejection>,
    ) -> Result<(), AppCommonError> {
        let movie_clip = movie_clip_res?.0;
        if movie_clip.title() == "NoRecordError" {
            Err(AppCommonError::NoRecordError)
        } else {
            Ok(())
        }
    }

    pub async fn all_movie_clips() -> Result<Json<Vec<MovieClip>>, AppCommonError> {
        Ok(Json(Vec::new()))
    }

    #[derive(Deserialize)]
    pub struct Limit {
        pub length: usize,
    }

    pub async fn order_by_like_limit_movie_clips(
        limit_res: Result<Query<Limit>, QueryRejection>,
    ) -> Result<Json<Vec<MovieClip>>, AppCommonError> {
        let _limit = limit_res?.0;
        Ok(Json(Vec::new()))
    }

    #[derive(Deserialize)]
    pub struct DateRange {
        pub start: Date,
        pub end: Date,
    }

    pub async fn order_by_create_date_range_movie_clips(
        date_range_res: Result<Query<DateRange>, QueryRejection>,
    ) -> Result<Json<Vec<MovieClip>>, AppCommonError> {
        let _date_range = date_range_res?.0;
        Ok(Json(Vec::new()))
    }

    pub async fn remove_by_id_movie_clip(
        id: Result<Path<MovieClipId>, PathRejection>,
    ) -> Result<(), AppCommonError> {
        let id = id?.0;
        let error_id: MovieClipId = "67e55044-10b1-426f-9247-bb680e5fe0c8".parse().unwrap();
        if error_id == id {
            Ok(())
        } else {
            Err(AppCommonError::NoRecordError)
        }
    }
}

mod episode_handlers {
    use common::AppCommonError;
    use domain::episode::{Episode, EpisodeId};
    use domain::Date;

    use axum::{
        extract::rejection::{JsonRejection, PathRejection, QueryRejection},
        extract::{Json, Path, Query},
    };
    use serde::Deserialize;

    pub async fn save_episode(
        episode_res: Result<Json<Episode>, JsonRejection>,
    ) -> Result<(), AppCommonError> {
        let episode = episode_res?.0;
        if episode.content() == "ConflictError" {
            Err(AppCommonError::ConflictError)
        } else {
            Ok(())
        }
    }

    pub async fn edit_episode(
        episode_res: Result<Json<Episode>, JsonRejection>,
    ) -> Result<(), AppCommonError> {
        let episode = episode_res?.0;
        if episode.content() == "NoRecordError" {
            Err(AppCommonError::NoRecordError)
        } else {
            Ok(())
        }
    }

    pub async fn all_episodes() -> Result<Json<Vec<Episode>>, AppCommonError> {
        Ok(Json(Vec::new()))
    }

    #[derive(Deserialize)]
    pub struct DateRange {
        pub start: Date,
        pub end: Date,
    }

    pub async fn order_by_date_range_episodes(
        date_range_res: Result<Query<DateRange>, QueryRejection>,
    ) -> Result<Json<Vec<Episode>>, AppCommonError> {
        let _date_range = date_range_res?.0;
        Ok(Json(Vec::new()))
    }

    pub async fn remove_by_id_episode(
        id: Result<Path<EpisodeId>, PathRejection>,
    ) -> Result<(), AppCommonError> {
        let id = id?.0;
        let error_id: EpisodeId = "67e55044-10b1-426f-9247-bb680e5fe0c8".parse().unwrap();
        if error_id == id {
            Ok(())
        } else {
            Err(AppCommonError::NoRecordError)
        }
    }
}

#[tokio::main]
async fn main() {
    use tower_http::cors::{Any, CorsLayer};

    use axum::{
        routing::{delete, get, put},
        Router,
    };

    use episode_handlers;
    use movie_clip_handlers;

    let movie_clip_router = Router::new()
        .route(
            "/api/movie_clip",
            put(movie_clip_handlers::save_movie_clip)
                .patch(movie_clip_handlers::edit_movie_clip)
                .get(movie_clip_handlers::all_movie_clips),
        )
        .route(
            "/api/movie_clip/order_like",
            get(movie_clip_handlers::order_by_like_limit_movie_clips),
        )
        .route(
            "/api/movie_clip/order_create_date",
            get(movie_clip_handlers::order_by_create_date_range_movie_clips),
        )
        .route(
            "/api/movie_clip/:id",
            delete(movie_clip_handlers::remove_by_id_movie_clip),
        );

    let episode_router = Router::new()
        .route(
            "/api/episode",
            put(episode_handlers::save_episode)
                .patch(episode_handlers::edit_episode)
                .get(episode_handlers::all_episodes),
        )
        .route(
            "/api/episode/order_date",
            get(episode_handlers::order_by_date_range_episodes),
        )
        .route(
            "/api/episode/:id",
            delete(episode_handlers::remove_by_id_episode),
        );

    let cors_layer = CorsLayer::new()
        .allow_methods(Any)
        .allow_headers(Any)
        .allow_origin(Any);

    let app: Router<()> = Router::new()
        .merge(movie_clip_router)
        .merge(episode_router)
        .layer(cors_layer);

    axum::Server::bind(&CONFIG.test_server_addr.parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
