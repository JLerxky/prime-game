use cli::cli::Cli;
use engine::engine;
use tokio::task;

#[tokio::main]
async fn main() {
    task::spawn(async {
        Cli::start();
    });

    engine::engine_start();
}
