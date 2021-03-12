// 纯GUI窗口运行, 无命令行窗口
// #![windows_subsystem = "windows"]
mod cli;
mod data;
mod engine;
mod net;
mod util;

use cli::cli::Cli;
use engine::engine::engine_start;
use tokio::task;

#[tokio::main]
async fn main() {
    // task::spawn(async {
    //     Cli::start();
    // });

    engine_start();
}
