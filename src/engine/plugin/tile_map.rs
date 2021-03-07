use bevy::prelude::*;

use crate::util::wave_func_collapse::wave_func_collapse;

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
}

pub struct TileMapPlugin;

impl Plugin for TileMapPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_resource(MapState { slots: Vec::new() })
            .add_startup_system(setup.system())
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

fn setup(
    commands: &mut Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    camera_transform_query: Query<&Transform, With<CameraCtrl>>,
    asset_server: Res<AssetServer>,
    mut map_state: Local<MapState>,
) {
    // 生成地图
    let add_x: usize = 3;
    let add_y: usize = 3;
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
    // fixed_timesteps: Res<FixedTimesteps>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    slot_query: Query<(Entity, &Transform), With<Slot>>,
    player_transform_query: Query<&Transform, With<Player>>,
    camera_transform_query: Query<&Transform, With<CameraCtrl>>,
    mut map_state: Local<MapState>,
) {
    let player_transform = player_transform_query.iter().next().unwrap();
    let camera_transform = camera_transform_query.iter().next().unwrap();
    // 生成地图
    let add_x: usize = 3;
    let add_y: usize = 3;
    let tile_center_transform = Vec3::new(
        (player_transform.translation.x % 50f32) * 50f32,
        (player_transform.translation.y % 50f32) * 50f32,
        0f32,
    );
    let tile_size = Vec2::new(50.0, 50.0);

    let mut texture_handle = materials.add(Color::rgb(0.5, 0.5, 1.0).into());

    // println!("{:?}", camera_transform);
    for (tile_entity, tile_transform) in slot_query.iter() {
        // println!("{:?},{:?}", camera_transform, tile_transform.translation);
    }
}

fn tile_map_clean_system(
    commands: &mut Commands,
    slot_query: Query<(Entity, &Transform), With<Slot>>,
    camera_transform_query: Query<&Transform, With<CameraCtrl>>,
) {
    let camera_transform = camera_transform_query.iter().next().unwrap();

    // println!("{:?}", camera_transform);
    for (tile_entity, tile_transform) in slot_query.iter() {
        // println!("{:?},{:?}", camera_transform, tile_transform.translation);
        if tile_transform.translation.x > camera_transform.translation.x + 985f32
            || tile_transform.translation.x < camera_transform.translation.x - 985f32
            || tile_transform.translation.y > camera_transform.translation.y + 565f32
            || tile_transform.translation.y < camera_transform.translation.y - 565f32
        {
            commands.despawn_recursive(tile_entity);
        }
    }
}
