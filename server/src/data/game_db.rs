use std::error::Error;

use crate::data::rocksdb::RocksDB;

pub struct GameData {
    pub table: String,
    pub key: String,
    pub data: Option<String>,
}

impl GameData {
    pub fn player_online(data: Option<String>) -> Self {
        GameData {
            table: "player".to_string(),
            key: "online".to_string(),
            data,
        }
    }
    pub fn player_group_addr(group: u32, data: Option<String>) -> Self {
        GameData {
            table: "player".to_string(),
            key: format!("group_addr_{}", group),
            data,
        }
    }
}

pub fn find(key: GameData) -> Result<String, Box<dyn Error>> {
    let rocks_db = RocksDB::open()?;
    match rocks_db.get_value(format!("{}-({})", key.table, key.key)) {
        Some(result) => Ok(result),
        None => Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::Other,
            "无数据!",
        ))),
    }
}

pub fn save(data: GameData) -> Result<(), Box<dyn Error>> {
    let rocks_db = RocksDB::open()?;
    rocks_db.put_value(format!("{}-({})", data.table, data.key), data.data.unwrap())
}

pub fn find_and_lock(key: GameData) -> Result<String, Box<dyn Error>> {
    let rocks_db = RocksDB::open()?;
    match rocks_db.get_value(format!("{}-({})", key.table, key.key)) {
        Some(result) => Ok(result),
        None => Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::Other,
            "oh no!",
        ))),
    }
}
