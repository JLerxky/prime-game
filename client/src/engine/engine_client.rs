use bevy::{prelude::*, winit::WinitPlugin};
use bevy_rapier2d::{
    na::Vector2,
    physics::{RapierConfiguration, RapierPhysicsPlugin},
    rapier::pipeline::PhysicsPipeline,
    render::RapierRenderPlugin,
};
use common::{GameEvent, UpdateData};

use super::{event::{
        keyboard_event::KeyboardEventPlugin, map_event::MapEventPlugin,
        window_event::WindowEventPlugin,
    }, plugin::{camera_ctrl::CameraCtrl, clipboard::Clipboard, fps::Fps, network::{NetworkPlugin, SenderState}, player::PlayerPlugin, tile_map::TileMapPlugin}};

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
        .add_resource(Msaa { samples: 4 })
        // 默认插件
        .add_plugins(DefaultPlugins)
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
        // 网络
        .add_plugin(NetworkPlugin)
        .add_system(test_network.system())
        // 设置摄像机
        .add_startup_system(set_camera.system())
        // 辅助功能插件
        .add_plugin(Fps)
        .add_plugin(Clipboard)
        // 事件
        .add_plugin(KeyboardEventPlugin)
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

fn setup_graphics(commands: &mut Commands, mut rapier_config: ResMut<RapierConfiguration>) {
    // configuration.scale = 40.0;

    rapier_config.gravity = Vector2::new(0.0, -512.0);
    commands.spawn(LightBundle {
        transform: Transform::from_translation(Vec3::new(1000.0, 100.0, 2000.0)),
        ..Default::default()
    });
}

fn enable_physics_profiling(mut pipeline: ResMut<PhysicsPipeline>) {
    pipeline.counters.enable()
}

fn test_network(net: Res<SenderState>) {
    let _ = net.engine_tx.try_send(GameEvent::Update(UpdateData {
        id: 21,
        translation: [1., 1.],
        rotation: [0., 0.],
    }));
}
