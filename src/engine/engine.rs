use bevy::prelude::*;

use crate::util::wave_func_collapse::wave_func_collapse;

use super::{
    event::{move_event::MoveEventPlugin, window_event::WindowEventPlugin},
    plugin::{camera_ctrl::CameraCtrl, clipboard::Clipboard, fps::Fps, player::PlayerPlugin},
    scene::snake::snake,
};

pub fn engine_start() {
    App::build()
        .add_resource(WindowDescriptor {
            title: String::from("初始游戏"),
            // 垂直同步
            vsync: true,
            // 是否可调整窗口大小
            resizable: false,
            // 是否有窗口外壳
            decorations: true,
            width: 1920f32,
            height: 1080f32,
            // 窗口模式
            // mode: WindowMode::BorderlessFullscreen,
            // 鼠标隐藏并锁定
            cursor_locked: true,
            cursor_visible: false,
            ..Default::default()
        })
        .add_resource(Msaa { samples: 8 })
        // 设置摄像机
        .add_startup_system(setCamera.system())
        // 初始设置
        .add_startup_system(setup.system())
        // 默认插件
        .add_plugins(DefaultPlugins)
        // 辅助功能插件
        .add_plugin(Fps)
        .add_plugin(Clipboard)
        // 事件
        .add_plugin(MoveEventPlugin)
        .add_plugin(WindowEventPlugin)
        // 玩家
        .add_plugin(PlayerPlugin)
        .run();
}

fn setCamera(commands: &mut Commands) {
    commands
        // cameras
        .spawn(Camera2dBundle::default())
        .with(CameraCtrl)
        .spawn(CameraUiBundle::default());
}

fn setup(
    commands: &mut Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
    mut windows: ResMut<Windows>,
) {
    let window = windows.get_primary_mut().unwrap();
    window.set_cursor_position(Vec2::new(window.width() / 2f32, window.height() / 2f32));
    let brick_rows = 30;
    let brick_columns = 30;
    let brick_spacing = 1.0;
    let brick_size = Vec2::new(40.0, 40.0);
    let bricks_width = brick_columns as f32 * (brick_size.x + brick_spacing) - brick_spacing;
    // center the bricks and move them up a bit
    let bricks_offset = Vec3::new(-(bricks_width - brick_size.x) / 2.0, 0.0, 0.0);

    let slots = wave_func_collapse(Vec2::new(-9.0, -9.0), 1, 1);

    for row in 0..=brick_rows {
        let y_position = row as f32 * (brick_size.y + brick_spacing);
        for column in 0..=brick_columns {
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
            let brick_position = Vec3::new(
                column as f32 * (brick_size.x + brick_spacing),
                y_position,
                0.0,
            ) + bricks_offset;
            commands.spawn(SpriteBundle {
                material: texture_handle.clone(),
                sprite: Sprite::new(brick_size),
                transform: Transform::from_translation(brick_position),
                ..Default::default()
            });
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
            //             bottom: Val::Px(brick_position[1] + 400.0),
            //             left: Val::Px(brick_position[0] + 400.0),
            //             ..Default::default()
            //         },
            //         ..Default::default()
            //     },
            //     transform: Transform::from_translation(brick_position),
            //     ..Default::default()
            // });
        }
    }
}

pub fn run_snake() {
    snake();
}
