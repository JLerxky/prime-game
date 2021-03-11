use bevy::prelude::*;
use heron::{Gravity, PhysicsPlugin};

use super::{
    event::{
        map_event::MapEventPlugin, move_event::MoveEventPlugin, window_event::WindowEventPlugin,
    },
    plugin::{
        camera_ctrl::CameraCtrl, clipboard::Clipboard, fps::Fps, player::PlayerPlugin,
        tile_map::TileMapPlugin,
    },
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
            width: 1600f32,
            height: 900f32,
            // 窗口模式
            // mode: WindowMode::BorderlessFullscreen,
            // 鼠标隐藏并锁定
            cursor_locked: true,
            cursor_visible: false,
            ..Default::default()
        })
        .add_resource(Msaa { samples: 8 })
        // 默认插件
        .add_plugins(DefaultPlugins)
        // 物理插件
        .add_plugin(PhysicsPlugin::default())
        .add_resource(Gravity::from(Vec3::new(0.0, -120.0, 0.0)))
        // 设置摄像机
        .add_startup_system(set_camera.system())
        // 辅助功能插件
        .add_plugin(Fps)
        .add_plugin(Clipboard)
        // 事件
        .add_plugin(MoveEventPlugin)
        .add_plugin(MapEventPlugin)
        .add_plugin(WindowEventPlugin)
        // 地图初始化
        .add_plugin(TileMapPlugin)
        // 玩家
        .add_plugin(PlayerPlugin)
        .run();
}

fn set_camera(commands: &mut Commands) {
    commands
        // cameras
        .spawn(Camera2dBundle::default())
        .with(CameraCtrl)
        .spawn(CameraUiBundle::default());
}

// pub fn run_snake() {
//     snake();
// }
