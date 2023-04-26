use presentation::App;

use axum::{extract::State, http::Response, response::IntoResponse};
use dioxus::prelude::*;
use shuttle_axum::ShuttleAxum;
use shuttle_runtime::CustomError as ShuttleCustomError;
use sqlx::postgres::PgPool;
use std::convert::Infallible;
use std::path::PathBuf;

/// dioxusアプリケーションのレンダリングを行う
fn render() -> String {
    let mut vdom = VirtualDom::new(App);
    let _ = vdom.rebuild();

    // dioxus_ssr::pre_render(&vdom)
    dioxus_ssr::render(&vdom)
}

/// 状態文字列のサーブ
async fn serve_text(State(full_html): State<String>) -> impl IntoResponse {
    let response: Response<String> = Response::builder()
        .header("Content-Type", "text/html; charset=utf-8")
        .body(full_html.into())
        .unwrap();

    Result::<_, Infallible>::Ok(response)
}

/// メインサーバー
#[shuttle_runtime::main]
async fn main_server(
    #[shuttle_shared_db::Postgres(
        local_uri = "postgres://postgres:{secrets.PASSWORD}@localhost/rikou_meitan"
    )]
    pool: PgPool,
    #[shuttle_static_folder::StaticFolder(folder = "dist_ssr")] static_folder: PathBuf,
) -> ShuttleAxum {
    use domain::video::{Kirinuki, Original};

    use serverside::handlers::{episode_handlers, movie_clip_handlers, video_handlers};

    use std::sync::Arc;

    use axum::{
        http::StatusCode,
        routing::{delete, get, get_service, patch, put},
        Router,
    };

    use tower::ServiceExt;
    use tower_http::services::ServeDir;

    // データベースのマイグレーション．
    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .map_err(|e| ShuttleCustomError::msg(format!("Migration error. {e}")))?;

    // // Htmlの作成・ディレクトリサーバー

    let index_html_text = include_str!("../dist_ssr/index.html");

    let (base_html, _) = index_html_text.split_once("<body>").unwrap();

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

    let serve_dir = ServeDir::new(static_folder)
        .append_index_html_on_directories(false)
        .fallback(
            get(serve_text)
                .with_state(full_html)
                .map_err(|err| -> std::io::Error { match err {} }), // よくわからん
        );

    // EpisodeについてのAPI
    let episode_repo =
        Arc::new(infrastructure::episode_repository_impl::EpisodePgDBRepository::new(pool.clone()));
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

    // MovieClipについてのAPI
    let movie_clip_repo = Arc::new(
        infrastructure::movie_clip_repository_impl::MovieClipPgDBRepository::new(pool.clone()),
    );
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

    // OriginalについてのAPI
    let original_repo = Arc::new(
        infrastructure::video_repository_impl::VideoPgDbRepository::<Original>::new(pool.clone()),
    );
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

    // KirinukiについてのAPI
    let kirinuki_repo = Arc::new(
        infrastructure::video_repository_impl::VideoPgDbRepository::<Kirinuki>::new(pool.clone()),
    );
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
                .merge(kirinuki_api_router),
        );
    Ok(app_router.into())
}
