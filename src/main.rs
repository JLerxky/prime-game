mod cli;
mod data;
mod engine;
mod net;
mod util;

use cli::cli::Cli;
use engine::engine::run_snake;
use tokio::task;

#[tokio::main]
async fn main() {
    task::spawn(async {
        Cli::start();
    });

    run_snake();
}
