use bevy::prelude::*;

use crate::util::wave_func_collapse::wave_func_collapse;

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

pub struct TileMapPlugin;

impl Plugin for TileMapPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(setup.system())
            .add_system(tile_map_collapse_system.system());
    }
}

fn setup(commands: &mut Commands, mut materials: ResMut<Assets<ColorMaterial>>) {
    // 生成地图
    let tile_rows = 30;
    let tile_columns = 60;
    let tile_spacing = 1.0;
    let tile_size = Vec2::new(40.0, 40.0);
    let tiles_width = tile_columns as f32 * (tile_size.x + tile_spacing) - tile_spacing;
    let tiles_high = tile_rows as f32 * (tile_size.y + tile_spacing) - tile_spacing;

    let tiles_offset = Vec3::new(
        -(tiles_width - tile_size.x) / 2.0,
        -(tiles_high - tile_size.y) / 2.0,
        0.0,
    );

    let slots = wave_func_collapse(Vec2::new(0.0, 0.0), 1, 1);

    for row in 0..=tile_rows {
        let y_position = row as f32 * (tile_size.y + tile_spacing);
        for column in 0..=tile_columns {
            let texture_handle;
            // let slot = slots[column][row].clone();
            // if let Some(tile) = slot.tile {
            //     texture_handle = materials.add(
            //         asset_server
            //             .load(format!("textures/tiles/{}.png", tile.name).as_str())
            //             .into(),
            //     );
            // } else {
            texture_handle = materials.add(Color::rgb(0.5, 0.5, 1.0).into());
            // }
            let tile_position = Vec3::new(
                column as f32 * (tile_size.x + tile_spacing),
                y_position,
                0.0,
            ) + tiles_offset;
            commands
                .spawn(SpriteBundle {
                    material: texture_handle.clone(),
                    sprite: Sprite::new(tile_size),
                    transform: Transform::from_translation(tile_position),
                    ..Default::default()
                })
                .with(Position { x: column, y: row });
            // commands.spawn(TextBundle {
            //     text: Text {
            //         value: format!("[{},{}]", column, row),
            //         font: asset_server.load("fonts/YouZai.ttf"),
            //         style: TextStyle {
            //             color: Color::rgb(0.5, 0.5, 1.0),
            //             font_size: 14.0,
            //             ..Default::default()
            //         },
            //     },
            //     style: Style {
            //         position_type: PositionType::Absolute,
            //         position: Rect {
            //             bottom: Val::Px(tile_position[1] + 400.0),
            //             left: Val::Px(tile_position[0] + 400.0),
            //             ..Default::default()
            //         },
            //         ..Default::default()
            //     },
            //     transform: Transform::from_translation(tile_position),
            //     ..Default::default()
            // });
        }
    }
}

fn tile_map_collapse_system() {}
