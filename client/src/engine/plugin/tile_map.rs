use bevy_rapier2d::rapier::dynamics::RigidBodyBuilder;
use protocol::data::tile_map_data::TileCollider;
use rand::Rng;
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
        TileJoint, // 5后
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

#[derive(Clone, Debug, PartialEq)]
pub struct TileTag {
    pub id: u32,
    pub name: String,
}

// 位置
#[derive(Clone, Debug)]
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
            texture_size: UVec3::new(64, 64, 0),
            chunk_size: UVec3::new(1, 1, 0),
            map_size: UVec3::new(5, 5, 3),
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
    let mut x = (window.width / tile_size.x as f32) as u32 + 2;
    let mut y = (window.height / tile_size.y as f32) as u32 + 2;
    x += ((x % 2) == 0) as u32;
    y += ((y % 2) == 0) as u32;
    tile_map.map_size = UVec3::new(x, y, 10);

    let center_pos = tile_map.center_point.as_f32()
        * tile_map.texture_size.as_f32()
        * tile_map.chunk_size.as_f32();

    println!(
        "tile_size: {}; map_size: {:?}",
        tile_size, tile_map.map_size
    );

    for x in -(tile_map.map_size.x as i32) / 2..=tile_map.map_size.x as i32 / 2 {
        let pos_x = x as f32 * tile_size.x as f32 + center_pos.x;
        for y in -(tile_map.map_size.y as i32) / 2..=tile_map.map_size.y as i32 / 2 {
            let pos_y = y as f32 * tile_size.y as f32 + center_pos.y;
            let tile_pos = Vec3::new(pos_x, pos_y, -5f32);
            println!("slot: ({},{}) pos: ({})", x, y, tile_pos);

            let texture_handle = materials.add(
                asset_server
                    .load("textures/prime/tiles/0-tileset_04.png")
                    .into(),
            );

            let rigid_body = RigidBodyBuilder::new_static().translation(tile_pos.x, tile_pos.y);
            // let collider = ColliderBuilder::cuboid(tile_size.x / 2f32, tile_size.y / 2f32);

            commands
                .spawn_bundle(SpriteBundle {
                    material: texture_handle.clone(),
                    sprite: Sprite::new(tile_size.truncate().as_f32()),
                    transform: Transform::from_translation(tile_pos),
                    ..Default::default()
                })
                .insert(rigid_body)
                // .insert(collider.friction(0.0))
                .insert(Slot {
                    superposition: Vec::new(),
                    entropy: 0,
                    tile: None,
                    point: tile_pos.as_i32(),
                });
        }
    }
}

fn create_map(tile_map: &mut TileMap, player_pos: Vec3) {
    // 1. 按玩家进入点计算初始位置、地图边界值
    let initial_pos = player_pos;
    let initial_point = pos_to_global_point(tile_map, initial_pos);

    let min_x = tile_map.center_point.x - (tile_map.map_size.max_element() as i32 / 2);
    let max_x = tile_map.center_point.x + (tile_map.map_size.max_element() as i32 / 2);
    let min_y = tile_map.center_point.y - (tile_map.map_size.max_element() as i32 / 2);
    let max_y = tile_map.center_point.y + (tile_map.map_size.max_element() as i32 / 2);

    // 2. 按Z轴从小到大生成图层
    for z in 0..tile_map.map_size.z {
        // 3. 按与初始点位的距离，一圈一圈生成
        for step in 0..tile_map.map_size.max_element() {
            // 4. 同圈内按照熵值从小到大生成
            // 4-1. 初始化Slot并计算熵
            for step_x in -(step as i32)..=(step as i32) {
                let point_x = initial_point.x + step_x as i32;
                // 判断x是否超过范围
                if point_x < min_x || point_x > max_x {
                    continue;
                }
                for step_y in -(step as i32)..=(step as i32) {
                    let point_y = initial_point.y + step_y as i32;
                    // 判断x是否超过范围
                    if point_y < min_y || point_y > max_y {
                        continue;
                    }
                    let point = IVec3::new(point_x, point_y, z as i32);

                    // 判断是否已初始化
                    if tile_map.slot_map.contains_key(&point) {
                        continue;
                    }
                    // TODO 测试输出
                    // println!("{}", point);

                    // 初始化Slot: 填充叠加态, 初始化熵
                    let slot = Slot {
                        point,
                        superposition: load_default_superposition(),
                        entropy: 999,
                        tile: None,
                    };
                    tile_map.slot_map.insert(point, slot);
                }
            }
            // 4-2. 按照熵值从小到大坍缩
            // TODO 4-2-1. 填充当前地图块四周已坍缩的tile，以供计算边缘slot的叠加态与熵
            // 4-2-2. 递归坍缩
            *tile_map = collapse(tile_map.clone());
        }
    }
}

// 世界坐标->地图索引
fn pos_to_global_point(tile_map: &TileMap, pos: Vec3) -> IVec3 {
    let point =
        pos / ((tile_map.chunk_size * tile_map.texture_size).as_f32() / Vec3::new(2., 2., 1.));
    point.as_i32()
}

// 递归坍缩
fn collapse(mut tile_map: TileMap) -> TileMap {
    let mut slot_list: Vec<Slot> = Vec::new();
    let slot_map = tile_map.slot_map.clone();

    // 取出当前所有未坍缩的slot
    for (_point, slot) in slot_map.iter() {
        if slot.entropy == 0 {
            slot_list.push(slot.clone());
        }
    }

    // 重新计算熵
    for slot in &mut slot_list {
        let mut superposition: Vec<Tile> = slot.superposition.clone();

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
                joint_list[0] = tile.joints.1.clone();
            }
        }
        // 下
        let point = IVec3::new(slot.point.x, slot.point.y - 1, slot.point.z);
        if let Some(slot) = slot_map.get(&point) {
            if let Some(tile) = &slot.tile {
                joint_list[1] = tile.joints.0.clone();
            }
        }
        // 左
        let point = IVec3::new(slot.point.x - 1, slot.point.y, slot.point.z);
        if let Some(slot) = slot_map.get(&point) {
            if let Some(tile) = &slot.tile {
                joint_list[2] = tile.joints.3.clone();
            }
        }
        // 右
        let point = IVec3::new(slot.point.x + 1, slot.point.y, slot.point.z);
        if let Some(slot) = slot_map.get(&point) {
            if let Some(tile) = &slot.tile {
                joint_list[3] = tile.joints.2.clone();
            }
        }
        // 前
        let point = IVec3::new(slot.point.x, slot.point.y, slot.point.z + 1);
        if let Some(slot) = slot_map.get(&point) {
            if let Some(tile) = &slot.tile {
                joint_list[4] = tile.joints.5.clone();
            }
        }
        // 后
        let point = IVec3::new(slot.point.x, slot.point.y, slot.point.z - 1);
        if let Some(slot) = slot_map.get(&point) {
            if let Some(tile) = &slot.tile {
                joint_list[5] = tile.joints.4.clone();
            }
        }

        // 剔除无效坍缩态
        'tile: for tile_i in 0..superposition.len() {
            let tile = superposition[tile_i].clone();
            for i in 0..6 as usize {
                match joint_list[i] {
                    TileJoint::None => {
                        superposition = Vec::new();
                        break 'tile;
                    }
                    TileJoint::One(ref filename) => {
                        if !filename.eq(&tile.filename) {
                            &superposition.remove(tile_i);
                            continue 'tile;
                        }
                    }
                    TileJoint::Some(ref filename_list_str) => {
                        let filename_list: Vec<&str> = filename_list_str.split(",").collect();
                        if !filename_list.contains(&tile.filename.as_str()) {
                            &superposition.remove(tile_i);
                            continue 'tile;
                        }
                    }
                    TileJoint::TagOne(ref tag) => {
                        if !tile.tags.contains(&tag) {
                            &superposition.remove(tile_i);
                            continue 'tile;
                        }
                    }
                    TileJoint::TagSome(ref tag_list) => {
                        let mut skip = true;
                        'tag: for t0 in tag_list {
                            for t1 in &tile.tags {
                                if t0.eq(&t1) {
                                    skip = false;
                                    break 'tag;
                                }
                            }
                        }
                        if skip {
                            &superposition.remove(tile_i);
                            continue 'tile;
                        }
                    }
                    _ => {}
                }
            }
        }

        // 更新slot
        slot.superposition = superposition;
        slot.entropy = slot.superposition.len();
    }

    // 获取最小熵slot
    let mut min_entropy = usize::MAX;
    let mut min_slot = None;
    for slot in slot_list {
        if slot.entropy == 0 {
            tile_map.slot_map.insert(slot.point, slot.clone());
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
        tile_map.slot_map.insert(slot.point, slot.clone());
    }

    // 判断是否完成坍缩, 完成则退出递归返回tile_map结果, 否则继续
    let result = tile_map;
    let mut complete = true;
    for (_, slot) in result.slot_map.iter() {
        if slot.entropy > 0 {
            complete = false;
            break;
        }
    }
    if complete {
        return result;
    } else {
        return collapse(result);
    }
}

// TODO 加载默认可用tile作为叠加态
fn load_default_superposition() -> Vec<Tile> {
    let superposition = Vec::new();
    superposition
}

#[test]
fn test_create_map() {
    let mut tile_map = TileMap {
        center_point: IVec3::new(0, 0, 0),
        texture_size: UVec3::new(64, 64, 1),
        chunk_size: UVec3::new(1, 1, 1),
        map_size: UVec3::new(5, 5, 3),
        slot_map: HashMap::new(),
    };
    let pos = Vec3::new(64., -32., 0.);
    create_map(&mut tile_map, pos);
    println!(
        "{:?}",
        tile_map
    );
}
