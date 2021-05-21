use std::error::Error;

use shadow_client::engine::engine_start;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    engine_start();
    Ok(())
}
