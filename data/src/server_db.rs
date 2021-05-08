use std::error::Error;

use protocol::data::{player_data::PlayerData, tile_map_data::TileState};

use crate::sled_db::SledDB;

#[derive(Debug)]
pub struct GameData {
    pub table: String,
    pub key: String,
    pub data: Option<String>,
}

static DB_PATH: &str = config::DB_PATH_SERVER;

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
    pub fn tile_map(point: glam::IVec3, data: Option<String>) -> Self {
        GameData {
            table: "tile_map".to_string(),
            key: format!("{},{},{}", point.x, point.y, point.z),
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

pub fn find_tile_map(point: (i32, i32, i32)) -> Result<TileState, Box<dyn Error>> {
    let db = &SledDB::open(DB_PATH)?.db;
    let data = &db.get(format!("tile_map-{:?}", point).as_bytes())?;
    if let Some(data) = data {
        Ok(bincode::deserialize(data)?)
    } else {
        Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::Other,
            "无数据!",
        )))
    }
}

pub fn save_tile_map(tile: TileState) -> Result<(), Box<dyn Error>> {
    let db = &SledDB::open(DB_PATH)?.db;
    let result = db.insert(
        format!("tile_map-{:?}", tile.point).as_bytes(),
        bincode::serialize(&tile)?,
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

pub fn find_all_tile() -> Vec<TileState> {
    let mut tiles = Vec::new();
    let db = &SledDB::open(DB_PATH).unwrap().db;
    for iter in db.scan_prefix("tile_map-") {
        match iter {
            Ok((_k, v)) => {
                if let Ok(t) = bincode::deserialize(&v) {
                    tiles.push(t);
                }
            }
            Err(e) => {
                println!("{}", e);
            }
        }
    }
    tiles
}

pub fn all_tile(path: &str) {
    let db = &SledDB::open(path).unwrap().db;
    for iter in db.scan_prefix("tile_map-") {
        match iter {
            Ok((k, v)) => {
                println!(
                    "{:?}==={:?}",
                    String::from_utf8(k.to_vec()).unwrap(),
                    String::from_utf8(v.to_vec()).unwrap()
                );
            }
            Err(e) => {
                println!("{}", e);
            }
        }
    }
}

pub fn save_player(player: PlayerData) -> Result<(), Box<dyn Error>> {
    let db = &SledDB::open(DB_PATH)?.db;
    let result = db.insert(
        format!("player-({})", player.uid).as_bytes(),
        bincode::serialize(&player)?,
    );
    match result {
        std::result::Result::Ok(_old) => {
            // println!("save: {}==={:?}", player.uid, player);
        }
        std::result::Result::Err(e) => {
            println!("error: {}", e);
        }
    }
    Ok(())
}

pub fn find_player(uid: u32) -> Result<PlayerData, Box<dyn Error>> {
    let db = &SledDB::open(DB_PATH)?.db;
    let data = &db.get(format!("player-({})", uid).as_bytes())?;
    if let Some(data) = data {
        Ok(bincode::deserialize(data)?)
    } else {
        Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::Other,
            "无数据!",
        )))
    }
}

pub fn next_entity_id(entity_type: u8) -> Result<u64, Box<dyn Error>> {
    let db = &SledDB::open(DB_PATH)?.db;
    let data_bt = &db.get(format!("entity-id-({})", entity_type).as_bytes())?;
    if let Some(data) = data_bt {
        let data_str = String::from_utf8(data.to_vec())?;
        if let Ok(mut next) = data_str.parse::<u64>() {
            next += 1;
            let _ = db.insert(
                format!("entity-id-({})", entity_type).as_bytes(),
                format!("{}", next).as_bytes().to_vec(),
            )?;
            return Ok(next);
        }
    }
    let _ = db.insert(
        format!("entity-id-({})", entity_type).as_bytes(),
        "0".as_bytes().to_vec(),
    )?;
    Ok(0)
}
