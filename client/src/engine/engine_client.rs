use bevy::{
    input::system::exit_on_esc_system,
    prelude::*,
    wgpu::{WgpuBackend, WgpuOptions, WgpuPowerOptions},
    winit::WinitPlugin,
};
use bevy_rapier2d::{
    na::Vector2,
    physics::{RapierConfiguration, RapierPhysicsPlugin},
    rapier::pipeline::PhysicsPipeline,
    render::RapierRenderPlugin,
};
// use protocol::Packet;

use super::{
    event::{
        control_event::ControlEventPlugin, heart_beat_event::HeartBeatEventPlugin,
        keyboard_event::KeyboardEventPlugin, map_event::MapEventPlugin,
        sync_event::SyncEventPlugin,
    },
    plugin::{
        camera_ctrl_plugin::CameraCtrl,
        clipboard_plugin::Clipboard,
        fps_plugin::Fps,
        network_plugin::{NetworkPlugin, SynEntity},
        ping_plugin::Ping,
        tile_map_plugin::TileMapPlugin,
    },
};

pub fn engine_start() {
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
            resizable: false,
            // 是否有窗口外壳
            decorations: true,
            width: 800f32,
            height: 450f32,
            // 窗口模式
            // mode: WindowMode::BorderlessFullscreen,
            // 鼠标隐藏并锁定
            // cursor_locked: true,
            // cursor_visible: false,
            ..Default::default()
        })
        // .insert_resource(Msaa { samples: 4 })
        // 窗口背景色
        .insert_resource(ClearColor(Color::rgb(0.5, 0.5, 0.9)))
        // 默认插件
        .add_plugins(DefaultPlugins)
        // esc退出系统
        .add_system(exit_on_esc_system.system())
        // 窗口插件
        // .add_resource(ClearColor(Color::rgb(
        //     0xF9 as f32 / 255.0,
        //     0xF9 as f32 / 255.0,
        //     0xFF as f32 / 255.0,
        // )))
        .add_plugin(WinitPlugin::default())
        // .add_plugin(WgpuPlugin::default())
        // 物理插件
        .add_plugin(RapierPhysicsPlugin)
        .add_plugin(RapierRenderPlugin)
        .add_startup_system(setup_graphics.system())
        .add_startup_system(enable_physics_profiling.system())
        // 设置摄像机
        .add_startup_system(set_camera.system())
        // 辅助功能插件
        .add_plugin(Fps)
        .add_plugin(Ping)
        .add_plugin(Clipboard)
        // 事件
        .add_plugin(ControlEventPlugin)
        .add_plugin(KeyboardEventPlugin)
        .add_plugin(MapEventPlugin)
        .add_plugin(HeartBeatEventPlugin)
        .add_plugin(SyncEventPlugin)
        // .add_plugin(WindowEventPlugin)
        // 地图初始化
        .add_plugin(TileMapPlugin)
        // 玩家
        // .add_plugin(PlayerPlugin)
        // 网络
        .add_plugin(NetworkPlugin)
        // 动画
        .add_system(animate_system.system())
        .run();
}

fn set_camera(mut commands: Commands) {
    commands
        // cameras
        .spawn_bundle(OrthographicCameraBundle::new_2d())
        .insert(CameraCtrl);
    commands
        .spawn_bundle(UiCameraBundle::default());
}

fn setup_graphics(mut commands: Commands, mut rapier_config: ResMut<RapierConfiguration>) {
    // configuration.scale = 40.0;

    rapier_config.gravity = Vector2::new(0.0, -512.0);
    commands.spawn_bundle(LightBundle {
        transform: Transform::from_translation(Vec3::new(1000.0, 100.0, 2000.0)),
        ..Default::default()
    });
}

fn enable_physics_profiling(mut pipeline: ResMut<PhysicsPipeline>) {
    pipeline.counters.enable()
}

fn animate_system(
    time: Res<Time>,
    mut animate_entity_query: Query<
        (&mut Timer, &mut TextureAtlasSprite, &Handle<TextureAtlas>),
        With<SynEntity>,
    >,
    texture_atlases: Res<Assets<TextureAtlas>>,
) {
    for (mut timer, mut sprite, texture_atlas_handle) in animate_entity_query.iter_mut() {
        timer.tick(time.delta());
        if timer.finished() {
            if let Some(texture_atlas) = texture_atlases.get(texture_atlas_handle) {
                sprite.index = ((sprite.index as usize + 1) % texture_atlas.textures.len()) as u32;
            }
        }
    }
}
