use bevy_rapier2d::rapier::dynamics::RigidBodyBuilder;
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
    // 碰撞体类型
    pub collider: TileCollider,
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

// 地形碰撞体类型
#[derive(Clone, Debug)]
pub enum TileCollider {
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
    texture_size: Vec3,
    chunk_size: Vec3,
    map_size: Vec3,
    slot_map: HashMap<Vec3, Slot>,
}

impl TileMap {
    pub fn position_to_slot(&self, position: Vec3) -> Vec3 {
        position
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
            texture_size: Vec3::new(64f32, 64f32, 0f32),
            chunk_size: Vec3::new(1f32, 1f32, 0f32),
            map_size: Vec3::new(0f32, 0f32, 0f32),
            slot_map: HashMap::new(),
        })
        .add_startup_system(setup.system());
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

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut tile_map: ResMut<TileMap>,
    window: Res<WindowDescriptor>,
) {
    println!(
        "窗口大小: {},{}; 瓷砖大小: {:?}",
        window.width,
        window.height,
        tile_map.texture_size * tile_map.chunk_size
    );

    // 计算tile_map大小
    let tile_size = tile_map.texture_size * tile_map.chunk_size;
    let mut x = (window.width / tile_size.x) as u32 + 2;
    let mut y = (window.height / tile_size.y) as u32 + 2;
    x += ((x % 2) == 0) as u32;
    y += ((y % 2) == 0) as u32;
    tile_map.map_size = Vec3::new(x as f32, y as f32, 10f32);

    println!(
        "tile_size: {}; map_size: {:?}",
        tile_size, tile_map.map_size
    );

    for x in -tile_map.map_size.x as i32 / 2..=tile_map.map_size.x as i32 / 2 {
        let pos_x = x as f32 * tile_size.x + tile_map.tile_center.x;
        for y in -tile_map.map_size.y as i32 / 2..=tile_map.map_size.y as i32 / 2 {
            let pos_y = y as f32 * tile_size.y + tile_map.tile_center.y;
            let tile_pos = Vec3::new(pos_x, pos_y, -5f32);
            println!("slot: ({},{}) pos: ({})", x, y, tile_pos);

            let texture_handle =
                materials.add(asset_server.load("textures/tiles/10_-3_1.png").into());

            let rigid_body = RigidBodyBuilder::new_static().translation(tile_pos.x, tile_pos.y);
            // let collider = ColliderBuilder::cuboid(tile_size.x / 2f32, tile_size.y / 2f32);

            commands
                .spawn_bundle(SpriteBundle {
                    material: texture_handle.clone(),
                    sprite: Sprite::new(tile_size.truncate()),
                    transform: Transform::from_translation(tile_pos),
                    ..Default::default()
                })
                .insert(rigid_body)
                // .insert(collider.friction(0.0))
                .insert(Slot {
                    superposition: Vec::new(),
                    entropy: 0,
                    tile: None,
                    point: Vec3::from(tile_pos),
                });
        }
    }
}
