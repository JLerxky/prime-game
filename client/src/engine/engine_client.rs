use bevy::{
    prelude::*,
    wgpu::{WgpuBackend, WgpuOptions, WgpuPowerOptions},
    window::WindowMode,
    winit::WinitPlugin,
};
use bevy_rapier2d::prelude::*;
// use protocol::Packet;

use super::{
    event::{
        control_event::ControlEventPlugin, heart_beat_event::HeartBeatEventPlugin,
        keyboard_event::KeyboardEventPlugin, map_event::MapEventPlugin,
        skill_event::SkillEventPlugin, sync_event::SyncEventPlugin,
    },
    plugin::{
        animate_plugin::AnimatePlugin, camera_ctrl_plugin::CameraCtrl,
        network_plugin::NetworkPlugin, player_plugin::PlayerPlugin, tile_map_plugin::TileMapPlugin,
        ui_plugin::UIPlugin,
    },
};

pub fn engine_start() {
    App::build()
        .insert_resource(WgpuOptions {
            backend: WgpuBackend::Auto,
            power_pref: WgpuPowerOptions::HighPerformance,
            // features: WgpuFeatures {
            //     features: vec![WgpuFeature::NonFillPolygonMode],
            // },
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
            width: 1366f32,
            height: 768f32,
            // 窗口模式
            mode: WindowMode::Windowed,
            // 鼠标隐藏并锁定
            // cursor_locked: true,
            // cursor_visible: false,
            ..Default::default()
        })
        // .insert_resource(Msaa { samples: 4 })
        // 窗口背景色
        .insert_resource(ClearColor(Color::rgb_u8(192, 126, 104)))
        // 默认插件
        .add_plugins(DefaultPlugins)
        // esc退出系统
        // .add_system(bevy::input::system::exit_on_esc_system.system())
        // 窗口插件
        // .add_resource(ClearColor(Color::rgb(
        //     0xF9 as f32 / 255.0,
        //     0xF9 as f32 / 255.0,
        //     0xFF as f32 / 255.0,
        // )))
        .add_plugin(WinitPlugin::default())
        // .add_plugin(WgpuPlugin::default())
        // 物理插件
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        // .add_plugin(RapierRenderPlugin)
        .add_startup_system(setup_graphics.system())
        .add_startup_system(enable_physics_profiling.system())
        // 设置摄像机
        .add_startup_system(set_camera.system())
        // BGM
        // .add_startup_system(setup_bgm.system())
        // 辅助功能插件
        // .add_plugin(super::plugin::fps_plugin::Fps)
        // .add_plugin(Ping)
        // .add_plugin(Clipboard)
        // 事件
        .add_plugin(ControlEventPlugin)
        .add_plugin(KeyboardEventPlugin)
        .add_plugin(MapEventPlugin)
        .add_plugin(HeartBeatEventPlugin)
        .add_plugin(SyncEventPlugin)
        .add_plugin(SkillEventPlugin)
        // .add_plugin(WindowEventPlugin)
        // 地图初始化
        .add_plugin(TileMapPlugin)
        // 玩家插件
        .add_plugin(PlayerPlugin)
        // 网络
        .add_plugin(NetworkPlugin)
        // 动画
        .add_plugin(AnimatePlugin)
        // egui
        .add_plugin(UIPlugin)
        // 日志输出
        // Adds frame time diagnostics
        // .add_plugin(bevy::diagnostic::FrameTimeDiagnosticsPlugin::default())
        // Adds a system that prints diagnostics to the console
        // .add_plugin(bevy::diagnostic::LogDiagnosticsPlugin::default())
        // Any plugin can register diagnostics
        // Uncomment this to add some render resource diagnostics:
        // .add_plugin(bevy::wgpu::diagnostic::WgpuResourceDiagnosticsPlugin::default())
        // Uncomment this to add an entity count diagnostics:
        // .add_plugin(bevy::diagnostic::EntityCountDiagnosticsPlugin::default())
        // Uncomment this to add an asset count diagnostics:
        // .add_plugin(bevy::asset::diagnostic::AssetCountDiagnosticsPlugin::<Texture>::default())
        .run();
}

fn set_camera(mut commands: Commands) {
    commands
        // cameras
        .spawn_bundle(OrthographicCameraBundle::new_2d())
        .insert(CameraCtrl);
    commands.spawn_bundle(UiCameraBundle::default());
}

fn setup_graphics(mut commands: Commands, mut rapier_config: ResMut<RapierConfiguration>) {
    // configuration.scale = 40.0;

    rapier_config.gravity = vector![0.0, -512.0];
    commands.spawn_bundle(LightBundle {
        transform: Transform::from_translation(Vec3::new(1000.0, 100.0, 2000.0)),
        ..Default::default()
    });
}

fn enable_physics_profiling(mut pipeline: ResMut<PhysicsPipeline>) {
    pipeline.counters.enable()
}

fn setup_bgm(asset_server: Res<AssetServer>, audio: Res<Audio>) {
    let music = asset_server.load("audio/MWH-BGM83.mp3");
    audio.play(music);
}
