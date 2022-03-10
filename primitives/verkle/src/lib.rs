use bytes::BufMut;
use codec::Encode;
use franklin_crypto::bellman::bn256::Fr;
use franklin_crypto::bellman::bn256::G1Affine;
use verkle_tree::bn256_verkle_tree::leaf::LeafNodeWith32BytesValue;
use verkle_tree::bn256_verkle_tree::proof::VerkleProof;
use verkle_tree::bn256_verkle_tree::VerkleTreeWith32BytesKeyValue;
use verkle_tree::ipa_fr::config::IpaConfig;

#[derive(Debug)]
pub enum StorageError {
    InvalidError,
}

pub type Result<T> = std::result::Result<T, StorageError>;

pub type Proof =
    VerkleProof<[u8; 32], LeafNodeWith32BytesValue<G1Affine>, G1Affine, IpaConfig<G1Affine>>;

pub type Elements = verkle_tree::verkle_tree::witness::Elements<Fr>;

#[derive(Debug)]
pub struct Storage {
    tree: VerkleTreeWith32BytesKeyValue,
    committer: IpaConfig<G1Affine>,
}

pub trait VerkleStorage {
    fn new() -> Self;
    fn put<K: Clone + AsRef<[u8]>, V: Encode>(&mut self, key: &K, data: &V) -> Result<()>;
    fn get<K: Clone + AsRef<[u8]>>(&self, key: &K) -> Result<std::option::Option<&[u8; 32]>>;
    fn remove<K: Clone + AsRef<[u8]>>(&mut self, key: &K) -> Result<()>;
    fn inclusion_proof<K: Clone + AsRef<[u8]>>(&mut self, key: &K) -> Result<(Proof, Elements)>;
    fn verify_proof(&mut self, proof: Proof, elements: Elements) -> Result<()>;
}

pub trait VerkleSMTStorage: VerkleStorage {
    fn inclusion_empty_proof<K: Clone + AsRef<[u8]>>(&self, key: &K) -> Result<Proof>;
}

fn generate_byte_key<K: Clone + AsRef<[u8]>>(key: &K) -> [u8; 32] {
    let raw = key.as_ref();
    let raw_u8: &[u8] = &raw[..];
    let mut byte_key = [0; 32];
    let mut buf = &mut byte_key[..];
    buf.put(&raw_u8[..]);

    byte_key
}

fn generate_byte_value<V: Encode>(value: &V) -> [u8; 32] {
    let raw: &[u8] = &value.encode();
    let mut byte_value = [0; 32];
    let mut buf = &mut byte_value[..];
    buf.put(&raw[..]);

    byte_value
}

impl VerkleStorage for Storage {
    fn new() -> Self {
        let domain_size = 256;
        let committer = IpaConfig::new(domain_size);
        let tree: VerkleTreeWith32BytesKeyValue =
            VerkleTreeWith32BytesKeyValue::new(committer.clone());

        Self { tree, committer }
    }

    fn put<K: Clone + AsRef<[u8]>, V: Encode>(&mut self, key: &K, data: &V) -> Result<()> {
        let byte_key = generate_byte_key(key);
        let byte_value = generate_byte_value(data);

        VerkleTreeWith32BytesKeyValue::insert(&mut self.tree, byte_key, byte_value);

        Ok(())
    }

    fn get<K: Clone + AsRef<[u8]>>(&self, _key: &K) -> Result<std::option::Option<&[u8; 32]>> {
        let byte_key = generate_byte_key(_key);
        let stored_value: Option<&[u8; 32]> =
            VerkleTreeWith32BytesKeyValue::get(&self.tree, &byte_key);

        Ok(stored_value)
    }

    fn remove<K: Clone + AsRef<[u8]>>(&mut self, _key: &K) -> Result<()> {
        let byte_key = generate_byte_key(_key);

        VerkleTreeWith32BytesKeyValue::remove(&mut self.tree, &byte_key);

        Ok(())
    }

    fn inclusion_proof<K: Clone + AsRef<[u8]>>(&mut self, _key: &K) -> Result<(Proof, Elements)> {
        let byte_key = generate_byte_key(_key);

        let keys = [byte_key];
        let (proof, elements) = VerkleProof::create(&mut self.tree, &keys).unwrap();

        Ok((proof, elements))
    }

    fn verify_proof(&mut self, proof: Proof, elements: Elements) -> Result<()> {
        let is_valid: bool =
            VerkleProof::check(&proof, &elements.zs, &elements.ys, &self.committer).unwrap();
        println!("is_valid: {:?}", is_valid);

        match VerkleProof::check(&proof, &elements.zs, &elements.ys, &self.committer).unwrap() {
            true => Ok(()),
            false => Err(StorageError::InvalidError),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use byteorder::{ByteOrder, LittleEndian};
    use codec::Decode;

    #[derive(Encode, Decode, PartialEq, Debug)]
    struct Value {
        pub a: u32,
    }

    fn get_index(v: Vec<u32>) -> usize {
        for (i, &item) in v.iter().enumerate() {
            if item == 0 {
                return i;
            }
        }

        return v.len();
    }

    fn get_value(v: &[u8; 32], index: usize) -> Value {
        let mut raw: &[u8] = &v[0..index + 3];

        Value::decode(&mut raw).unwrap()
    }

    fn get_step_vec(v: &[u8; 32]) -> Vec<u32> {
        let mut result: Vec<u32> = Vec::new();
        for i in (0..v.len()).step_by(4) {
            result.push(LittleEndian::read_u32(&v[i..]));
        }

        result
    }

    fn get_decoded(v: &[u8; 32]) -> Value {
        let vec = get_step_vec(v);
        let index = get_index(vec);
        let decoded = get_value(&v, index);

        decoded
    }

    #[test]
    fn works_put() {
        let mut storage: Storage = VerkleStorage::new();

        let k = vec![5, 10];
        let v: i32 = 10;
        let res = storage.put(&k, &v).expect("put ok.");
        assert_eq!(res, ());
    }

    #[test]
    fn works_get() {
        let mut storage: Storage = VerkleStorage::new();

        let k = vec![5, 10];
        let v = Value { a: 10 };
        storage.put(&k, &v).expect("put ok.");
        let res = storage.get(&k).expect("get ok.").unwrap();

        let decoded = get_decoded(res);

        assert_eq!(v, decoded);
    }

    #[test]
    fn works_remove() {
        let mut storage: Storage = VerkleStorage::new();

        let k = vec![5, 10];
        let v = Value { a: 10 };

        storage.put(&k, &v).expect("put ok.");
        let res = storage.get(&k).expect("get ok.");
        assert!(res.is_some());

        storage.remove(&k).expect("remove ok.");
        let res = storage.get(&k).expect("get ok.");
        assert!(res.is_none());
    }

    #[test]
    fn works_verify() {
        let mut storage: Storage = VerkleStorage::new();

        let k = vec![5, 10];
        let v = Value { a: 10 };
        storage.put(&k, &v).expect("put ok.");

        let (proof, elements) = storage.inclusion_proof(&k).expect("inclusion proof ok.");
        let res = storage
            .verify_proof(proof, elements)
            .expect("verify proof ok");
        assert_eq!(res, ());
    }
}
