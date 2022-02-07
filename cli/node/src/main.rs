use intmax_config::{Config, ConfigKind};
use intmax_runner::gen_runner;

async fn run() {
    let config = Config::new(ConfigKind::DEV).expect("setup config file error.");
    let runner = gen_runner(&config);
    runner.run().await;
}

#[tokio::main]
async fn main() {
    println!("Hello, world!");
    run().await;
}
