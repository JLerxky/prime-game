use std::time::SystemTime;

use bevy::prelude::*;
use bevy_rapier2d::{
    na::Vector2,
    physics::RigidBodyHandleComponent,
    rapier::dynamics::{RigidBodyBuilder, RigidBodySet},
};
use protocol::data::{
    player_data::PlayerData,
    update_data::{EntityType, UpdateData},
};

use crate::plugin::network_plugin::{SynEntity, PLAYER};

#[derive(Debug, Hash, PartialEq, Eq, Clone, StageLabel)]
struct CheckEntityHealthFixedUpdateStage;

pub struct SyncEventPlugin;

impl Plugin for SyncEventPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_event::<SyncEvent>()
            .add_system(event_listener_system.system());
    }
}

pub struct SyncEvent {
    pub update_data: UpdateData,
}

fn event_listener_system(
    mut sync_event_reader: EventReader<SyncEvent>,
    mut commands: Commands,
    mut syn_entity_query: Query<(&mut SynEntity, &RigidBodyHandleComponent)>,
    mut rigid_bodies: ResMut<RigidBodySet>,
    mut player_query: Query<&mut PlayerData>,
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
                            Vector2::new(rigid_body_state.linvel.0, rigid_body_state.linvel.1),
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
                        }
                    }

                    continue 'update_data;
                }
            }

            // 未生成的实体根据实体类型生成新实体
            let tile_size = Vec2::new(64f32, 64f32);

            let mut scale = Vec3::new(64f32 / tile_size.x, 64f32 / tile_size.y, 0.);

            match rigid_body_state.entity_type {
                EntityType::Skill => {
                    scale = Vec3::new(1., 1., 0.);
                }
                _ => {}
            }

            commands
                .spawn_bundle(SpriteSheetBundle {
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
                .insert(RigidBodyBuilder::new_dynamic())
                .insert(Timer::from_seconds(1., true))
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
