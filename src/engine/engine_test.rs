use bevy::prelude::*;
use heron::{Body, BodyType, Gravity, PhysicsPlugin, Velocity};

use super::{event::window_event::WindowEventPlugin, plugin::fps::Fps};

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
            width: 960f32,
            height: 540f32,
            // 窗口模式
            // mode: WindowMode::BorderlessFullscreen,
            // 鼠标隐藏并锁定
            cursor_locked: true,
            cursor_visible: false,
            ..Default::default()
        })
        .add_resource(Msaa { samples: 8 })
        // 默认插件
        .add_plugins(DefaultPlugins) // 辅助功能插件
        .add_plugin(Fps) // 事件
        .add_plugin(WindowEventPlugin)
        // Default Bevy plugins.
        .add_plugin(PhysicsPlugin::default())
        .add_resource(Gravity::from(Vec3::new(0.0, -120.0, 0.0)))
        .add_startup_system(setup_physics.system())
        .run();
}

fn setup_physics(commands: &mut Commands, mut materials: ResMut<Assets<ColorMaterial>>) {
    commands
        .spawn(Camera2dBundle::default())
        .spawn(CameraUiBundle::default());

    let texture_handle = materials.add(Color::rgb(0.5, 0.5, 1.0).into());

    commands
        .spawn(SpriteBundle {
            material: texture_handle.clone(),
            sprite: Sprite::new(Vec2::new(100.0, 100.0)),
            transform: Transform::from_translation(Vec3::new(100.0, 400.0, 0.0)),
            ..Default::default()
        })
        .with(Body::Cuboid {
            half_extends: Vec3::new(50.0, 50.0, 0.0),
        })
        .with(BodyType::Dynamic);

    commands
        .spawn(SpriteBundle {
            material: texture_handle.clone(),
            sprite: Sprite::new(Vec2::new(100.0, 100.0)),
            transform: Transform::from_translation(Vec3::new(100.0, 300.0, 0.0)),
            ..Default::default()
        })
        .with(Body::Cuboid {
            half_extends: Vec3::new(50.0, 50.0, 0.0),
        })
        .with(BodyType::Dynamic);

    commands
        .spawn(SpriteBundle {
            material: texture_handle.clone(),
            sprite: Sprite::new(Vec2::new(100.0, 100.0)),
            transform: Transform::from_translation(Vec3::new(100.0, 200.0, 0.0)),
            ..Default::default()
        })
        .with(Body::Cuboid {
            half_extends: Vec3::new(50.0, 50.0, 0.0),
        })
        .with(BodyType::Dynamic);
    // .with(Gravity::from(Vec3::new(0.0, -9.81, 0.0)))
    // .with(Velocity::from(Vec2::new(30.0, 0.0)));

    commands
        .spawn(SpriteBundle {
            material: materials.add(Color::rgb(1.0, 1.0, 1.0).into()),
            sprite: Sprite::new(Vec2::new(100.0, 100.0)),
            transform: Transform::from_translation(Vec3::new(100.0, -225.0, 0.0)),
            ..Default::default()
        })
        .with(Body::Cuboid {
            half_extends: Vec3::new(50.0, 50.0, 0.0),
        })
        .with(BodyType::Static);
}
