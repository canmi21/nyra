mod core;
use core::logs::{debug, error};

#[tokio::main]
async fn main() {
    debug("Server starting...");
    if let Err(e) = core::bootstrap::start().await {
        error(&format!("Failed to start server: {}", e));
    }
}
