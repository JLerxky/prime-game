use bevy::prelude::*;

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
        .add_plugins(DefaultPlugins)
        .run();
}
