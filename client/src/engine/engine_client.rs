use bevy::{core::FixedTimestep, prelude::*, winit::WinitPlugin};
use bevy_rapier2d::{
    na::Vector2,
    physics::{RapierConfiguration, RapierPhysicsPlugin},
    rapier::pipeline::PhysicsPipeline,
    render::RapierRenderPlugin,
};
// use protocol::Packet;

use super::{
    event::{
        keyboard_event::KeyboardEventPlugin, map_event::MapEventPlugin,
        window_event::WindowEventPlugin,
    },
    plugin::{
        camera_ctrl::CameraCtrl,
        clipboard::Clipboard,
        fps::Fps,
        network::{NetWorkState, NetworkPlugin},
        player::PlayerPlugin,
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
            width: 800f32,
            height: 450f32,
            // 窗口模式
            // mode: WindowMode::BorderlessFullscreen,
            // 鼠标隐藏并锁定
            // cursor_locked: true,
            // cursor_visible: false,
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
        // 设置摄像机
        .add_startup_system(set_camera.system())
        // 辅助功能插件
        .add_plugin(Fps)
        .add_plugin(Clipboard)
        // 事件
        .add_plugin(KeyboardEventPlugin)
        .add_plugin(MapEventPlugin)
        // .add_plugin(WindowEventPlugin)
        // 地图初始化
        .add_plugin(TileMapPlugin)
        // 玩家
        .add_plugin(PlayerPlugin)
        // 网络
        .add_plugin(NetworkPlugin)
        .add_system(network_synchronization.system())
        // .add_stage_after(
        //     stage::UPDATE,
        //     "network_synchronization_fixed_update",
        //     SystemStage::parallel()
        //         .with_run_criteria(
        //             FixedTimestep::step(1.0 / 1.0)
        //                 .with_label("network_synchronization_fixed_timestep"),
        //         )
        //         .with_system(network_synchronization.system()),
        // )
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

fn network_synchronization(
    commands: &mut Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut net: ResMut<NetWorkState>,
    mut syn_entity_query: Query<(&mut SynEntity, &mut Transform)>,
) {
    // println!("1");
    if let Ok(mut rb_states) = net.rb_states.lock() {
        let rb_states_c = rb_states.clone();
        rb_states.clear();
        // println!("2");
        'states: for rigid_body_state in rb_states_c.iter() {
            // println!("3");
            for (syn_entity, mut transform) in syn_entity_query.iter_mut() {
                // println!("4");
                if syn_entity.id == rigid_body_state.id.into() {
                    *transform = Transform {
                        translation: Vec3::new(
                            rigid_body_state.translation.0,
                            rigid_body_state.translation.1,
                            99.0,
                        ),
                        rotation: Quat::from([
                            rigid_body_state.rotation.0,
                            rigid_body_state.rotation.1,
                            0.0,
                            0.0,
                        ]),
                        scale: Vec3::new(1., 1., 1.),
                    };
                    continue 'states;
                }
            }
            // println!("5");
            commands
                .spawn(SpriteBundle {
                    material: materials.add(Color::rgb(0.0, 0.0, 0.8).into()),
                    sprite: Sprite::new(Vec2::new(50.0, 50.0)),
                    transform: Transform {
                        translation: Vec3::new(
                            rigid_body_state.translation.0,
                            rigid_body_state.translation.1,
                            99.0,
                        ),
                        rotation: Quat::from([
                            rigid_body_state.rotation.0,
                            rigid_body_state.rotation.1,
                            0.0,
                            0.0,
                        ]),
                        scale: Vec3::new(1., 1., 1.),
                    },
                    ..Default::default()
                })
                .with(SynEntity {
                    id: rigid_body_state.id.into(),
                });
        }
    }
}

struct SynEntity {
    pub id: u128,
}
