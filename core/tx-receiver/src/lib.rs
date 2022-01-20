use async_trait::async_trait;
use runner::Receiver;

pub struct TxReceiver;

#[async_trait]
impl Receiver for TxReceiver {
    async fn run() {
        println!("TODO:: Tx Receiver")
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
