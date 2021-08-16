use bevy::prelude::*;
use bevy_rapier2d::{
    na::Vector2,
    physics::{RapierConfiguration, RapierPhysicsPlugin},
};

use crate::{
    event::sync_event::SyncEventPlugin,
    plugin::network_plugin::{NetworkPlugin, SynEntity},
};

pub fn engine_start() {
    App::build()
        .add_plugins(MinimalPlugins)
        .add_plugin(bevy::log::LogPlugin::default())
        .add_plugin(bevy::diagnostic::DiagnosticsPlugin::default())
        .add_plugin(TransformPlugin::default())
        // 物理
        .add_plugin(RapierPhysicsPlugin)
        .add_startup_system(setup_graphics.system())
        // 网络
        .add_plugin(SyncEventPlugin)
        .add_plugin(NetworkPlugin)
        // 日志输出
        .add_plugin(bevy::diagnostic::LogDiagnosticsPlugin::default())
        .add_system(log_entity.system())
        // 实体数量
        // .add_plugin(bevy::diagnostic::EntityCountDiagnosticsPlugin::default())
        .run();
}

fn setup_graphics(mut commands: Commands, mut rapier_config: ResMut<RapierConfiguration>) {
    rapier_config.gravity = Vector2::new(0., 0.);
    commands.spawn_bundle(LightBundle {
        transform: Transform::from_translation(Vec3::new(1000.0, 100.0, 2000.0)),
        ..Default::default()
    });
}

fn log_entity(time: Res<Time>, mut entity_query: Query<(&mut Timer, &Transform), With<SynEntity>>) {
    for (mut timer, transform) in entity_query.iter_mut() {
        timer.tick(time.delta());
        if timer.finished() {
            println!(
                "实体: ({}, {})",
                transform.translation.x, transform.translation.y
            );
        }
    }
}