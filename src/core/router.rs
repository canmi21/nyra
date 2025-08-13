use axum::{routing::get_service, Router};
use tower_http::services::{ServeDir, ServeFile};

pub fn create_router() -> Router {
    let static_files_service = ServeDir::new("dist")
        .fallback(ServeFile::new("dist/index.html"));
    Router::new().fallback_service(get_service(static_files_service))
}
