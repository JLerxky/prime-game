use bevy::prelude::*;

use super::{
    event::{move_event::MoveEventPlugin, window_event::WindowEventPlugin},
    plugin::{
        camera_ctrl::CameraCtrl, clipboard::Clipboard, fps::Fps, player::PlayerPlugin,
        tile_map::TileMapPlugin,
    },
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
        // 地图初始化
        .add_plugin(TileMapPlugin)
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
    // 设置鼠标初始位置
    let window = windows.get_primary_mut().unwrap();
    window.set_cursor_position(Vec2::new(window.width() / 2f32, window.height() / 2f32));
}

pub fn run_snake() {
    snake();
}
