use runner::Runner;
use su_receiver::SuReceiver;
use tx_receiver::TxReceiver;

struct Runtime;
impl Runner for Runtime {
    type TxReceiver = TxReceiver;
    type SuReceiver = SuReceiver;
}

fn run() {
    Runtime::run();
    loop {}
}

#[tokio::main]
async fn main() {
    println!("Hello, world!");
    run();
}
