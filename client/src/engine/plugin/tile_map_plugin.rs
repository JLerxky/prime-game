use common::tile_map::create_map;
use protocol::data::tile_map_data::{Slot, TileMap};
use std::collections::HashMap;

use bevy::prelude::*;

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
            map_size: UVec3::new(5, 5, 2),
            slot_map: HashMap::new(),
        })
        .add_startup_system(setup.system());
    }
}

#[test]
fn test() {
    debug!("{:?}", 1080 as i32 / 100i32);
    debug!("{:?}", 1920 as i32 / 100i32);
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
    debug!(
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

    debug!(
        "tile_size: {}; map_size: {:?}",
        tile_size, tile_map.map_size
    );

    create_map(&mut tile_map);

    // debug!("{:?}", &tile_map.slot_map);

    for (point, slot) in tile_map.slot_map.iter() {
        let x = point.x;
        let y = point.y;
        let pos_x = x as f32 * tile_size.x as f32 + center_pos.x;
        let pos_y = y as f32 * tile_size.y as f32 + center_pos.y;
        let tile_pos = Vec3::new(pos_x, pos_y, point.z as f32);
        // debug!("slot: ({},{}) pos: ({})", x, y, tile_pos);

        // let rigid_body = RigidBodyBuilder::new_static().translation(tile_pos.x, tile_pos.y);
        // let collider = ColliderBuilder::cuboid(tile_size.x / 2f32, tile_size.y / 2f32);

        if let Some(tile) = &slot.tile {
            let texture_handle = materials.add(
                asset_server
                    .load(format!("textures/prime/tiles/{}", tile.filename).as_str())
                    .into(),
            );

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
}
