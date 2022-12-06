#[cfg(feature = "server")]
#[tokio::main]
async fn main() {
    use config::CONFIG;
    use infrastructure::episode_repository_impl::InMemoryEpisodeRepository;
    use infrastructure::movie_clip_repository_impl::InMemoryMovieClipRepository;
    use serverside::app_state::InMemoryAppState;
    use serverside::handlers::{episode_handlers, movie_clip_handlers};

    use std::sync::Arc;
    use tower_http::cors::{Any, CorsLayer};

    use axum::{
        routing::{delete, get, put},
        Router,
    };

    let app_state = InMemoryAppState {
        movie_clip_repo: Arc::new(InMemoryMovieClipRepository::new()),
        episode_repo: Arc::new(InMemoryEpisodeRepository::new()),
    };

    let movie_clip_router: Router<()> = Router::new()
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
        )
        .with_state(app_state.clone());

    let episode_router: Router<()> = Router::new()
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
        )
        .with_state(app_state);

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

#[cfg(not(feature = "server"))]
fn main() {}
