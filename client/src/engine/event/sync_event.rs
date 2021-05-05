use std::time::SystemTime;

use bevy::prelude::*;
use protocol::data::update_data::UpdateData;

use crate::engine::plugin::{
    camera_ctrl_plugin::CameraCtrl,
    network_plugin::{SynEntity, UID},
};

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
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    asset_server: Res<AssetServer>,
    mut syn_entity_query: Query<(&mut SynEntity, &mut Transform), Without<CameraCtrl>>,
    mut camera_query: Query<(&mut Transform, &CameraCtrl), Without<SynEntity>>,
    // mut sync_event_writer: EventWriter<SyncEvent>,
) {
    for sync_event in sync_event_reader.iter() {
        let update_data = sync_event.update_data.clone();
        let health_now = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        'update_data: for rigid_body_state in update_data.states {
            for (mut syn_entity, mut transform) in syn_entity_query.iter_mut() {
                // println!("3");
                if syn_entity.id == rigid_body_state.id.into() {
                    *transform = Transform {
                        translation: Vec3::new(
                            rigid_body_state.translation.0,
                            rigid_body_state.translation.1,
                            50.0,
                        ),
                        rotation: Quat::from_rotation_z(rigid_body_state.rotation),
                        scale: transform.scale,
                    };
                    syn_entity.health = health_now;
                    syn_entity.animate_type = rigid_body_state.animate;
                    unsafe {
                        if rigid_body_state.entity_type == 1 && UID == rigid_body_state.id as u32 {
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
            let mut texture_handle = asset_server.load("textures/chars/0.png");
            let mut tile_size = Vec2::new(64f32, 64f32);

            match rigid_body_state.entity_type {
                // tile
                0 => {
                    texture_handle = asset_server
                        .load(format!("textures/tile/{}.png", rigid_body_state.texture.0).as_str());
                }
                // 玩家实体
                1 => {
                    texture_handle = asset_server.load(
                        format!("textures/prime/char/{}.png", rigid_body_state.texture.0).as_str(),
                    );
                    tile_size = Vec2::new(16f32, 17f32);
                }
                // 可动实体
                2 => {
                    texture_handle = asset_server.load(
                        format!("textures/prime/char/{}.png", rigid_body_state.texture.0).as_str(),
                    );
                    tile_size = Vec2::new(tile_size.x * 1f32, tile_size.y * 2f32);
                }
                // 不可动实体
                3 => {
                    texture_handle = asset_server.load(
                        format!("textures/unmovable/{}.png", rigid_body_state.texture.0).as_str(),
                    );
                }
                // 其它
                _ => {}
            }

            let scale = Vec3::new(64f32 / tile_size.x, 64f32 / tile_size.y, 0.);

            let texture_atlas = TextureAtlas::from_grid(
                texture_handle,
                tile_size,
                rigid_body_state.texture.1.into(),
                rigid_body_state.texture.2.into(),
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
