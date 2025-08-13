use crate::core::axum::run_server;
use std::io::Error;

pub async fn start() -> Result<(), Error> {
    run_server().await;
    Ok(())
}
