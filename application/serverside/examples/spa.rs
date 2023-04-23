#[tokio::main]
async fn main() {
    use config::CONFIG;
    use domain::video::{Kirinuki, Original};

    use serverside::handlers::{episode_handlers, movie_clip_handlers, video_handlers};

    use std::path::Path;
    use std::sync::Arc;

    use axum::{
        http::StatusCode,
        routing::{delete, get, get_service, patch, put},
        Router,
    };
    use tower_http::services::ServeDir;

    #[cfg(not(feature = "inmemory"))]
    let pool = async {
        use sqlx::postgres::PgPoolOptions;

        let database_url = std::env::var("DATABASE_URL").unwrap();
        PgPoolOptions::new()
            .idle_timeout(std::time::Duration::from_secs(1))
            .connect(&database_url)
            .await
            .unwrap()
    }
    .await;

    // episode_repo
    #[cfg(feature = "inmemory")]
    let episode_repo =
        Arc::new(infrastructure::episode_repository_impl::InMemoryEpisodeRepository::new());

    #[cfg(not(feature = "inmemory"))]
    let episode_repo =
        Arc::new(infrastructure::episode_repository_impl::EpisodePgDBRepository::new(pool.clone()));

    // movie_clip_repo
    #[cfg(feature = "inmemory")]
    let movie_clip_repo =
        Arc::new(infrastructure::movie_clip_repository_impl::InMemoryMovieClipRepository::new());

    #[cfg(not(feature = "inmemory"))]
    let movie_clip_repo = Arc::new(
        infrastructure::movie_clip_repository_impl::MovieClipPgDBRepository::new(pool.clone()),
    );

    // original_repo
    #[cfg(feature = "inmemory")]
    let original_repo =
        Arc::new(infrastructure::video_repository_impl::InMemoryVideoRepository::<Original>::new());

    #[cfg(not(feature = "inmemory"))]
    let original_repo = Arc::new(
        infrastructure::video_repository_impl::VideoPgDbRepository::<Original>::new(pool.clone()),
    );

    // kirinuki_repo
    #[cfg(feature = "inmemory")]
    let kirinuki_repo =
        Arc::new(infrastructure::video_repository_impl::InMemoryVideoRepository::<Kirinuki>::new());

    #[cfg(not(feature = "inmemory"))]
    let kirinuki_repo = Arc::new(
        infrastructure::video_repository_impl::VideoPgDbRepository::<Kirinuki>::new(pool.clone()),
    );

    let episode_api_router: Router<()> = Router::new()
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
        .route("/episode/:id", delete(episode_handlers::remove_episode))
        .with_state(episode_repo);

    let movie_clip_api_router: Router<()> = Router::new()
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
        )
        .with_state(movie_clip_repo);

    let original_api_router: Router<()> = Router::new()
        .route(
            "/original",
            put(video_handlers::save_video::<Original>)
                .patch(video_handlers::edit_video::<Original>)
                .get(video_handlers::all_videos::<Original>),
        )
        .route(
            "/original/query",
            get(video_handlers::get_videos_with_query::<Original>)
                .post(video_handlers::get_videos_with_query::<Original>),
        )
        .route(
            "/original/:id",
            delete(video_handlers::remove_video::<Original>),
        )
        .route(
            "/original/increment_like/:id",
            patch(video_handlers::increment_like_video::<Original>),
        )
        .with_state(original_repo);

    let kirinuki_api_router: Router<()> = Router::new()
        .route(
            "/kirinuki",
            put(video_handlers::save_video::<Kirinuki>)
                .patch(video_handlers::edit_video::<Kirinuki>)
                .get(video_handlers::all_videos::<Kirinuki>),
        )
        .route(
            "/kirinuki/query",
            get(video_handlers::get_videos_with_query::<Kirinuki>)
                .post(video_handlers::get_videos_with_query::<Kirinuki>),
        )
        .route(
            "/kirinuki/:id",
            delete(video_handlers::remove_video::<Kirinuki>),
        )
        .route(
            "/kirinuki/increment_like/:id",
            patch(video_handlers::increment_like_video::<Kirinuki>),
        )
        .with_state(kirinuki_repo);

    // distのパス
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    let dist_path = Path::new(manifest_dir).join("../../presentation/dist_spa");
    assert!(dist_path.exists());

    // アプリルーター
    let app_router: Router<()> = Router::new()
        .nest_service(
            "/",
            get_service(ServeDir::new(dist_path))
                .handle_error(|_| async move { StatusCode::NOT_FOUND }),
        )
        .nest(
            "/api",
            episode_api_router
                .merge(movie_clip_api_router)
                .merge(original_api_router)
                .merge(kirinuki_api_router),
        );

    println!("server started: {}", CONFIG.test_server_addr);

    axum::Server::bind(&CONFIG.test_server_addr.parse().unwrap())
        .serve(app_router.into_make_service())
        .await
        .unwrap();
}
