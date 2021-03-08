use bevy::{core::FixedTimestep, math::Vec3Swizzles, prelude::*};
use serde::{Deserialize, Serialize};

use crate::{
    engine::event::map_event::MapEvent,
    util::collapse::{vec3_to_key, wave_func_collapse},
};

use super::camera_ctrl::CameraCtrl;

// 坐标
#[derive(Copy, Clone, Debug)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}

// 瓷砖
#[derive(Copy, Clone, Debug, Deserialize, Serialize)]
pub struct Tile {
    // 文件名作为name
    pub id: i32,
    // 旋转 0-0 1-90 2-180 3-270
    pub rotation: u8,
    // 可连接id
    pub top: u32,
    pub down: u32,
    pub left: u32,
    pub right: u32,
}

// 位置
#[derive(Copy, Clone, Debug, Deserialize, Serialize)]
pub struct Slot {
    // 位置
    pub position: Vec3,
    // 是否坍缩
    pub is_collapsed: bool,
    // 叠加态（可选瓷砖集合）
    pub superposition: [Option<Tile>; 13],
    // 熵
    pub entropy: usize,
    // 确定态（当前瓷砖）
    pub tile: Option<Tile>,
}

pub fn get_tiles() -> [Option<Tile>; 13] {
    // 加载瓷砖素材
    [
        Some(Tile {
            id: 1,
            rotation: 0,
            top: 1,
            down: 0,
            left: 0,
            right: 1,
        }),
        Some(Tile {
            id: 2,
            rotation: 0,
            top: 0,
            down: 0,
            left: 1,
            right: 1,
        }),
        Some(Tile {
            id: 3,
            rotation: 0,
            top: 0,
            down: 1,
            left: 1,
            right: 0,
        }),
        Some(Tile {
            id: 4,
            rotation: 0,
            top: 1,
            down: 1,
            left: 0,
            right: 0,
        }),
        Some(Tile {
            id: 5,
            rotation: 0,
            top: 0,
            down: 1,
            left: 0,
            right: 1,
        }),
        Some(Tile {
            id: 6,
            rotation: 0,
            top: 1,
            down: 0,
            left: 1,
            right: 0,
        }),
        Some(Tile {
            id: 7,
            rotation: 0,
            top: 1,
            down: 1,
            left: 1,
            right: 0,
        }),
        Some(Tile {
            id: 8,
            rotation: 0,
            top: 0,
            down: 0,
            left: 0,
            right: 0,
        }),
        Some(Tile {
            id: 9,
            rotation: 0,
            top: 1,
            down: 1,
            left: 1,
            right: 1,
        }),
        Some(Tile {
            id: 10,
            rotation: 0,
            top: 1,
            down: 1,
            left: 1,
            right: 1,
        }),
        Some(Tile {
            id: 11,
            rotation: 0,
            top: 1,
            down: 0,
            left: 1,
            right: 1,
        }),
        Some(Tile {
            id: 12,
            rotation: 0,
            top: 1,
            down: 1,
            left: 0,
            right: 1,
        }),
        Some(Tile {
            id: 13,
            rotation: 0,
            top: 0,
            down: 1,
            left: 1,
            right: 1,
        }),
    ]
}

pub fn get_none_tiles() -> [Option<Tile>; 13] {
    [
        None, None, None, None, None, None, None, None, None, None, None, None, None,
    ]
}

impl Slot {
    pub fn new(position: Vec3) -> Slot {
        let tiles = get_tiles();
        Slot {
            position,
            is_collapsed: false,
            superposition: tiles,
            entropy: tiles.len(),
            tile: None,
        }
    }
}

#[derive(Reflect, Default)]
#[reflect(Component)]
pub struct MapState {
    tile_center: Vec3,
    tile_size: Vec3,
}

pub struct TileMapPlugin;

impl Plugin for TileMapPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.register_type::<MapState>()
            .add_resource(MapState {
                tile_center: Vec3::new(0f32, 0f32, 0f32),
                tile_size: Vec3::new(50f32, 50f32, 0f32),
            })
            .add_startup_system(setup.system())
            // .add_system(tile_map_produce_system.system())
            // .add_system(tile_map_clean_system.system())
            .add_stage_after(
                stage::UPDATE,
                "build_map_fixed_update",
                SystemStage::parallel()
                    .with_run_criteria(
                        FixedTimestep::step(0.1).with_label("build_map_fixed_timestep"),
                    )
                    .with_system(tile_map_produce_system.system()),
            )
            .add_stage_after(
                stage::UPDATE,
                "clean_map_fixed_update",
                SystemStage::parallel()
                    .with_run_criteria(
                        FixedTimestep::step(2.0).with_label("clean_map_fixed_timestep"),
                    )
                    .with_system(tile_map_clean_system.system()),
            );
    }
}

#[test]
fn test() {
    println!("{:?}", 1080 as i32 / 100i32);
    println!("{:?}", 1920 as i32 / 100i32);
}

fn setup<'a>(
    commands: &mut Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
    map_state: ResMut<MapState>,
    window: Res<WindowDescriptor>,
) {
    println!(
        "窗口大小: {},{}; 瓷砖大小: {:?}",
        window.width, window.height, map_state.tile_size
    );
    // 生成地图
    let tile_center = map_state.tile_center;
    let tile_size = map_state.tile_size;
    let mut add_x: usize = window.width as usize / (tile_size.x as usize * 2);
    let mut add_y: usize = window.height as usize / (tile_size.y as usize * 2);
    add_x += (add_x % 2 == 0) as usize + 1;
    add_y += (add_y % 2 == 0) as usize + 1;
    println!("瓷砖数: {},{}", add_x, add_y);

    let slots = wave_func_collapse(
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(add_x as f32, add_y as f32, 0f32),
        tile_size,
    );

    let mut texture_handle = materials.add(Color::rgb(0.5, 0.5, 1.0).into());
    for x in -(add_x as i32)..=(add_x as i32) {
        let x_position = x as f32 * tile_size.y;
        for y in -(add_y as i32)..=(add_y as i32) {
            let tile_position = Vec3::new(x_position, y as f32 * tile_size.x, 0.0) + tile_center;

            let slot_option = slots.get(&vec3_to_key(tile_position));
            if let Some(slot) = slot_option {
                if let Some(tile) = slot.tile {
                    texture_handle = materials.add(
                        asset_server
                            .load(format!("textures/tiles/{}.png", tile.id).as_str())
                            .into(),
                    );
                } else {
                    texture_handle = materials.add(Color::rgb(0.5, 0.5, 1.0).into());
                }
            }
            commands
                .spawn(SpriteBundle {
                    material: texture_handle.clone(),
                    sprite: Sprite::new(tile_size.xy()),
                    transform: Transform::from_translation(tile_position),
                    ..Default::default()
                })
                .with(Slot {
                    position: tile_position,
                    is_collapsed: true,
                    superposition: get_none_tiles(),
                    entropy: 0,
                    tile: None,
                });
        }
    }
}

fn tile_map_produce_system(
    commands: &mut Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut map_state: ResMut<MapState>,
    slot_exist_query: Query<(Entity, &Transform), With<Slot>>,
    // player_transform_query: Query<&Transform, With<Player>>,
    camera_transform_query: Query<&Transform, With<CameraCtrl>>,
    window: Res<WindowDescriptor>,
    mut map_event_reader: Local<EventReader<MapEvent>>,
    mut map_events: ResMut<Events<MapEvent>>,
) {
    let mut has_event = false;
    for _ in map_event_reader.iter(&map_events) {
        has_event = true;
    }
    map_events.clear();
    if has_event {
        println!("生成事件！");
        // let player_transform = player_transform_query.iter().next().unwrap();
        let camera_transform = camera_transform_query.iter().next().unwrap();
        map_state.tile_center = camera_transform.translation;
        let tile_size = map_state.tile_size;
        let mut count = 0;
        // 扩展地图
        let mut add_x: usize = window.width as usize / (tile_size.x as usize * 2);
        let mut add_y: usize = window.height as usize / (tile_size.y as usize * 2);
        add_x += (add_x % 2 == 0) as usize + 1;
        add_y += (add_y % 2 == 0) as usize + 1;

        let tile_center_transform = Vec3::new(
            (camera_transform.translation.x as i32 / tile_size.x as i32) as f32 * tile_size.x,
            (camera_transform.translation.y as i32 / tile_size.y as i32) as f32 * tile_size.y,
            0f32,
        );

        let texture_handle = materials.add(Color::rgb(0.5, 0.5, 1.0).into());

        for x in -(add_x as i32)..=(add_x as i32) {
            let x_position = x as f32 * tile_size.x;
            for y in -(add_y as i32)..=(add_y as i32) {
                let y_position = y as f32 * tile_size.y;
                let tile_position = Vec3::new(x_position, y_position, 0.0) + tile_center_transform;

                // println!(
                //     "{:?},{:?},{:?}",
                //     player_transform.translation, tile_center_transform, tile_position
                // );

                // 存在性检查
                let mut exist = false;
                for (_exist_entity, exist_transform) in slot_exist_query.iter() {
                    if exist_transform.translation.distance(tile_position) == 0f32 {
                        exist = true;
                        break;
                    }
                }
                if exist {
                    continue;
                }

                // 生成
                count += 1;
                commands
                    .spawn(SpriteBundle {
                        material: texture_handle.clone(),
                        sprite: Sprite::new(tile_size.xy()),
                        transform: Transform::from_translation(tile_position),
                        ..Default::default()
                    })
                    .with(Slot {
                        position: tile_position,
                        is_collapsed: true,
                        superposition: get_none_tiles(),
                        entropy: 0,
                        tile: None,
                    });
            }
        }

        if count > 0 {
            println!(
                "{:?},{:?}",
                camera_transform.translation, tile_center_transform
            );
            println!("新生成瓷砖: {}", count);
        }
    }
}

fn tile_map_clean_system(
    commands: &mut Commands,
    entity_query: Query<Entity>,
    slot_query: Query<(Entity, &Transform), With<Slot>>,
    camera_transform_query: Query<&Transform, With<CameraCtrl>>,
    window: Res<WindowDescriptor>,
    map_state: Res<MapState>,
    // mut map_event_reader: Local<EventReader<MapEvent>>,
    // map_events: Res<Events<MapEvent>>,
) {
    let camera_transform = camera_transform_query.iter().next().unwrap();
    // for map_event in map_event_reader.iter(&map_events) {
    // match map_event {
    //     MapEvent::Clean => {

    // println!(
    //     "window: {},{}; map_state: {:?}",
    //     window.width, window.height, map_state.tile_size
    // );
    let w = window.width / 2f32 + (map_state.tile_size.x * 2f32);
    let h = window.height / 2f32 + (map_state.tile_size.y * 2f32);
    for (tile_entity, tile_transform) in slot_query.iter() {
        if tile_transform.translation.x > camera_transform.translation.x + w
            || tile_transform.translation.x < camera_transform.translation.x - w
            || tile_transform.translation.y > camera_transform.translation.y + h
            || tile_transform.translation.y < camera_transform.translation.y - h
        {
            // println!("Clean: {:?}", tile_transform);
            commands.despawn_recursive(tile_entity);
        }
    }
    println!("垃圾回收完成，剩余实体数: {}", entity_query.iter().len());
    //     }
    //     _ => {}
    // }
    // }
}
