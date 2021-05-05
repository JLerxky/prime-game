use std::error::Error;

use server::engine::engine_server::engine_start;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let cli = async move { server::cli::cli::Cli::start() };
    let engine = engine_start();
    let _ = tokio::join!(cli, engine);
    Ok(())
}
