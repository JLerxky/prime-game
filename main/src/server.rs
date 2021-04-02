use std::error::Error;

use server::engine::engine_server::engine_start;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let _ = engine_start().await;
    Ok(())
}
