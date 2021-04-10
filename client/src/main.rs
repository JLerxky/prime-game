use std::error::Error;

use client::engine::engine_client::engine_start;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    engine_start();
    Ok(())
}
