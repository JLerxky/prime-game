use crate::rocksdb::{ColumnFamily, RocksDB};
use std::error::Error;

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
    pub fn player_addr_health(addr: String, data: Option<String>) -> Self {
        GameData {
            table: "player".to_string(),
            key: format!("addr_health_{}", addr),
            data,
        }
    }
    pub fn player_addr_uid(addr: String, data: Option<String>) -> Self {
        GameData {
            table: "player".to_string(),
            key: format!("addr_uid_{}", addr),
            data,
        }
    }
    pub fn player_queue_uid(data: Option<String>) -> Self {
        GameData {
            table: "player".to_string(),
            key: "queue_uid".to_string(),
            data,
        }
    }
}

pub fn find(key: GameData) -> Result<String, Box<dyn Error>> {
    let rocks_db = RocksDB::open(ColumnFamily::GameServer)?;
    match rocks_db.get_value(format!("{}-({})", key.table, key.key)) {
        Some(result) => Ok(result),
        None => Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::Other,
            "无数据!",
        ))),
    }
}

pub fn next_u64(key: GameData) -> Result<u64, Box<dyn Error>> {
    let rocks_db = RocksDB::open(ColumnFamily::GameServer)?;
    match rocks_db.get_value(format!("{}-({})", key.table, key.key)) {
        Some(result) => {
            if let Ok(mut next) = result.parse::<u64>() {
                next += 1;
                save(GameData::player_queue_uid(Some(format!("{}", next))))?;
                return Ok(next);
            }
        }
        None => {}
    }
    save(GameData::player_queue_uid(Some(format!("{}", 0))))?;
    Ok(0)
}

pub fn save(data: GameData) -> Result<(), Box<dyn Error>> {
    let rocks_db = RocksDB::open(ColumnFamily::GameServer)?;
    rocks_db.put_value(format!("{}-({})", data.table, data.key), data.data.unwrap())
}

pub fn find_and_lock(key: GameData) -> Result<String, Box<dyn Error>> {
    let rocks_db = RocksDB::open(ColumnFamily::GameServer)?;
    match rocks_db.get_value(format!("{}-({})", key.table, key.key)) {
        Some(result) => Ok(result),
        None => Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::Other,
            "oh no!",
        ))),
    }
}
