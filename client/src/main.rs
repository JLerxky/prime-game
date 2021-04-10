// 纯GUI窗口运行, 无命令行窗口
#![windows_subsystem = "windows"]

use std::error::Error;

use client::engine::engine_client::engine_start;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    engine_start();
    Ok(())
}
