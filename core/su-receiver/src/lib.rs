use async_trait::async_trait;
use runner::Receiver;

pub struct SuReceiver;

#[async_trait]
impl Receiver for SuReceiver {
    async fn run() {
        println!("TODO:: Su Receiver")
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
