use std::io;

use codec::{Decode, Encode};
pub use kvdb::DBKey;
use kvdb::{DBTransaction, DBValue, KeyValueDB};

/// An identifier for a column.
pub type ColumnId = u32;

pub struct Db<DB: KeyValueDB> {
    db: DB,
    col: ColumnId,
}

impl<DB: KeyValueDB> Db<DB> {
    pub fn new(db: DB, col: ColumnId) -> Db<DB> {
        Db { db, col }
    }
}

pub struct DBTx {
    tx: DBTransaction,
    col: ColumnId,
}

impl DBTx {
    /// Insert a key-value pair in the transaction. Any existing value will be overwritten upon write.
    pub fn put<K: Clone + AsRef<[u8]>, V: Encode>(&mut self, key: &K, value: &V) {
        self.tx.put(self.col, key.as_ref(), value.encode().as_ref());
    }

    /// Delete value by key.
    pub fn delete<K: Clone + AsRef<[u8]>>(&mut self, key: &K) {
        self.tx.delete(self.col, key.as_ref());
    }

    /// Delete all values with the given key prefix.
    /// Using an empty prefix here will remove all keys
    /// (all keys start with the empty prefix).
    pub fn delete_prefix<K: Clone + AsRef<[u8]>>(&mut self, prefix: &K) {
        self.tx.delete_prefix(self.col, prefix.as_ref());
    }
}

pub trait Database: Send + Sync {
    /// Commit the `transaction` to the database atomically. Any further calls to `get` or `lookup`
    /// will reflect the new state.
    fn commit(&self, tx: DBTx) -> io::Result<()>;

    fn get_raw(&self, key: &DBKey) -> Option<DBValue>;

    /// Retrieve the value previously stored against `key` or `None` if
    /// `key` is not currently in the database.
    fn get<V: Decode>(&self, key: &DBKey) -> Option<V> {
        if let Some(res) = self.get_raw(key) {
            return V::decode(&mut &res[..]).ok();
        }
        None
    }

    /// Check if the value exists in the database without retrieving it.
    fn contains(&self, key: &DBKey) -> bool {
        if let Some(_) = self.get_raw(key) {
            return true;
        }
        false
    }

    /// Check value size in the database possibly without retrieving it.
    fn value_size(&self, key: &DBKey) -> Option<usize> {
        self.get_raw(key).map(|v| v.len())
    }
}

impl<DB: KeyValueDB> Database for Db<DB> {
    fn commit(&self, tx: DBTx) -> io::Result<()> {
        self.db.write(tx.tx)
    }

    fn get_raw(&self, key: &DBKey) -> Option<DBValue> {
        if let Ok(res) = self.db.get(self.col, key.as_ref()) {
            return res;
        }
        None
    }
}

impl<DB: KeyValueDB> Db<DB> {
    pub fn make_tx(&self) -> DBTx {
        DBTx {
            tx: DBTransaction::new(),
            col: self.col,
        }
    }

    pub fn make_tx_with_capacity(&self, cap: usize) -> DBTx {
        DBTx {
            tx: DBTransaction::with_capacity(cap),
            col: self.col,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Encode, Decode, PartialEq, Debug)]
    struct ValueA {
        pub a: u32,
        pub b: u32,
    }

    #[derive(Encode, Decode, PartialEq, Debug)]
    struct ValueB {
        pub x: u128,
        pub y: String,
    }

    fn try_test_db<DB: KeyValueDB>(db: Db<DB>) {
        let mut tx = db.make_tx();
        let value_a = ValueA { a: 2, b: 3 };
        tx.put(&vec![0, 1], &value_a);
        let value_b = ValueB {
            x: 10,
            y: String::from("abc"),
        };
        tx.put(&vec![0, 2], &value_b);
        let value_b_2 = ValueB {
            x: 100,
            y: String::from("xyz"),
        };
        tx.put(&vec![1, 1], &value_b_2);

        assert!(db.commit(tx).is_ok());

        let ret_value_a: Option<ValueA> = db.get(&DBKey::from_vec(vec![0, 1]));
        assert_eq!(ret_value_a, Some(value_a));

        let ret_value_b: Option<ValueB> = db.get(&DBKey::from_vec(vec![0, 2]));
        assert_eq!(ret_value_b, Some(value_b));

        let ret_value_b_2: Option<ValueB> = db.get(&DBKey::from_vec(vec![1, 1]));
        assert_eq!(ret_value_b_2, Some(value_b_2));

        let ret_value_ng_a: Option<ValueA> = db.get(&DBKey::from_vec(vec![0, 3]));
        assert_eq!(ret_value_ng_a, None);

        let ret_value_ng_b: Option<ValueB> = db.get(&DBKey::from_vec(vec![0, 4]));
        assert_eq!(ret_value_ng_b, None);

        let ret_value_b_2: Option<ValueB> = db.get(&DBKey::from_vec(vec![1, 2]));
        assert_eq!(ret_value_b_2, None);

        let mut tx_2 = db.make_tx();
        tx_2.delete(&DBKey::from_vec(vec![1, 1]));
        tx_2.delete_prefix(&DBKey::from_vec(vec![0]));

        assert!(db.commit(tx_2).is_ok());

        let none_value_a: Option<ValueA> = db.get(&DBKey::from_vec(vec![0, 1]));
        assert_eq!(none_value_a, None);

        let none_value_b: Option<ValueB> = db.get(&DBKey::from_vec(vec![0, 2]));
        assert_eq!(none_value_b, None);

        let none_value_b_2: Option<ValueB> = db.get(&DBKey::from_vec(vec![1, 1]));
        assert_eq!(none_value_b_2, None);
    }

    #[test]
    fn it_works_onmemory() {
        let db = Db::<kvdb_memorydb::InMemory>::new(kvdb_memorydb::create(256), 128);
        try_test_db(db);
    }

    use kvdb_rocksdb::{Database as RocksDB, DatabaseConfig};

    #[test]
    fn it_works_rocksdb() {
        let config = DatabaseConfig::with_columns(64);
        let dir = tempfile::Builder::new()
            .prefix("rocksdb-example")
            .tempdir()
            .unwrap();
        println!(
            "Database is put in: {} (maybe check if it was deleted)",
            dir.path().to_string_lossy()
        );
        let rocks_db = RocksDB::open(&config, &dir.path()).unwrap();
        let db = Db::<RocksDB>::new(rocks_db, 32);
        try_test_db(db);
    }
}
