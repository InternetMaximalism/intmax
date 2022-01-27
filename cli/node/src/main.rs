use intmax_config::{Config, ConfigKind};
use intmax_runner::gen_runner;

fn run() {
    let config = Config::new(ConfigKind::DEV).expect("setup config file error.");
    let runner = gen_runner(&config);
    runner.run();
}

fn main() {
    println!("Hello, world!");
    run();
}
