use rocksdb::Error;

use crate::data::rocksdb::RocksDB;

pub struct GameData {
    pub table: String,
    pub key: String,
    pub data: Option<String>,
}

pub fn find(key: GameData) -> Option<String> {
    let rocks_db = RocksDB::open();
    match rocks_db.get_value(format!("{}-({})", key.table, key.key)) {
        Some(result) => Some(result),
        None => None,
    }
}

pub fn save(data: GameData) -> Result<(), Error> {
    let rocks_db = RocksDB::open();
    rocks_db.put_value(format!("{}-({})", data.table, data.key), data.data.unwrap())
}

pub fn find_and_lock(key: GameData) -> Option<String> {
    let rocks_db = RocksDB::open();
    match rocks_db.get_value(format!("{}-({})", key.table, key.key)) {
        Some(result) => Some(result),
        None => None,
    }
}
