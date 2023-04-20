#[tokio::main]
async fn main() {
    use config::CONFIG;
    use domain::video::{Kirinuki, Original};

    #[cfg(not(feature = "inmemory"))]
    use serverside::app_state::AppState;

    #[cfg(feature = "inmemory")]
    use serverside::app_state::InMemoryAppState;

    use serverside::handlers::{
        episode_handlers, kirinuki_handlers, movie_clip_handlers, original_handlers,
    };

    use std::sync::Arc;
    use tower_http::cors::{Any, CorsLayer};

    use axum::{
        routing::{delete, get, patch, put},
        Router,
    };
    use sqlx::postgres::PgPoolOptions;

    // inmemoryの場合
    #[cfg(feature = "inmemory")]
    let app_state = {
        use infrastructure::episode_repository_impl::InMemoryEpisodeRepository;
        use infrastructure::movie_clip_repository_impl::InMemoryMovieClipRepository;
        use infrastructure::video_repository_impl::InMemoryVideoRepository;

        InMemoryAppState {
            movie_clip_repo: Arc::new(InMemoryMovieClipRepository::new()),
            episode_repo: Arc::new(InMemoryEpisodeRepository::new()),
            original_repo: Arc::new(InMemoryVideoRepository::<Original>::new()),
            kirinuki_repo: Arc::new(InMemoryVideoRepository::<Kirinuki>::new()),
        }
    };

    // データベースの場合
    #[cfg(not(feature = "inmemory"))]
    let app_state = async {
        use infrastructure::episode_repository_impl::EpisodePgDBRepository;
        use infrastructure::movie_clip_repository_impl::MovieClipPgDBRepository;
        use infrastructure::video_repository_impl::VideoPgDbRepository;

        let database_url = std::env::var("DATABASE_URL").unwrap();
        let pool = PgPoolOptions::new()
            .idle_timeout(std::time::Duration::from_secs(1))
            .connect(&database_url)
            .await
            .unwrap();

        AppState {
            movie_clip_repo: Arc::new(MovieClipPgDBRepository::new(pool.clone())),
            episode_repo: Arc::new(EpisodePgDBRepository::new(pool.clone())),
            original_repo: Arc::new(VideoPgDbRepository::<Original>::new(pool.clone())),
            kirinuki_repo: Arc::new(VideoPgDbRepository::<Kirinuki>::new(pool.clone())),
        }
    }
    .await;

    #[cfg(feature = "inmemory")]
    type AppState = InMemoryAppState;

    let episode_api_router: Router<AppState> = Router::new()
        .route(
            "/episode",
            put(episode_handlers::save_episode)
                .patch(episode_handlers::edit_episode)
                .get(episode_handlers::all_episodes),
        )
        .route(
            "/episode/query",
            get(episode_handlers::get_episodes_with_query),
        )
        .route("/episode/:id", delete(episode_handlers::remove_episode));

    let movie_clip_api_router: Router<AppState> = Router::new()
        .route(
            "/movie_clip",
            put(movie_clip_handlers::save_movie_clip)
                .patch(movie_clip_handlers::edit_movie_clip)
                .get(movie_clip_handlers::all_movie_clips),
        )
        .route(
            "/movie_clip/query",
            get(movie_clip_handlers::get_movie_clips_with_query)
                .post(movie_clip_handlers::get_movie_clips_with_query),
        )
        .route(
            "/movie_clip/:id",
            delete(movie_clip_handlers::remove_movie_clip),
        )
        .route(
            "/movie_clip/increment_like/:id",
            patch(movie_clip_handlers::increment_like_movie_clip),
        );

    let original_api_router: Router<AppState> = Router::new()
        .route(
            "/original",
            put(original_handlers::save_original)
                .patch(original_handlers::edit_original)
                .get(original_handlers::all_originals),
        )
        .route(
            "/original/query",
            get(original_handlers::get_originals_with_query)
                .post(original_handlers::get_originals_with_query),
        )
        .route("/original/:id", delete(original_handlers::remove_original))
        .route(
            "/original/increment_like/:id",
            patch(original_handlers::increment_like_original),
        );

    let kirinuki_api_router: Router<AppState> = Router::new()
        .route(
            "/kirinuki",
            put(kirinuki_handlers::save_kirinuki)
                .patch(kirinuki_handlers::edit_kirinuki)
                .get(kirinuki_handlers::all_kirinukis),
        )
        .route(
            "/kirinuki/query",
            get(kirinuki_handlers::get_kirinukis_with_query)
                .post(kirinuki_handlers::get_kirinukis_with_query),
        )
        .route("/kirinuki/:id", delete(kirinuki_handlers::remove_kirinuki))
        .route(
            "/kirinuki/increment_like/:id",
            patch(kirinuki_handlers::increment_like_kirinuki),
        );

    let cors_layer = CorsLayer::new()
        .allow_methods(Any)
        .allow_headers(Any)
        .allow_origin(Any);

    let app_router: Router<()> = Router::new()
        .nest(
            "/api",
            episode_api_router
                .merge(movie_clip_api_router)
                .merge(original_api_router)
                .merge(kirinuki_api_router)
                .with_state(app_state),
        )
        .layer(cors_layer);

    println!("server started: {}", CONFIG.test_server_addr);

    axum::Server::bind(&CONFIG.test_server_addr.parse().unwrap())
        .serve(app_router.into_make_service())
        .await
        .unwrap();
}
