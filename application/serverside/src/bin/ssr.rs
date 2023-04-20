use presentation::App;

use axum::{extract::State, http::Response, response::IntoResponse};
use std::convert::Infallible;

use dioxus::prelude::*;

async fn render(State(base_html): State<String>) -> impl IntoResponse {
    let mut vdom = VirtualDom::new(App);
    let _ = vdom.rebuild();

    let body_content = format!(
        r#"
{}
    <body>
        <div id="main">
{}
        </div>
    </body>
</html>
        "#,
        base_html,
        dioxus_ssr::pre_render(&vdom)
    );

    let response: Response<String> = Response::builder()
        .header("Content-Type", "text/html; charset=utf-8")
        .body(body_content.into())
        .unwrap();

    Result::<_, Infallible>::Ok(response)
}

#[tokio::main]
async fn main() {
    use config::CONFIG;
    use domain::video::{Kirinuki, Original};
    use serverside::app_state::InMemoryAppState;
    use serverside::handlers::{
        episode_handlers, kirinuki_handlers, movie_clip_handlers, original_handlers,
    };

    use std::path::Path;
    use std::sync::Arc;

    use axum::{
        http::StatusCode,
        routing::{delete, get, get_service, patch, put},
        Router,
    };
    use tower::ServiceExt;
    use tower_http::services::ServeDir;

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

    #[cfg(feature = "inmemory")]
    type AppState = InMemoryAppState;

    // 各種APIルーター
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

    // distのパス
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    let dist_path = Path::new(manifest_dir).join("../../presentation/dist_ssr");
    assert!(dist_path.exists());

    // base_html
    let index_html_text = tokio::fs::read_to_string(dist_path.join("index.html"))
        .await
        .expect("failed to read index.html");

    let (base_html, _) = index_html_text.split_once("<body>").unwrap();

    // アプリルーター
    let app_router: Router<()> = Router::new()
        .nest_service(
            "/",
            get_service(
                ServeDir::new(dist_path)
                    .append_index_html_on_directories(false)
                    .fallback(
                        get(render)
                            .with_state(base_html.to_string())
                            .map_err(|err| -> std::io::Error { match err {} }), // よくわからん
                    ),
            )
            .handle_error(|_| async move { StatusCode::INTERNAL_SERVER_ERROR }),
        )
        .nest(
            "/api",
            episode_api_router
                .merge(movie_clip_api_router)
                .merge(original_api_router)
                .merge(kirinuki_api_router)
                .with_state(app_state),
        );

    println!("server started: {}", CONFIG.test_server_addr);

    axum::Server::bind(&CONFIG.test_server_addr.parse().unwrap())
        .serve(app_router.into_make_service())
        .await
        .unwrap();
}
