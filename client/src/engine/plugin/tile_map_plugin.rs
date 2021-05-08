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
            map_size: UVec3::new(56, 56, 2),
            slot_map: HashMap::new(),
        })
        .add_startup_system(setup.system());
    }
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

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut tile_map: ResMut<TileMap>,
    _window: Res<WindowDescriptor>,
    net_state: ResMut<NetWorkState>,
) {
    // 计算tile_size大小
    let tile_size = tile_map.texture_size * tile_map.chunk_size;

    let center_pos = tile_map.center_point.as_f32()
        * tile_map.texture_size.as_f32()
        * tile_map.chunk_size.as_f32();

    let min_x = tile_map.center_point.x - (tile_map.map_size.x as i32 / 2);
    let max_x = tile_map.center_point.x + (tile_map.map_size.x as i32 / 2);
    let min_y = tile_map.center_point.y - (tile_map.map_size.y as i32 / 2);
    let max_y = tile_map.center_point.y + (tile_map.map_size.y as i32 / 2);

    // 2. 按Z轴从小到大生成图层
    for z in 1..tile_map.map_size.z {
        for x in min_x..=max_x {
            for y in min_y..=max_y {
                let point = IVec3::new(x as i32, y as i32, z as i32);
                let pos_x = x as f32 * tile_size.x as f32 + center_pos.x;
                let pos_y = y as f32 * tile_size.y as f32 + center_pos.y;
                let tile_pos = Vec3::new(pos_x, pos_y, z as f32);

                if let Some(tile) = get_tile(point, &net_state) {
                    // 若最上层也为泥地则不创建精灵
                    if z == 1 && tile.filename.eq("0-tileset_30.png") {
                        continue;
                    }

                    tile_map.slot_map.insert(
                        point.clone(),
                        Slot {
                            point,
                            superposition: Vec::new(),
                            entropy: 0,
                            tile: Some(tile.clone()),
                        },
                    );

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
