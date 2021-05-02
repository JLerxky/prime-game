use std::collections::HashMap;

use glam::{IVec3, UVec3};
use serde::{Deserialize, Serialize};

// TileMap数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TileMapData {
    pub map_id: u128,
    pub tiles: Vec<TileState>,
}

// Tile数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TileData {
    pub point: (i32, i32, i32),
    pub tile: Option<Tile>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TileState {
    pub point: (i32, i32, i32),
    pub filename: String,
    pub collider: TileCollider,
}

// 地形碰撞体类型
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum TileCollider {
    None,
    Full,
    HalfTop,
    HalfBottom,
    HalfLeft,
    HalfRight,
    HalfFront,
    HalfBack,
    HalfCenter,
    HalfCenterX,
    HalfCenterY,
}

// 瓷砖
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct Tile {
    // 文件名作为name
    pub filename: String,
    // 层级
    pub layer: usize,
    // 随机权重
    pub rng_seed: u8,
    // 碰撞体类型
    pub collider: TileCollider,
    // 可连接点类型
    pub joints: [TileJoint; 6],
}

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub enum TileJoint {
    All,
    None,
    TagOne(String),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TileTag {
    pub id: u32,
    pub name: String,
}

// 位置
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Slot {
    // map坐标
    pub point: IVec3,
    // 叠加态（可选瓷砖集合）
    pub superposition: Vec<Tile>,
    // 熵 (superposition.len(), 等于0则已坍缩)
    pub entropy: usize,
    // 确定态（当前瓷砖）
    pub tile: Option<Tile>,
}

impl Slot {
    pub fn new(point: IVec3) -> Slot {
        let tiles = Vec::new();
        Slot {
            point,
            superposition: tiles.clone(),
            entropy: tiles.len(),
            tile: None,
        }
    }
}

#[derive(Clone, Debug)]
pub struct TileMap {
    pub center_point: IVec3,
    pub texture_size: UVec3,
    pub chunk_size: UVec3,
    pub map_size: UVec3,
    pub slot_map: HashMap<IVec3, Slot>,
}
