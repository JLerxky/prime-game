use protocol::packet::Packet;
use server::{cli::cli, engine::engine_server::engine_start, net::net_server::net_server_start};
use tokio::{runtime::Runtime, sync::mpsc};

fn main() {
    let engine_runtime = Runtime::new().unwrap();
    let net_runtime = Runtime::new().unwrap();
    let cli_runtime = Runtime::new().unwrap();

    let (net_tx, net_rx) = mpsc::channel::<Packet>(100);
    let (engine_tx, engine_rx) = mpsc::channel::<Packet>(100);

    // cli模块
    cli_runtime.spawn(async { cli::Cli::cli_start() });
    // 网络服务器模块
    net_runtime.spawn(net_server_start(net_tx, engine_rx));
    // 物理引擎主循环
    engine_runtime.block_on(engine_start(net_rx, engine_tx));
}
