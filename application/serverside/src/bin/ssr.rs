use presentation::App;

use axum::{extract::State, http::Response, response::IntoResponse};
use std::convert::Infallible;

use dioxus::prelude::*;

/// dioxusアプリケーションのレンダリングを行う
fn render() -> String {
    let mut vdom = VirtualDom::new(App);
    let _ = vdom.rebuild();

    // dioxus_ssr::pre_render(&vdom)
    dioxus_ssr::render(&vdom)
}

/// レンダリングと文字列のサーブ
#[allow(dead_code)]
async fn render_and_serve(State(base_html): State<String>) -> impl IntoResponse {
    let full_html = format!(
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
        render()
    );

    let response: Response<String> = Response::builder()
        .header("Content-Type", "text/html; charset=utf-8")
        .body(full_html.into())
        .unwrap();

    Result::<_, Infallible>::Ok(response)
}

/// 状態文字列のサーブ
async fn serve_text(State(full_html): State<String>) -> impl IntoResponse {
    let response: Response<String> = Response::builder()
        .header("Content-Type", "text/html; charset=utf-8")
        .body(full_html.into())
        .unwrap();

    Result::<_, Infallible>::Ok(response)
}

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

    use std::path::Path;
    use std::sync::Arc;

    use axum::{
        http::StatusCode,
        routing::{delete, get, get_service, patch, put},
        Router,
    };
    use sqlx::postgres::PgPoolOptions;
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

    // render_and_serve
    // base_html
    let index_html_text = tokio::fs::read_to_string(dist_path.join("index.html"))
        .await
        .expect("failed to read index.html");

    let (base_html, _) = index_html_text.split_once("<body>").unwrap();

    // // ディレクトリ・ルートのサーブを行うサービス(毎回レンダリング)
    // let serve_dir = ServeDir::new(dist_path)
    //     .append_index_html_on_directories(false)
    //     .fallback(
    //         get(render_and_serve)
    //             .with_state(base_html.to_string())
    //             .map_err(|err| -> std::io::Error { match err {} }), // よくわからん
    //     );

    // ディレクトリ・ルートのサーブを行うサービス(起動時のみレンダリング)
    let full_html = format!(
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
        render()
    );
    let serve_dir = ServeDir::new(dist_path)
        .append_index_html_on_directories(false)
        .fallback(
            get(serve_text)
                .with_state(full_html)
                .map_err(|err| -> std::io::Error { match err {} }), // よくわからん
        );

    // アプリルーター
    let app_router: Router<()> = Router::new()
        .nest_service(
            "/",
            get_service(serve_dir)
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
