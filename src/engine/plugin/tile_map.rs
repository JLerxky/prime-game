use bevy::{core::FixedTimestep, prelude::*};
use bevy_rapier2d::rapier::{dynamics::RigidBodyBuilder, geometry::ColliderBuilder};
use serde::{Deserialize, Serialize};

use crate::engine::event::map_event::MapEvent;

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
    pub id: u32,
    // 层级
    pub layer: u8,
    // 可在哪行生成
    pub row: i32,
    // 可连接id
    pub top: u32,
    pub down: u32,
    pub left: u32,
    pub right: u32,
}

impl Tile {
    fn name(&self) -> String {
        format!("{}_{}_{}", self.layer, self.row, self.row)
    }
    fn filename(&self) -> String {
        format!(
            "textures/tiles/{}_{}_{}.png",
            self.layer, self.row, self.row
        )
    }
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
            layer: 0,
            row: 6,
            top: 0,
            down: 0,
            left: 0,
            right: 0,
        }),
        Some(Tile {
            id: 1,
            layer: 0,
            row: 5,
            top: 0,
            down: 0,
            left: 0,
            right: 0,
        }),
        Some(Tile {
            id: 1,
            layer: 0,
            row: 4,
            top: 0,
            down: 0,
            left: 0,
            right: 0,
        }),
        Some(Tile {
            id: 1,
            layer: 0,
            row: 3,
            top: 0,
            down: 0,
            left: 0,
            right: 0,
        }),
        Some(Tile {
            id: 1,
            layer: 0,
            row: 2,
            top: 0,
            down: 0,
            left: 0,
            right: 0,
        }),
        Some(Tile {
            id: 1,
            layer: 0,
            row: 1,
            top: 0,
            down: 0,
            left: 0,
            right: 0,
        }),
        Some(Tile {
            id: 1,
            layer: 0,
            row: 0,
            top: 0,
            down: 0,
            left: 0,
            right: 0,
        }),
        Some(Tile {
            id: 1,
            layer: 0,
            row: -1,
            top: 0,
            down: 0,
            left: 0,
            right: 0,
        }),
        Some(Tile {
            id: 1,
            layer: 10,
            row: -2,
            top: 0,
            down: 0,
            left: 0,
            right: 0,
        }),
        Some(Tile {
            id: 1,
            layer: 10,
            row: -3,
            top: 0,
            down: 0,
            left: 0,
            right: 0,
        }),
        Some(Tile {
            id: 1,
            layer: 10,
            row: -4,
            top: 0,
            down: 0,
            left: 0,
            right: 0,
        }),
        Some(Tile {
            id: 1,
            layer: 10,
            row: -5,
            top: 0,
            down: 0,
            left: 0,
            right: 0,
        }),
        Some(Tile {
            id: 1,
            layer: 10,
            row: -6,
            top: 0,
            down: 0,
            left: 0,
            right: 0,
        }),
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
}

pub struct TileMapPlugin;

impl Plugin for TileMapPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.register_type::<MapState>()
            .add_resource(MapState {
                tile_center: Vec3::new(0f32, 0f32, 0f32),
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
    // 生成地图
    let tile_center = map_state.tile_center;
    let tile_size = Vec2::new(window.width / 21f32 * 1f32, window.height / 13f32 * 1f32);
    println!(
        "窗口大小: {},{}; 瓷砖大小: {:?}",
        window.width, window.height, tile_size
    );
    // 长21格，高13格
    let x_size: i32 = 21i32;
    let y_size: i32 = 13i32;

    // 波函数坍缩生成场景
    // let slots = wave_func_collapse(
    //     Vec3::new(0.0, 0.0, 0.0),
    //     Vec3::new(add_x as f32, add_y as f32, 0f32),
    //     tile_size,
    // );

    /*
     * The ground
     */
    commands
        .spawn(SpriteBundle {
            material: materials.add(Color::rgb(0.0, 0.0, 0.8).into()),
            sprite: Sprite::new(Vec2::new(300.0, 10.0)),
            ..Default::default()
        })
        .with(RigidBodyBuilder::new_static().translation(400.0, -tile_size.y / 2f32))
        .with(ColliderBuilder::cuboid(150.0, 5.0).friction(0.0));

    commands
        .spawn(SpriteBundle {
            material: materials.add(Color::rgb(0.0, 0.0, 0.8).into()),
            sprite: Sprite::new(Vec2::new(300.0, 10.0)),
            ..Default::default()
        })
        .with(RigidBodyBuilder::new_static().translation(0.0, -tile_size.y / 2f32 * 3f32 + 5f32))
        .with(ColliderBuilder::cuboid(150.0, 5.0).friction(0.0));

    // commands
    //     .spawn(SpriteBundle {
    //         material: materials.add(Color::rgb(0.0, 0.0, 0.0).into()),
    //         sprite: Sprite::new(Vec2::new(300.0, 10.0)),
    //         transform: Transform::from_translation(Vec3::new(0.0, tile_size.y, 10.0)),
    //         ..Default::default()
    //     })
    //     .with(RigidBodyBuilder::new_static())
    //     .with(ColliderBuilder::cuboid(150.0, 5.0));

    // 天空背景
    let mut texture_handle;
    for x in -x_size / 2..=x_size / 2 {
        let x_pos = x as f32 * tile_size.x;
        for y in -y_size / 2..=y_size / 2 {
            for z in 0..=1 {
                let tile_position =
                    Vec3::new(x_pos, y as f32 * tile_size.y, z as f32 * 10f32) + tile_center;

                // let slot_option = slots.get(&vec3_to_key(tile_position));
                // if let Some(slot) = slot_option {
                //     if let Some(tile) = slot.tile {
                //         texture_handle = materials.add(
                //             asset_server
                //                 .load(format!("textures/tiles/{}.png", tile.id).as_str())
                //                 .into(),
                //         );
                //     }
                // }
                if y <= -2 && y >= -6 && z >= 1 {
                    texture_handle = materials.add(
                        asset_server
                            .load(format!("textures/tiles/{}_{}_1.png", z as i8 * 10i8, y).as_str())
                            .into(),
                    );

                    let rigid_body = RigidBodyBuilder::new_static()
                        .translation(tile_position.x, tile_position.y);
                    let collider = ColliderBuilder::cuboid(tile_size.x / 2f32, tile_size.y / 2f32);

                    commands
                        .spawn(SpriteBundle {
                            material: texture_handle.clone(),
                            sprite: Sprite::new(tile_size),
                            transform: Transform::from_translation(tile_position),
                            ..Default::default()
                        })
                        .with(rigid_body)
                        .with(collider.friction(0.0))
                        .with(Slot {
                            position: tile_position,
                            is_collapsed: true,
                            superposition: [None; 13],
                            entropy: 0,
                            tile: None,
                        });
                }
                if (y > -2 || y < -6) && z <= 0 {
                    texture_handle = materials.add(
                        asset_server
                            .load(format!("textures/tiles/{}_{}_1.png", z as i8 * 10i8, y).as_str())
                            .into(),
                    );
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
                            superposition: [None; 13],
                            entropy: 0,
                            tile: None,
                        });
                }
            }
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
    asset_server: Res<AssetServer>,
) {
    let mut has_event = false;
    for _ in map_event_reader.iter(&map_events) {
        has_event = true;
    }
    map_events.clear();
    if has_event {
        println!("生成事件！");
        let camera_transform = camera_transform_query.iter().next().unwrap();
        map_state.tile_center = camera_transform.translation;

        let tile_size = Vec2::new(window.width / 21f32 * 1f32, window.height / 13f32 * 1f32);
        let mut count = 0;
        // 长21格，高13格
        let x_size: i32 = 21i32;
        let y_size: i32 = 13i32;

        let tile_center_transform = Vec3::new(
            (camera_transform.translation.x as i32 / tile_size.x as i32) as f32 * tile_size.x,
            (camera_transform.translation.y as i32 / tile_size.y as i32) as f32 * tile_size.y,
            0f32,
        );

        let mut texture_handle;
        for x in -x_size / 2 - 2..=x_size / 2 + 2 {
            let x_pos = x as f32 * tile_size.x;
            for y in -y_size / 2..=y_size / 2 {
                for z in 0..=1 {
                    let tile_position = Vec3::new(x_pos, y as f32 * tile_size.y, z as f32 * 10f32)
                        + tile_center_transform;
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

                    if y <= -2 && y >= -6 && z >= 1 {
                        texture_handle = materials.add(
                            asset_server
                                .load(
                                    format!("textures/tiles/{}_{}_1.png", z as i8 * 10i8, y)
                                        .as_str(),
                                )
                                .into(),
                        );

                        let rigid_body = RigidBodyBuilder::new_static()
                            .translation(tile_position.x, tile_position.y);
                        let collider =
                            ColliderBuilder::cuboid(tile_size.x / 2f32, tile_size.y / 2f32);

                        commands
                            .spawn(SpriteBundle {
                                material: texture_handle.clone(),
                                sprite: Sprite::new(tile_size),
                                transform: Transform::from_translation(tile_position),
                                ..Default::default()
                            })
                            .with(rigid_body)
                            .with(collider.friction(0.0))
                            .with(Slot {
                                position: tile_position,
                                is_collapsed: true,
                                superposition: [None; 13],
                                entropy: 0,
                                tile: None,
                            });
                        // 生成
                        count += 1;
                    }
                    if (y > -2 || y < -6) && z <= 0 {
                        texture_handle = materials.add(
                            asset_server
                                .load(
                                    format!("textures/tiles/{}_{}_1.png", z as i8 * 10i8, y)
                                        .as_str(),
                                )
                                .into(),
                        );
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
                                superposition: [None; 13],
                                entropy: 0,
                                tile: None,
                            });
                        // 生成
                        count += 1;
                    }
                }
            }
        }

        if count > 0 {
            println!(
                "{:?},{:?}",
                camera_transform.translation, map_state.tile_center
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
    let tile_size = Vec2::new(
        window.width / 1920f32 * 64f32,
        window.height / 1080f32 * 64f32,
    );
    let w = window.width / 2f32 + (tile_size.x * 2f32);
    let h = window.height / 2f32 + (tile_size.y * 2f32);
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
