use bevy::{
    input::system::exit_on_esc_system,
    prelude::*,
    wgpu::{WgpuBackend, WgpuOptions, WgpuPowerOptions},
};
use example::plugin::tile_map::TileMapPlugin;

fn main() {
    App::build()
        .insert_resource(WgpuOptions {
            backend: WgpuBackend::Auto,
            power_pref: WgpuPowerOptions::HighPerformance,
            ..Default::default()
        })
        .insert_resource(WindowDescriptor {
            title: String::from("初始游戏"),
            // 垂直同步
            vsync: true,
            // 是否可调整窗口大小
            resizable: true,
            // 是否有窗口外壳
            decorations: true,
            width: 400f32,
            height: 400f32,
            // 窗口模式
            // mode: WindowMode::BorderlessFullscreen,
            // 鼠标隐藏并锁定
            // cursor_locked: true,
            // cursor_visible: false,
            ..Default::default()
        })
        .insert_resource(Msaa { samples: 4 })
        // 窗口背景色
        .insert_resource(ClearColor(Color::rgb(0.5, 0.5, 0.9)))
        // 设置摄像机
        .add_startup_system(set_camera.system())
        // 默认插件
        .add_plugins(DefaultPlugins)
        // esc退出系统
        .add_system(exit_on_esc_system.system())
        // 地图初始化
        .add_plugin(TileMapPlugin)
        .run();
}

fn set_camera(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(UiCameraBundle::default());
}
