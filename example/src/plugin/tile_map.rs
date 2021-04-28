use protocol::data::tile_map_data::TileCollider;
use rand::Rng;
use std::collections::HashMap;

use bevy::prelude::*;

use crate::data::load_default_superposition;

// 瓷砖
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Tile {
    // 文件名作为name
    pub filename: String,
    // 层级
    pub layer: usize,
    // 碰撞体类型
    pub collider: TileCollider,
    // 可连接点类型
    pub joints: [TileJoint; 6],
}

impl Tile {}

#[derive(Clone, Debug, PartialEq, Eq)]
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
    center_point: IVec3,
    texture_size: UVec3,
    chunk_size: UVec3,
    map_size: UVec3,
    slot_map: HashMap<IVec3, Slot>,
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
            center_point: IVec3::new(0, 0, 0),
            texture_size: UVec3::new(64, 64, 1),
            chunk_size: UVec3::new(1, 1, 1),
            map_size: UVec3::new(5, 5, 1),
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
    // 根据窗口大小修改tile_map大小
    let mut x = (window.width / tile_size.x as f32) as u32 + 2;
    let mut y = (window.height / tile_size.y as f32) as u32 + 2;
    x += ((x % 2) == 0) as u32;
    y += ((y % 2) == 0) as u32;
    tile_map.map_size = UVec3::new(x, y, tile_map.map_size.z);

    let center_pos = tile_map.center_point.as_f32()
        * tile_map.texture_size.as_f32()
        * tile_map.chunk_size.as_f32();

    println!(
        "tile_size: {}; map_size: {:?}",
        tile_size, tile_map.map_size
    );

    create_map(&mut tile_map);

    // println!("{:?}", &tile_map.slot_map);

    for (point, slot) in tile_map.slot_map.iter() {
        let x = point.x;
        let y = point.y;
        let pos_x = x as f32 * tile_size.x as f32 + center_pos.x;
        let pos_y = y as f32 * tile_size.y as f32 + center_pos.y;
        let tile_pos = Vec3::new(pos_x, pos_y, point.z as f32);
        // println!("slot: ({},{}) pos: ({})", x, y, tile_pos);

        let mut texture_handle = materials.add(
            asset_server
                .load("textures/prime/tiles/0-tileset_50.png")
                .into(),
        );
        if let Some(tile) = &slot.tile {
            texture_handle = materials.add(
                asset_server
                    .load(format!("textures/prime/tiles/{}", tile.filename).as_str())
                    .into(),
            );
        }

        // let rigid_body = RigidBodyBuilder::new_static().translation(tile_pos.x, tile_pos.y);
        // let collider = ColliderBuilder::cuboid(tile_size.x / 2f32, tile_size.y / 2f32);

        commands
            .spawn_bundle(SpriteBundle {
                material: texture_handle.clone(),
                sprite: Sprite::new(tile_size.truncate().as_f32()),
                transform: Transform::from_translation(tile_pos),
                ..Default::default()
            })
            // .insert(rigid_body)
            // .insert(collider.friction(0.0))
            .insert(Slot {
                superposition: Vec::new(),
                entropy: 0,
                tile: None,
                point: tile_pos.as_i32(),
            });
    }
}

fn create_map(tile_map: &mut TileMap) {
    // 1. 计算地图边界值

    let min_x = tile_map.center_point.x - (tile_map.map_size.x as i32 / 2);
    let max_x = tile_map.center_point.x + (tile_map.map_size.x as i32 / 2);
    let min_y = tile_map.center_point.y - (tile_map.map_size.y as i32 / 2);
    let max_y = tile_map.center_point.y + (tile_map.map_size.y as i32 / 2);

    // 2. 按Z轴从小到大生成图层
    for z in 0..tile_map.map_size.z {
        for point_x in min_x..=max_x {
            for point_y in min_y..=max_y {
                let point = IVec3::new(point_x, point_y, z as i32);

                // 判断是否已初始化
                if tile_map.slot_map.contains_key(&point) {
                    continue;
                }

                // 初始化Slot: 填充叠加态, 初始化熵
                let superposition = load_default_superposition();
                let entropy = superposition.len();
                let slot = Slot {
                    point,
                    superposition,
                    entropy,
                    tile: None,
                };
                tile_map.slot_map.insert(point, slot);
            }
        }
    }
    // 4-2. 按照熵值从小到大坍缩
    // TODO 4-2-1. 填充当前地图块四周已坍缩的tile，以供计算边缘slot的叠加态与熵
    // 4-2-2. 递归坍缩
    tile_map.slot_map = collapse(tile_map.slot_map.clone());
}

// 世界坐标->地图索引
fn pos_to_global_point(tile_map: &TileMap, pos: Vec3) -> IVec3 {
    let point =
        pos / ((tile_map.chunk_size * tile_map.texture_size).as_f32() / Vec3::new(2., 2., 1.));
    point.as_i32()
}

// 递归坍缩
fn collapse(mut slot_map: HashMap<IVec3, Slot>) -> HashMap<IVec3, Slot> {
    let mut slot_list: Vec<Slot> = Vec::new();

    // 取出当前所有未坍缩的slot
    for (_point, slot) in slot_map.iter() {
        if slot.entropy > 0 {
            slot_list.push(slot.clone());
        }
    }

    // 重新计算熵
    println!("{:?}", &slot_list.len());
    for slot in &mut slot_list {
        let superposition: Vec<Tile> = slot.superposition.clone();

        // 取得紧贴的slot连接限制条件tile_joint
        let mut joint_list = [
            TileJoint::All, // 0上
            TileJoint::All, // 1下
            TileJoint::All, // 2左
            TileJoint::All, // 3右
            TileJoint::All, // 4前
            TileJoint::All, // 5后
        ];
        // 上
        let point = IVec3::new(slot.point.x, slot.point.y + 1, slot.point.z);
        if let Some(slot) = slot_map.get(&point) {
            if let Some(tile) = &slot.tile {
                joint_list[0] = tile.joints[1].clone();
            }
        }
        // 下
        let point = IVec3::new(slot.point.x, slot.point.y - 1, slot.point.z);
        if let Some(slot) = slot_map.get(&point) {
            if let Some(tile) = &slot.tile {
                joint_list[1] = tile.joints[0].clone();
            }
        }
        // 左
        let point = IVec3::new(slot.point.x - 1, slot.point.y, slot.point.z);
        if let Some(slot) = slot_map.get(&point) {
            if let Some(tile) = &slot.tile {
                joint_list[2] = tile.joints[3].clone();
            }
        }
        // 右
        let point = IVec3::new(slot.point.x + 1, slot.point.y, slot.point.z);
        if let Some(slot) = slot_map.get(&point) {
            if let Some(tile) = &slot.tile {
                joint_list[3] = tile.joints[2].clone();
            }
        }
        // 前
        let point = IVec3::new(slot.point.x, slot.point.y, slot.point.z + 1);
        if let Some(slot) = slot_map.get(&point) {
            if let Some(tile) = &slot.tile {
                joint_list[4] = tile.joints[5].clone();
            }
        }
        // 后
        let point = IVec3::new(slot.point.x, slot.point.y, slot.point.z - 1);
        if let Some(slot) = slot_map.get(&point) {
            if let Some(tile) = &slot.tile {
                joint_list[5] = tile.joints[4].clone();
            }
        }

        // 剔除无效坍缩态
        let mut superposition_new = Vec::new();
        'tile: for tile in superposition.iter() {
            println!("-------------------{:?}", &tile.filename);
            for i in 0..6 as usize {
                match joint_list[i] {
                    TileJoint::None => {
                        superposition_new = Vec::new();
                        break 'tile;
                    }
                    TileJoint::TagOne(ref tag) => {
                        match &tile.joints[i] {
                            TileJoint::All => {}
                            TileJoint::None => {
                                continue 'tile;
                            }
                            TileJoint::TagOne(t_tage) => {
                                println!("{}:{:?}---{:?}:{}", i, tag, t_tage, i);
                                if !t_tage.eq(tag) {
                                    continue 'tile;
                                }
                            }
                        }
                        // continue 'tile;
                    }
                    _ => {}
                }
            }
            superposition_new.push(tile.clone());
        }

        // 更新slot
        slot.superposition = superposition_new;
        slot.entropy = slot.superposition.len();

        println!("{:?}", joint_list);
        println! {"{:?}-{:?}", &slot.point, &slot.entropy};
    }

    // 获取最小熵slot
    let mut min_entropy = usize::MAX;
    let mut min_slot = None;
    for slot in slot_list {
        if slot.entropy == 0 {
            slot_map.insert(slot.point, slot.clone());
            continue;
        }
        if slot.entropy < min_entropy && slot.entropy != 0 {
            min_entropy = slot.entropy;
            min_slot = Some(slot);
        }
    }

    // 执行slot坍缩
    if let Some(mut slot) = min_slot {
        let i = rand::thread_rng().gen_range(0..slot.superposition.len());

        slot.tile = Some(slot.superposition[i].clone());
        slot.superposition = Vec::new();
        slot.entropy = 0;
        if let Some(_slot) = slot_map.insert(slot.point, slot.clone()) {
            println!("更新{:?}", slot);
        }
        // 判断是否完成坍缩, 完成则退出递归返回tile_map结果, 否则继续
        return collapse(slot_map);
    } else {
        return slot_map;
    }
}

#[test]
fn test_create_map() {
    let mut tile_map = TileMap {
        center_point: IVec3::new(0, 0, 0),
        texture_size: UVec3::new(64, 64, 1),
        chunk_size: UVec3::new(1, 1, 1),
        map_size: UVec3::new(5, 5, 1),
        slot_map: HashMap::new(),
    };
    create_map(&mut tile_map);
    println!("{:?}", tile_map);
}
