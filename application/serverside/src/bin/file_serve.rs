use std::path::Path;

#[tokio::main]
async fn main() {
    use axum::{http::StatusCode, routing::get_service, Router};
    use tower_http::services::ServeDir;

    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    let dist_path = Path::new(manifest_dir).join("../../presentation/dist");
    assert!(dist_path.exists());

    let app: Router<()> = Router::new().nest_service(
        "/",
        get_service(ServeDir::new(dist_path))
            .handle_error(|_| async move { StatusCode::NOT_FOUND }),
    );

    axum::Server::bind(&"127.0.0.1:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
