#[cfg(feature = "mock")]
#[tokio::main]
async fn main() {
    use server::app_state::MockAppState;
    use server::handlers::{episode_handlers, movie_clip_handlers};

    use axum::{
        routing::{delete, get, put},
        Router,
    };

    use tower_http::cors::{Any, CorsLayer};

    let app_state = MockAppState::default();

    let movie_clip_router = Router::new()
        .route(
            "/movie_clip",
            put(movie_clip_handlers::save_movie_clip_handler)
                .patch(movie_clip_handlers::edit_movie_clip_handler)
                .get(movie_clip_handlers::all_movie_clips_handler),
        )
        .route(
            "/movie_clip/order_like",
            get(movie_clip_handlers::order_by_like_limit_movie_clips_handler),
        )
        .route(
            "/movie_clip/order_create_date",
            get(movie_clip_handlers::order_by_create_date_range_movie_clips_handler),
        )
        .route(
            "/movie_clip/:id",
            delete(movie_clip_handlers::remove_by_id_movie_clip_handler),
        )
        .with_state(app_state.clone());

    let episode_router = Router::new()
        .route(
            "/episode",
            put(episode_handlers::save_episode_handler)
                .patch(episode_handlers::edit_episode_handler)
                .get(episode_handlers::all_episodes_handler),
        )
        .route(
            "/episode/order_date",
            get(episode_handlers::order_by_date_range_episodes_handler),
        )
        .route(
            "/episode/:id",
            delete(episode_handlers::remove_by_id_episode_handler),
        )
        .with_state(app_state.clone());

    let cors_layer = CorsLayer::new()
        .allow_methods(Any)
        .allow_headers(Any)
        .allow_origin(Any);

    let app: Router<()> = Router::new()
        .merge(movie_clip_router)
        .merge(episode_router)
        .layer(cors_layer);

    axum::Server::bind(&"127.0.0.1:8000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

#[cfg(not(feature = "mock"))]
fn main() {
    println!("--features mock を指定してください");
}
