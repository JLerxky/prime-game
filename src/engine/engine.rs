use bevy::prelude::*;

use super::{
    event::my_event::MyEventPlugin,
    plugin::{clipboard::Clipboard, fps::Fps},
    scene::snake::snake,
};

pub fn engine_start() {
    App::build()
        .add_resource(WindowDescriptor {
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
            title: String::from("初始游戏"),
            ..Default::default()
        })
        // 设置摄像机
        .add_startup_system(setCamera.system())
        // 默认插件
        .add_plugins(DefaultPlugins)
        // 辅助功能插件
        .add_plugin(Fps)
        .add_plugin(Clipboard)
        // 事件
        .add_plugin(MyEventPlugin)
        // 场景
        // .add_plugin(BreakOut)
        .run();
}

fn setCamera(commands: &mut Commands) {
    commands
        // cameras
        .spawn(Camera2dBundle::default())
        .spawn(CameraUiBundle::default());
}

pub fn run_snake() {
    snake();
}
