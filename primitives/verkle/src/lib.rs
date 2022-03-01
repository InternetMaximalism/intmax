use codec::{Decode, Encode};

#[derive(Debug)]
pub enum StorageError {
    InvalidError,
}

pub struct Proof {
    pub sibilings: Vec<Vec<u8>>,
    pub leaf: Vec<u8>,
}

pub type Result<T> = std::result::Result<T, StorageError>;

pub trait VerckleStorage {
    fn inclusion_proof<K: Clone + AsRef<[u8]>>(&self, key: &K) -> Result<Proof>;
    fn verify_proof(&self, proof: &Proof) -> Result<()>;
    fn put<K: Clone + AsRef<[u8]>, V: Encode>(&self, key: &K, data: &V) -> Result<()>;
    fn remove<K: Clone + AsRef<[u8]>, V: Encode>(&self, key: &K, data: &V) -> Result<()>;
    fn get<K: Clone + AsRef<[u8]>, V: Decode>(&self, key: &K) -> Option<V>;
}

pub trait VerckleSMTStorage: VerckleStorage {
    fn inclusion_empty_proof<K: Clone + AsRef<[u8]>>(&self, key: &K) -> Result<Proof>;
}

#[cfg(test)]
mod tests {
    use super::*;

    struct MockVeckleStorage;

    impl VerckleStorage for MockVeckleStorage {
        fn inclusion_proof<K: Clone + AsRef<[u8]>>(&self, _key: &K) -> Result<Proof> {
            Ok(Proof {
                sibilings: vec![],
                leaf: vec![],
            })
        }
        fn verify_proof(&self, _proof: &Proof) -> Result<()> {
            Ok(())
        }
        fn put<K: Clone + AsRef<[u8]>, V: Encode>(&self, _key: &K, _data: &V) -> Result<()> {
            Ok(())
        }
        fn remove<K: Clone + AsRef<[u8]>, V: Encode>(&self, _key: &K, _data: &V) -> Result<()> {
            Ok(())
        }
        fn get<K: Clone + AsRef<[u8]>, V: Decode>(&self, _key: &K) -> Option<V> {
            None
        }
    }

    impl VerckleSMTStorage for MockVeckleStorage {
        fn inclusion_empty_proof<K: Clone + AsRef<[u8]>>(&self, _key: &K) -> Result<Proof> {
            Ok(Proof {
                sibilings: vec![],
                leaf: vec![],
            })
        }
    }

    #[test]
    fn wrong_test() {
        let storage = MockVeckleStorage {};

        let k = vec![5, 10];
        let v: i32 = 10;
        storage.put(&k, &v).expect("put ok.");

        let proof = storage
            .inclusion_proof(&k)
            .expect("failed inclusion proof.");
        storage.verify_proof(&proof).expect("failed verify proof");
    }
}
