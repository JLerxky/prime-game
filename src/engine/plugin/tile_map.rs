use std::collections::HashMap;

use bevy::prelude::*;

use crate::{engine::event::map_event::MapEvent, util::wave_func_collapse::wave_func_collapse};

use super::{camera_ctrl::CameraCtrl, player::Player};

// 坐标
#[derive(Copy, Clone, Debug)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}

// 瓷砖
#[derive(Clone, Debug)]
pub struct Tile {
    // 文件名作为name
    pub name: String,
    // 旋转 0-0 1-90 2-180 3-270
    pub rotation: u8,
    // 可连接id
    pub top: u32,
    pub down: u32,
    pub left: u32,
    pub right: u32,
}

// 位置
#[derive(Clone, Debug)]
pub struct Slot {
    // 位置
    pub position: Vec3,
    // 是否坍缩
    pub is_collapsed: bool,
    // 叠加态（可选瓷砖集合）
    pub superposition: Vec<Tile>,
    // 熵
    pub entropy: u64,
    // 确定态（当前瓷砖）
    pub tile: Option<Tile>,
}

impl Slot {
    pub fn new() -> Slot {
        Slot {
            position: Vec3::new(0.0, 0.0, 0.0),
            is_collapsed: false,
            superposition: vec![],
            entropy: 0,
            tile: None,
        }
    }
}

pub struct TileMap {
    position: Vec3,
}

#[derive(Default)]
pub struct MapState {
    slots: Vec<Vec<Slot>>,
    tiles: HashMap<i32, HashMap<i32, bool>>,
}

pub struct TileMapPlugin;

impl Plugin for TileMapPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_resource(MapState { slots: Vec::new(), tiles: HashMap::new() })
            .add_startup_system(setup.system())
            .add_system(tile_map_produce_system.system())
            .add_system(tile_map_clean_system.system());
        // .add_stage_after(
        //     stage::UPDATE,
        //     "fixed_update",
        //     SystemStage::parallel()
        //         .with_run_criteria(
        //             FixedTimestep::step(1.0).with_label("build_map_fixed_timestep"),
        //         )
        //         .with_system(tile_map_collapse_system.system()),
        // );
    }
}

#[test]
fn test() {
    println!("{:?}", 1080 as i32 / 100i32);
    println!("{:?}", 1920 as i32 / 100i32);
}

fn setup(
    commands: &mut Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    camera_transform_query: Query<&Transform, With<CameraCtrl>>,
    asset_server: Res<AssetServer>,
    mut map_state: Local<MapState>,
    window: Res<WindowDescriptor>,
) {
    println!("{},{}", window.width, window.height);
    // 生成地图
    let mut add_x: usize = window.width as usize / 100usize;
    let mut add_y: usize = window.height as usize / 100usize;
    add_x += (add_x % 2 == 0) as usize;
    add_y += (add_y % 2 == 0) as usize;
    println!("{},{}", add_x, add_y);

    let tile_center = Vec3::new(0f32, 0f32, 0f32);
    let tile_size = Vec2::new(50.0, 50.0);

    map_state.slots = wave_func_collapse(Vec3::new(0.0, 0.0, 0.0), add_x, add_y);

    let mut texture_handle = materials.add(Color::rgb(0.5, 0.5, 1.0).into());
    for x in -(add_x as i32)..=(add_x as i32) {
        let x_position = x as f32 * tile_size.y;
        for y in -(add_y as i32)..=(add_y as i32) {
            let tile_position = Vec3::new(x_position, y as f32 * tile_size.x, 0.0) + tile_center;

            // let slot = slots[(x as usize + add_x) as usize][y as usize + add_y].clone();
            // if let Some(tile) = slot.tile {
            //     texture_handle = materials.add(
            //         asset_server
            //             .load(format!("textures/tiles/{}.png", tile.name).as_str())
            //             .into(),
            //     );
            // }
            commands
                .spawn(SpriteBundle {
                    material: texture_handle.clone(),
                    sprite: Sprite::new(tile_size),
                    transform: Transform::from_translation(tile_position),
                    ..Default::default()
                })
                .with(Slot {
                    position: tile_position,
                    is_collapsed: true,
                    superposition: vec![],
                    entropy: 0,
                    tile: None,
                });
        }
    }
}

fn tile_map_produce_system(
    commands: &mut Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    slot_query: Query<(Entity, &Transform), With<Slot>>,
    player_transform_query: Query<&Transform, With<Player>>,
    camera_transform_query: Query<&Transform, With<CameraCtrl>>,
    window: Res<WindowDescriptor>,
    mut map_event_reader: Local<EventReader<MapEvent>>,
    map_events: Res<Events<MapEvent>>,
) {
    let player_transform = player_transform_query.iter().next().unwrap();
    let camera_transform = camera_transform_query.iter().next().unwrap();
    let mut count = 0;
    for map_event in map_event_reader.iter(&map_events) {
        // 扩展地图
        let mut add_x: usize = 3;
        let mut add_y: usize = 3;
        add_x += (add_x % 2 == 0) as usize;
        add_y += (add_y % 2 == 0) as usize;

        let mut tile_center_transform = Vec3::new(
            (player_transform.translation.x as i32 / 50i32) as f32 * 50f32,
            (player_transform.translation.y as i32 / 50i32) as f32 * 50f32,
            0f32,
        );

        // println!(
        //     "{:?},{:?}",
        //     player_transform.translation, tile_center_transform
        // );
        let tile_size = Vec2::new(50.0, 50.0);

        let mut texture_handle = materials.add(Color::rgb(0.5, 0.5, 1.0).into());
        match map_event {
            MapEvent::Add => {
                for x in -(add_x as i32)..=(add_x as i32) {
                    let x_position = x as f32 * tile_size.y;
                    for y in -(add_y as i32)..=(add_y as i32) {
                        let y_position = y as f32 * tile_size.x;
                        let tile_position =
                            Vec3::new(x_position, y_position, 0.0) + tile_center_transform;

                        // println!(
                        //     "{:?},{:?},{:?}",
                        //     player_transform.translation, tile_center_transform, tile_position
                        // );

                        count += 1;
                        commands
                            .spawn(SpriteBundle {
                                material: texture_handle.clone(),
                                sprite: Sprite::new(tile_size),
                                transform: Transform::from_translation(tile_position),
                                ..Default::default()
                            })
                            .with(Slot {
                                position: tile_position,
                                is_collapsed: true,
                                superposition: vec![],
                                entropy: 0,
                                tile: None,
                            });
                    }
                }
            }
            _ => {}
        }
    }
    println!("生成： {}, {}", count, slot_query.iter().count());
}

fn tile_map_clean_system(
    commands: &mut Commands,
    entity_query: Query<Entity>,
    slot_query: Query<(Entity, &Transform), With<Slot>>,
    camera_transform_query: Query<&Transform, With<CameraCtrl>>,
    // mut map_event_reader: Local<EventReader<MapEvent>>,
    // map_events: Res<Events<MapEvent>>,
) {
    let camera_transform = camera_transform_query.iter().next().unwrap();
    // for map_event in map_event_reader.iter(&map_events) {
    // match map_event {
    //     MapEvent::Clean => {
    for (tile_entity, tile_transform) in slot_query.iter() {
        if tile_transform.translation.x > camera_transform.translation.x + 985f32
            || tile_transform.translation.x < camera_transform.translation.x - 985f32
            || tile_transform.translation.y > camera_transform.translation.y + 565f32
            || tile_transform.translation.y < camera_transform.translation.y - 565f32
        {
            println!("Clean: {:?}", tile_transform);
            commands.despawn_recursive(tile_entity);
            println!("{}", entity_query.iter().len());
        }
    }
    //     }
    //     _ => {}
    // }
    // }
}
