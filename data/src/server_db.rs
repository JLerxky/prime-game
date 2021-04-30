use std::error::Error;

use crate::sled_db::SledDB;

#[derive(Debug)]
pub struct GameData {
    pub table: String,
    pub key: String,
    pub data: Option<String>,
}

static DB_PATH: &str = "db_sled_server";

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
    let db = &SledDB::open(DB_PATH)?.db;
    let data = &db.get(format!("{}-({})", key.table, key.key).as_bytes())?;
    if let Some(data) = data {
        Ok(String::from_utf8(data.to_vec())?)
    } else {
        Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::Other,
            "无数据!",
        )))
    }
}

pub fn next_u64(key: GameData) -> Result<u64, Box<dyn Error>> {
    let db = &SledDB::open(DB_PATH)?.db;
    let data_bt = &db.get(format!("{}-({})", key.table, key.key).as_bytes())?;
    if let Some(data) = data_bt {
        let data_str = String::from_utf8(data.to_vec())?;
        if let Ok(mut next) = data_str.parse::<u64>() {
            next += 1;
            save(GameData::player_queue_uid(Some(format!("{}", next))))?;
            return Ok(next);
        }
    }
    save(GameData::player_queue_uid(Some(format!("{}", 0))))?;
    Ok(0)
}

pub fn save(data: GameData) -> Result<(), Box<dyn Error>> {
    let db = &SledDB::open(DB_PATH)?.db;
    let result = db.insert(
        format!("{}-({})", data.table, data.key).as_bytes(),
        data.data.unwrap().as_bytes(),
    );
    match result {
        std::result::Result::Ok(_old) => {
            // println!("old: {:?}", old);
        }
        std::result::Result::Err(e) => {
            println!("error: {}", e);
        }
    }
    Ok(())
}

#[test]
fn test_server_db() {
    let _ = save(GameData::player_addr_uid(
        "0:0:0:0".to_string(),
        Some("999".to_string()),
    ));
    match find(GameData::player_addr_uid("0:0:0:0".to_string(), None)) {
        Ok(data) => {
            println!("{}", data);
        }
        Err(_) => {}
    }
}
