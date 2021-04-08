use bevy::{prelude::*, winit::WinitPlugin};
use bevy_rapier2d::{
    na::Vector2,
    physics::{RapierConfiguration, RapierPhysicsPlugin},
    rapier::pipeline::PhysicsPipeline,
    render::RapierRenderPlugin,
};
// use protocol::Packet;

use super::{
    event::{
        control_event::ControlEventPlugin, keyboard_event::KeyboardEventPlugin,
        map_event::MapEventPlugin,
    },
    plugin::{
        camera_ctrl::CameraCtrl,
        clipboard::Clipboard,
        fps::Fps,
        network::{NetWorkState, NetworkPlugin, UID},
        tile_map::TileMapPlugin,
    },
};

pub fn engine_start() {
    App::build()
        .insert_resource(WindowDescriptor {
            title: String::from("初始游戏"),
            // 垂直同步
            vsync: true,
            // 是否可调整窗口大小
            resizable: false,
            // 是否有窗口外壳
            decorations: true,
            width: 1600f32,
            height: 900f32,
            // 窗口模式
            // mode: WindowMode::BorderlessFullscreen,
            // 鼠标隐藏并锁定
            // cursor_locked: true,
            // cursor_visible: false,
            ..Default::default()
        })
        .insert_resource(Msaa { samples: 8 })
        // 窗口背景色
        .insert_resource(ClearColor(Color::rgb(0.5, 0.5, 0.9)))
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

fn set_camera(mut commands: Commands) {
    commands
        // cameras
        .spawn_bundle(OrthographicCameraBundle::new_2d())
        .insert(CameraCtrl);
    commands.spawn_bundle(UiCameraBundle::default());
    // .insert(CameraCtrl);
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

fn network_synchronization(
    mut commands: Commands,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    asset_server: Res<AssetServer>,
    window: Res<WindowDescriptor>,
    net: ResMut<NetWorkState>,
    mut syn_entity_query: Query<(&mut SynEntity, &mut Transform), Without<CameraCtrl>>,
    mut camera_query: Query<(&mut Transform, &CameraCtrl)>,
) {
    // println!("1");
    if let Ok(mut update_data_list) = net.update_data_list.lock() {
        // 同步服务器发来的状态
        if !update_data_list.is_empty() {
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
                            rotation: Quat::from_rotation_z(rigid_body_state.rotation),
                            scale: Vec3::new(1., 1., 1.),
                        };
                        unsafe {
                            if rigid_body_state.entity_type == 1
                                && UID == rigid_body_state.id as u32
                            {
                                if let Some((mut camera_transform, _)) =
                                    camera_query.iter_mut().next()
                                {
                                    camera_transform.translation = Vec3::new(
                                        rigid_body_state.translation.0,
                                        rigid_body_state.translation.1,
                                        99.0,
                                    );
                                }
                            }
                        }
                        continue 'update_data;
                    }
                }
                // println!("4");

                // 未生成的实体根据实体类型生成新实体
                let mut texture_handle = asset_server.load("textures/chars/0.png");
                let mut tile_size =
                    Vec2::new(window.width / 21f32 * 1f32, window.height / 13f32 * 1f32);

                match rigid_body_state.entity_type {
                    // tile
                    0 => {
                        texture_handle = asset_server.load(
                            format!("textures/tile/{}.png", rigid_body_state.texture.0).as_str(),
                        );
                    }
                    // 玩家实体
                    1 => {
                        texture_handle = asset_server.load(
                            format!("textures/chars/{}.png", rigid_body_state.texture.0).as_str(),
                        );
                        tile_size *= 2f32;
                    }
                    // 可动实体
                    2 => {
                        texture_handle = asset_server.load(
                            format!("textures/movable/{}.png", rigid_body_state.texture.0).as_str(),
                        );
                        tile_size =
                            Vec2::new(window.width / 21f32 * 0.5f32, window.height / 13f32 * 1f32);
                    }
                    // 不可动实体
                    3 => {
                        texture_handle = asset_server.load(
                            format!("textures/unmovable/{}.png", rigid_body_state.texture.0)
                                .as_str(),
                        );
                    }
                    // 其它
                    _ => {}
                }

                let scale = Vec3::new(1., 1., 0.);

                let texture_atlas = TextureAtlas::from_grid(
                    texture_handle,
                    tile_size,
                    rigid_body_state.texture.1.into(),
                    1,
                );
                let texture_atlas_handle = texture_atlases.add(texture_atlas);
                commands
                    .spawn_bundle(SpriteSheetBundle {
                        texture_atlas: texture_atlas_handle,
                        transform: Transform {
                            translation: Vec3::new(
                                rigid_body_state.translation.0,
                                rigid_body_state.translation.1,
                                99.0,
                            ),
                            rotation: Quat::from_rotation_z(rigid_body_state.rotation),
                            scale,
                        },
                        ..Default::default()
                    })
                    .insert(Timer::from_seconds(0.1, true))
                    .insert(SynEntity {
                        id: rigid_body_state.id.into(),
                    });
            }
        }
        // 向服务器发送玩家操作
        // println!("1");
        // if let Ok(mut control_queue) = net.control_queue.lock() {
        //     // println!("2");
        //     let control_queue_c = control_queue.clone();
        //     control_queue.clear();
        //     for control_data in control_queue_c.iter() {
        //         // println!("3");
        //         let engine_tx = net.engine_tx.clone();
        //         let control_data = control_data.clone();
        //         tokio::spawn(async move {
        //             let _ = engine_tx
        //                 .send(Packet::Game(GameRoute::Control(control_data)))
        //                 .await;
        //         });
        //     }
        // }
    }
}

struct SynEntity {
    pub id: u128,
}
