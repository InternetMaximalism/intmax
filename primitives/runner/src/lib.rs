use async_trait::async_trait;

pub trait Runner {
    type TxReceiver: Receiver;
    type SuReceiver: Receiver;
    fn run() {
        tokio::spawn(async {
            Self::TxReceiver::run().await;
        });
        tokio::spawn(async {
            Self::SuReceiver::run().await;
        });
    }
}

#[async_trait]
pub trait Receiver {
    async fn run();
}

#[cfg(test)]
mod tests {
    use super::*;
    use state::Storage;
    use std::{sync::RwLock, thread, time};

    static VALUE: Storage<RwLock<i32>> = Storage::new();

    struct MockReceiver1;
    #[async_trait]
    impl Receiver for MockReceiver1 {
        async fn run() {
            let ten_millis = time::Duration::from_millis(10);
            thread::sleep(1 * ten_millis);
            let mut value = VALUE.get().write().unwrap();
            *value = 1;
            // println!("sp1 done {:?}", VALUE.get().read().unwrap());
        }
    }

    struct MockReceiver2;
    #[async_trait]
    impl Receiver for MockReceiver2 {
        async fn run() {
            let ten_millis = time::Duration::from_millis(10);
            thread::sleep(50 * ten_millis);
            let mut value = VALUE.get().write().unwrap();
            *value = 2;
        }
    }

    struct Runtime;
    impl Runner for Runtime {
        type TxReceiver = MockReceiver1;
        type SuReceiver = MockReceiver2;
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn it_works() {
        let ten_millis = time::Duration::from_millis(10);
        VALUE.set(RwLock::new(0));

        Runtime::run();
        assert_eq!(*VALUE.get().read().unwrap(), 0);

        thread::sleep(10 * ten_millis);
        assert_eq!(*VALUE.get().read().unwrap(), 1);

        thread::sleep(100 * ten_millis);
        assert_eq!(*VALUE.get().read().unwrap(), 2);
    }
}
