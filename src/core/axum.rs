use crate::core::logs::{info, error};
use crate::core::router::create_router;
use std::net::SocketAddr;
use tokio::net::TcpListener;

pub async fn run_server() {
    let app = create_router();
    let port = 30723;
    let addr = SocketAddr::from(([0, 0, 0, 0], port));

    let listener = match TcpListener::bind(&addr).await {
        Ok(listener) => listener,
        Err(e) => {
            error(&format!("Failed to bind to address {}: {}", addr, e));
            return;
        }
    };

    info(&format!("Server listening on http://localhost:{}", port));

    if let Err(e) = axum::serve(listener, app.into_make_service()).await {
        error(&format!("Server error: {}", e));
    }
}
