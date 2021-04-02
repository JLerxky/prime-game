use bevy::{prelude::*, winit::WinitPlugin};
use bevy_rapier2d::{
    na::Vector2,
    physics::{RapierConfiguration, RapierPhysicsPlugin},
    rapier::pipeline::PhysicsPipeline,
    render::RapierRenderPlugin,
};
// use protocol::Packet;

use super::{event::{control_event::ControlEventPlugin, keyboard_event::KeyboardEventPlugin, map_event::MapEventPlugin}, plugin::{
        camera_ctrl::CameraCtrl,
        clipboard::Clipboard,
        fps::Fps,
        network::{NetWorkState, NetworkPlugin},
        tile_map::TileMapPlugin,
    }};

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
        .add_plugin(ControlEventPlugin)
        .add_plugin(KeyboardEventPlugin)
        .add_plugin(MapEventPlugin)
        // .add_plugin(WindowEventPlugin)
        // 地图初始化
        .add_plugin(TileMapPlugin)
        // 玩家
        // .add_plugin(PlayerPlugin)
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
        .add_system(animate_system.system())
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

fn animate_system(
    time: Res<Time>,
    mut animate_entity_query: Query<
        (&mut Timer, &mut TextureAtlasSprite, &Handle<TextureAtlas>),
        With<SynEntity>,
    >,
    texture_atlases: Res<Assets<TextureAtlas>>,
) {
    for (mut timer, mut sprite, texture_atlas_handle) in animate_entity_query.iter_mut() {
        timer.tick(time.delta_seconds());
        if timer.finished() {
            if let Some(texture_atlas) = texture_atlases.get(texture_atlas_handle) {
                sprite.index = ((sprite.index as usize + 1) % texture_atlas.textures.len()) as u32;
            }
        }
    }
}

fn network_synchronization(
    commands: &mut Commands,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    asset_server: Res<AssetServer>,
    window: Res<WindowDescriptor>,
    net: ResMut<NetWorkState>,
    mut syn_entity_query: Query<(&mut SynEntity, &mut Transform)>,
) {
    // println!("1");
    if let Ok(mut update_data_list) = net.update_data_list.lock() {
        if update_data_list.is_empty() {
            return;
        }
        let update_data = update_data_list[0].clone();
        update_data_list.remove(0);
        // println!("2");
        'update_data: for rigid_body_state in update_data.states {
            for (syn_entity, mut transform) in syn_entity_query.iter_mut() {
                // println!("3");
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
                    continue 'update_data;
                }
            }
            // println!("4");

            let texture_handle = asset_server
                .load(format!("textures/chars/{}.png", rigid_body_state.texture.0).as_str());
            // let tile_size = Vec2::new(100.0, 120.0);
            let tile_size = Vec2::new(window.width / 21f32 * 2f32, window.height / 13f32 * 4f32);
            let texture_atlas = TextureAtlas::from_grid(
                texture_handle,
                tile_size,
                rigid_body_state.texture.1.into(),
                1,
            );
            let texture_atlas_handle = texture_atlases.add(texture_atlas);
            commands
                .spawn(SpriteSheetBundle {
                    texture_atlas: texture_atlas_handle,
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
                .with(Timer::from_seconds(0.1, true))
                .with(SynEntity {
                    id: rigid_body_state.id.into(),
                });
        }
    }
}

struct SynEntity {
    pub id: u128,
}
