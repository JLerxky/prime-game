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
    pub position: Vec2,
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
            position: Vec2::new(0.0, 0.0),
            is_collapsed: false,
            superposition: vec![],
            entropy: 0,
            tile: None,
        }
    }
}

pub struct TileMap {
    transform: Vec3,
}

pub struct TileMapPlugin;

impl Plugin for TileMapPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(setup.system())
            .add_system(tile_map_collapse_system.system());
    }
}

fn setup(
    commands: &mut Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
) {
    // 生成地图
    let add_x: usize = 19;
    let add_y: usize = 11;
    let tile_center = Vec3::new(0f32, 0f32, 0f32);
    let tile_size = Vec2::new(50.0, 50.0);

    let slots = wave_func_collapse(Vec2::new(0.0, 0.0), add_x, add_y);

    let mut texture_handle = materials.add(Color::rgb(0.5, 0.5, 1.0).into());
    for x in -(add_x as i32)..=(add_x as i32) {
        let x_position = x as f32 * tile_size.y;
        for y in -(add_y as i32)..=(add_y as i32) {
            let tile_position = Vec3::new(x_position, y as f32 * tile_size.x, 0.0) + tile_center;

            let slot = slots[(x as usize + add_x) as usize][y as usize + add_y].clone();
            if let Some(tile) = slot.tile {
                texture_handle = materials.add(
                    asset_server
                        .load(format!("textures/tiles/{}.png", tile.name).as_str())
                        .into(),
                );
            }
            commands
                .spawn(SpriteBundle {
                    material: texture_handle.clone(),
                    sprite: Sprite::new(tile_size),
                    transform: Transform::from_translation(tile_position),
                    ..Default::default()
                })
                .with(TileMap {
                    transform: tile_position,
                });
        }
    }
}

fn tile_map_collapse_system(
    commands: &mut Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    player_transform_query: Query<&Transform, With<Player>>,
    camera_transform_query: Query<&Transform, With<CameraCtrl>>,
) {
    let player_transform = player_transform_query.iter().next().unwrap();
    let camera_transform = camera_transform_query.iter().next().unwrap();

    // println!("{:?}", player_transform);
}
