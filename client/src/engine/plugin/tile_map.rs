use std::collections::HashMap;

use bevy::prelude::*;

// 瓷砖
#[derive(Clone, Debug)]
pub struct Tile {
    // 文件名作为name
    pub filename: String,
    // 层级
    pub layer: usize,
    // 标签
    pub tags: Vec<TileTag>,
    // 可连接点类型
    pub joints: (
        TileJoint, // 0上
        TileJoint, // 1下
        TileJoint, // 2左
        TileJoint, // 3右
        TileJoint, // 4前
        TileJoint, // 6后
    ),
}

impl Tile {}

#[derive(Clone, Debug)]
pub enum TileJoint {
    All,
    None,
    One(String),
    Some(String),
    TagOne(TileTag),
    TagSome(Vec<TileTag>),
}

#[derive(Clone, Debug)]
pub struct TileTag {
    pub id: u32,
    pub name: String,
}

// 位置
#[derive(Clone, Debug)]
pub struct Slot {
    // map坐标
    pub point: Vec3,
    // 叠加态（可选瓷砖集合）
    pub superposition: Vec<Tile>,
    // 熵 (superposition.len(), 等于0则已坍缩)
    pub entropy: usize,
    // 确定态（当前瓷砖）
    pub tile: Option<Tile>,
}

impl Slot {
    pub fn new(point: Vec3) -> Slot {
        let tiles = Vec::new();
        Slot {
            point,
            superposition: tiles.clone(),
            entropy: tiles.len(),
            tile: None,
        }
    }
}

pub struct TileMap {
    tile_center: Vec3,
    texture_size: Vec2,
    chunk_size: Vec2,
    layers: usize,
    slot_map: HashMap<Vec3, Slot>,
}

impl TileMap {
    pub fn position_to_slot(&self, position: Vec2) -> (usize, usize) {
        (0, 0)
    }
}

pub struct TileMapPlugin;

#[derive(Debug, Hash, PartialEq, Eq, Clone, StageLabel)]
struct BuildMapFixedUpdateStage;

#[derive(Debug, Hash, PartialEq, Eq, Clone, StageLabel)]
struct CleanMapFixedUpdateStage;

impl Plugin for TileMapPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.insert_resource(TileMap {
            tile_center: Vec3::new(0f32, 0f32, 0f32),
            texture_size: Vec2::new(0f32, 0f32),
            chunk_size: Vec2::new(0f32, 0f32),
            layers: 10,
            slot_map: HashMap::new(),
        });
    }
}

#[test]
fn test() {
    println!("{:?}", 1080 as i32 / 100i32);
    println!("{:?}", 1920 as i32 / 100i32);
}

// 1. 获取tile素材资源, 生成tile可用集合
// 2. 按所需创建地图大小生成tilemap, 创建slot_map
// 3. slot坍缩, 生成新地图
// 4. 将新地图持久化
// 5. 玩家到达地图位置时提供已生成的地图数据