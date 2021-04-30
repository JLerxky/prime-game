use std::error::Error;

use common::tile_map::create_init_map;
use server::engine::engine_server::engine_start;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // let _ = engine_start().await;
    create_init_map();
    Ok(())
}
