use std::error::Error;

use protocol::data::{player_data::PlayerData, tile_map_data::TileState};

use crate::sled_db::SledDB;

#[derive(Debug)]
pub struct GameData {
    pub table: String,
    pub key: String,
    pub data: Option<String>,
}

static DB_PATH: &str = config::DB_PATH_CLIENT;

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

pub fn find_tile_map(point: glam::IVec3) -> Result<TileState, Box<dyn Error>> {
    let db = &SledDB::open(DB_PATH)?.db;
    let key = GameData {
        table: "tile_map".to_string(),
        key: format!("{},{},{}", point.x, point.y, point.z),
        data: None,
    };
    let data = &db.get(format!("{}-({})", key.table, key.key).as_bytes())?;
    if let Some(data) = data {
        Ok(bincode::deserialize(data)?)
    } else {
        Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::Other,
            "无数据!",
        )))
    }
}

pub fn save_tile_map(point: glam::IVec3, tile: TileState) -> Result<(), Box<dyn Error>> {
    let db = &SledDB::open(DB_PATH)?.db;
    let data = GameData {
        table: "tile_map".to_string(),
        key: format!("{},{},{}", point.x, point.y, point.z),
        data: None,
    };
    let result = db.insert(
        format!("{}-({})", data.table, data.key).as_bytes(),
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

#[test]
fn test_server_db() {
    let point = glam::IVec3::new(0, 0, 0);
    if let Ok(tile) = find_tile_map(point) {
        println!("{:?}", tile);
    }
}

#[test]
fn test_iter() {
    SledDB::show_all(&format!("../{}", config::DB_PATH_CLIENT));
}
