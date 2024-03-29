use std::time::SystemTime;

use bevy::{core::FixedTimestep, prelude::*};
use bevy_rapier2d::{
    physics::RigidBodyHandleComponent,
    rapier::dynamics::{RigidBodyBuilder, RigidBodySet},
};
use protocol::data::{
    player_data::PlayerData,
    update_data::{EntityType, UpdateData},
};

use crate::engine::plugin::{
    camera_ctrl_plugin::CameraCtrl,
    network_plugin::{SynEntity, PLAYER},
};

#[derive(Debug, Hash, PartialEq, Eq, Clone, StageLabel)]
struct CheckEntityHealthFixedUpdateStage;

pub struct SyncEventPlugin;

impl Plugin for SyncEventPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_event::<SyncEvent>()
            .add_system(event_listener_system.system())
            .add_stage_after(
                CoreStage::Update,
                CheckEntityHealthFixedUpdateStage,
                SystemStage::parallel()
                    .with_run_criteria(
                        FixedTimestep::step(100.).with_label("build_map_fixed_timestep"),
                    )
                    .with_system(check_entity_health.system()),
            );
    }
}

pub struct SyncEvent {
    pub update_data: UpdateData,
}

fn check_entity_health(
    mut commands: Commands,
    syn_entity_query: Query<(&SynEntity, Entity), Without<CameraCtrl>>,
) {
    let health_now = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_secs();
    for (syn_entity, entity) in syn_entity_query.iter() {
        if health_now > (syn_entity.health + 1) {
            commands.entity(entity).despawn_recursive();
        }
    }
}

fn event_listener_system(
    mut sync_event_reader: EventReader<SyncEvent>,
    mut commands: Commands,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
    mut syn_entity_query: Query<(&mut SynEntity, &RigidBodyHandleComponent), Without<CameraCtrl>>,
    mut camera_query: Query<(&mut Transform, &CameraCtrl), Without<SynEntity>>,
    audio: Res<Audio>,
    mut rigid_bodies: ResMut<RigidBodySet>,
    mut player_query: Query<&mut PlayerData>,
    // mut sync_event_writer: EventWriter<SyncEvent>,
) {
    for sync_event in sync_event_reader.iter() {
        let health_now = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        'update_data: for rigid_body_state in &sync_event.update_data.states {
            for (mut syn_entity, rb_handle) in syn_entity_query.iter_mut() {
                if syn_entity.entity_type == rigid_body_state.entity_type
                    && syn_entity.id == rigid_body_state.id
                {
                    syn_entity.health = health_now;
                    syn_entity.animate_type = rigid_body_state.animate;

                    if let Some(rb) = rigid_bodies.get_mut(rb_handle.handle()) {
                        rb.set_linvel(
                            [rigid_body_state.linvel.0, rigid_body_state.linvel.1].into(),
                            true,
                        );
                        rb.set_angvel(rigid_body_state.angvel.0, true);
                        let mut pos = rb.position().clone();

                        if (pos.translation.x - rigid_body_state.translation.0).abs() > 10. {
                            pos.translation.x = rigid_body_state.translation.0;
                        }
                        if (pos.translation.y - rigid_body_state.translation.1).abs() > 10. {
                            pos.translation.y = rigid_body_state.translation.1;
                        }

                        rb.set_position(pos, true);
                    }

                    unsafe {
                        if rigid_body_state.entity_type == EntityType::Player
                            && PLAYER.uid == rigid_body_state.id as u32
                        {
                            if let Ok(mut player_state) = player_query.single_mut() {
                                player_state.uid = rigid_body_state.id as u32;
                            }

                            if let Some((mut camera_transform, _)) = camera_query.iter_mut().next()
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

            // 未生成的实体根据实体类型生成新实体
            let texture_handle;
            let mut tile_size = Vec2::new(64f32, 64f32);

            match rigid_body_state.entity_type {
                // tile
                EntityType::Static => {
                    texture_handle = asset_server
                        .load(format!("textures/tile/{}.png", rigid_body_state.texture.0).as_str());
                }
                // 玩家实体
                EntityType::Player => {
                    texture_handle = asset_server.load(
                        format!("textures/prime/char/{}.png", rigid_body_state.texture.0).as_str(),
                    );
                    tile_size = Vec2::new(16f32, 17f32);
                }
                // 可动实体
                EntityType::Moveable => {
                    texture_handle = asset_server.load(
                        format!("textures/prime/char/{}.png", rigid_body_state.texture.0).as_str(),
                    );
                    tile_size = Vec2::new(tile_size.x * 1f32, tile_size.y * 2f32);
                }
                EntityType::Trap => {
                    texture_handle = asset_server.load(
                        format!("textures/prime/fx/{}.png", rigid_body_state.texture.0).as_str(),
                    );
                }
                EntityType::Skill => {
                    texture_handle = asset_server.load(
                        format!("textures/prime/fx/{}.png", rigid_body_state.texture.0).as_str(),
                    );
                    tile_size = Vec2::new(32f32, 32f32);
                    let music = asset_server.load("audio/MWH-skill05.mp3");
                    audio.play(music);
                }
            }

            let mut scale = Vec3::new(64f32 / tile_size.x, 64f32 / tile_size.y, 0.);

            match rigid_body_state.entity_type {
                EntityType::Skill => {
                    scale = Vec3::new(1., 1., 0.);
                }
                _ => {}
            }

            let texture_atlas = TextureAtlas::from_grid(
                texture_handle,
                tile_size,
                rigid_body_state.texture.1.into(),
                rigid_body_state.texture.2.into(),
            );
            let texture_atlas_handle = texture_atlases.add(texture_atlas);
            let blood_box_handle = materials.add(
                asset_server
                    .load("textures/rpg/2d misc/prehistoric-platformer/hud/health-bar-top-1.png")
                    .into(),
            );
            let blood_backgound_handle = materials.add(
                asset_server
                    .load(
                        "textures/rpg/2d misc/prehistoric-platformer/hud/health-bar-backgound.png",
                    )
                    .into(),
            );
            let blood_handle = materials.add(
                asset_server
                    .load("textures/rpg/2d misc/prehistoric-platformer/hud/bar-middle.png")
                    .into(),
            );
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
                .insert(
                    RigidBodyBuilder::new_dynamic().translation(
                        [
                            rigid_body_state.translation.0,
                            rigid_body_state.translation.1,
                        ]
                        .into(),
                    ),
                )
                .with_children(|parent| {
                    if rigid_body_state.entity_type == EntityType::Player {
                        // 血条背景
                        parent.spawn_bundle(SpriteBundle {
                            material: blood_backgound_handle,
                            transform: Transform {
                                translation: Vec3::new(0., 12., 99.0),
                                scale: Vec3::new(0.1, 0.1, 0.),
                                ..Default::default()
                            },
                            ..Default::default()
                        });
                        // 血量值
                        let mut player = PlayerData {
                            uid: rigid_body_state.id as u32,
                            hp: 0,
                            mp: 0,
                            max_hp: 100,
                            max_mp: 100,
                        };
                        if let Ok(player_db) =
                            data::client_db::find_player(rigid_body_state.id as u32)
                        {
                            player = player_db;
                        }
                        let blood_len = 12. * (player.hp as f32 / player.max_hp as f32);
                        parent
                            .spawn_bundle(SpriteBundle {
                                material: blood_handle,
                                transform: Transform {
                                    translation: Vec3::new((blood_len / 2.) - 6., 12., 99.0),
                                    scale: Vec3::new(blood_len / 4., 0.1, 0.),
                                    ..Default::default()
                                },
                                ..Default::default()
                            })
                            .insert(player);
                        // 血量框
                        parent.spawn_bundle(SpriteBundle {
                            material: blood_box_handle,
                            transform: Transform {
                                translation: Vec3::new(0., 12., 99.0),
                                scale: Vec3::new(0.1, 0.1, 0.),
                                ..Default::default()
                            },
                            ..Default::default()
                        });
                    }
                })
                .insert(Timer::from_seconds(0.1, true))
                .insert(SynEntity {
                    id: rigid_body_state.id,
                    entity_type: rigid_body_state.entity_type,
                    health: SystemTime::now()
                        .duration_since(SystemTime::UNIX_EPOCH)
                        .unwrap()
                        .as_secs(),
                    animate_type: rigid_body_state.animate,
                    animate_index: 0,
                });
        }
    }
}
