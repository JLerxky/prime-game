use protocol::{
    data::tile_map_data::{Slot, Tile, TileData, TileMap},
    packet::Packet,
    route::GameRoute,
};
use std::collections::HashMap;

use bevy::prelude::*;

use super::network_plugin::NetWorkState;

pub struct TileMapPlugin;

#[derive(Debug, Hash, PartialEq, Eq, Clone, StageLabel)]
struct UpdateTileFixedUpdateStage;

#[derive(Debug, Hash, PartialEq, Eq, Clone, StageLabel)]
struct CleanMapFixedUpdateStage;

impl Plugin for TileMapPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.insert_resource(TileMap {
            center_point: IVec3::new(0, 0, 0),
            texture_size: UVec3::new(64, 64, 1),
            chunk_size: UVec3::new(1, 1, 1),
            map_size: UVec3::new(30, 30, 2),
            slot_map: HashMap::new(),
        })
        .add_startup_system(setup.system());
        // .add_stage_after(
        //     CoreStage::Update,
        //     UpdateTileFixedUpdateStage,
        //     SystemStage::parallel()
        //         .with_run_criteria(
        //             bevy::core::FixedTimestep::step(0.5).with_label("update_tile_fixed_timestep"),
        //         )
        //         .with_system(update_tile.system()),
        // );
    }
}

#[test]
fn test() {
    debug!("{:?}", 1080 as i32 / 100i32);
    debug!("{:?}", 1920 as i32 / 100i32);
}

fn get_tile(point: IVec3, net_state: &ResMut<NetWorkState>) -> Option<Tile> {
    if let Ok(tile) = data::client_db::find_tile_map(point) {
        return Some(tile);
    } else {
        if let Ok(mut to_be_sent_queue) = net_state.to_be_sent_queue.lock() {
            to_be_sent_queue.push(Packet::Game(GameRoute::Tile(TileData {
                point: (point.x, point.y, point.z),
                tile: None,
            })));
        }
    }
    None
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
    net_state: ResMut<NetWorkState>,
) {
    debug!(
        "窗口大小: {},{}; 瓷砖大小: {:?}",
        window.width,
        window.height,
        tile_map.texture_size * tile_map.chunk_size
    );

    // 计算tile_size大小
    let tile_size = tile_map.texture_size * tile_map.chunk_size;
    // 根据窗口大小修改tile_map大小
    // let mut x = (window.width / tile_size.x as f32) as u32 + 2;
    // let mut y = (window.height / tile_size.y as f32) as u32 + 2;
    // x += ((x % 2) == 0) as u32;
    // y += ((y % 2) == 0) as u32;
    // tile_map.map_size = UVec3::new(x, y, tile_map.map_size.z);

    let center_pos = tile_map.center_point.as_f32()
        * tile_map.texture_size.as_f32()
        * tile_map.chunk_size.as_f32();

    debug!(
        "tile_size: {}; map_size: {:?}",
        tile_size, tile_map.map_size
    );

    // debug!("{:?}", &tile_map.slot_map);

    let min_x = tile_map.center_point.x - (tile_map.map_size.x as i32 / 2);
    let max_x = tile_map.center_point.x + (tile_map.map_size.x as i32 / 2);
    let min_y = tile_map.center_point.y - (tile_map.map_size.y as i32 / 2);
    let max_y = tile_map.center_point.y + (tile_map.map_size.y as i32 / 2);

    // 2. 按Z轴从小到大生成图层
    for z in 0..tile_map.map_size.z {
        for x in min_x..=max_x {
            for y in min_y..=max_y {
                let point = IVec3::new(x as i32, y as i32, z as i32);
                let pos_x = x as f32 * tile_size.x as f32 + center_pos.x;
                let pos_y = y as f32 * tile_size.y as f32 + center_pos.y;
                let tile_pos = Vec3::new(pos_x, pos_y, z as f32);
                // debug!("slot: ({},{}) pos: ({})", x, y, tile_pos);

                // let rigid_body = RigidBodyBuilder::new_static().translation(tile_pos.x, tile_pos.y);
                // let collider = ColliderBuilder::cuboid(tile_size.x / 2f32, tile_size.y / 2f32);
                // println!("point: {}", point);
                if let Some(tile) = get_tile(point, &net_state) {
                    tile_map.slot_map.insert(
                        point.clone(),
                        Slot {
                            point,
                            superposition: Vec::new(),
                            entropy: 0,
                            tile: Some(tile.clone()),
                        },
                    );
                    // println!("tile: {:?}", tile);
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
    }
}

fn update_tile(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut tile_map: ResMut<TileMap>,
    camera_query: Query<
        (&Transform, &super::camera_ctrl_plugin::CameraCtrl),
        Without<super::network_plugin::SynEntity>,
    >,
    net_state: ResMut<NetWorkState>,
) {
    if let Some((camera_transform, _)) = camera_query.iter().next() {
        // 修改center_point
        let camera_point =
            common::tile_map::pos_to_global_point(&tile_map, camera_transform.translation);
        tile_map.center_point = IVec3::new(camera_point.x, camera_point.y, tile_map.center_point.z);
        // 计算tile_size大小
        let tile_size = tile_map.texture_size * tile_map.chunk_size;

        let min_x = tile_map.center_point.x - (tile_map.map_size.x as i32 / 2);
        let max_x = tile_map.center_point.x + (tile_map.map_size.x as i32 / 2);
        let min_y = tile_map.center_point.y - (tile_map.map_size.y as i32 / 2);
        let max_y = tile_map.center_point.y + (tile_map.map_size.y as i32 / 2);

        let center_pos = tile_map.center_point.as_f32()
            * tile_map.texture_size.as_f32()
            * tile_map.chunk_size.as_f32();

        // 2. 按Z轴从小到大生成图层
        for z in 0..tile_map.map_size.z {
            for x in min_x..=max_x {
                for y in min_y..=max_y {
                    let point = IVec3::new(x as i32, y as i32, z as i32);
                    let pos_x = x as f32 * tile_size.x as f32 + center_pos.x;
                    let pos_y = y as f32 * tile_size.y as f32 + center_pos.y;
                    let tile_pos = Vec3::new(pos_x, pos_y, z as f32);
                    // debug!("slot: ({},{}) pos: ({})", x, y, tile_pos);

                    // let rigid_body = RigidBodyBuilder::new_static().translation(tile_pos.x, tile_pos.y);
                    // let collider = ColliderBuilder::cuboid(tile_size.x / 2f32, tile_size.y / 2f32);
                    // println!("point: {}", point);
                    if let Some(slot) = tile_map.slot_map.get(&point) {
                        if let Some(_) = &slot.tile {
                            continue;
                        }
                    }
                    if let Some(tile) = get_tile(point, &net_state) {
                        // println!("tile: {:?}", tile);
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
        }
    }
}
