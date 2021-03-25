
use std::error::Error;

use server::net::net_server::start_server;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // let net_server = start_server();
    // tokio::task::spawn(async { start_server().await.unwrap() });
    // match tokio::try_join!(net_server) {
    //     Ok(_) => println!("游戏服务器启动完成！"),
    //     Err(e) => println!("服务器启动失败: {}", e),
    // }
    start_server().await.unwrap();
    // engine_start();
    Ok(())
}
