use common::GameEvent;
use std::error::Error;
use tokio::sync::mpsc;

use server::{engine::engine_server::engine_start, net::net_server::start_server};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let (net_tx, net_rx) = mpsc::channel::<GameEvent>(100);
    let (engine_tx, engine_rx) = mpsc::channel::<GameEvent>(100);

    let net_server = start_server(net_tx, engine_rx);
    let engine_server = engine_start(engine_tx, net_rx);
    match tokio::try_join!(engine_server, net_server) {
        Ok(_) => {},
        Err(e) => println!("服务器启动失败: {}", e),
    }
    Ok(())
}
