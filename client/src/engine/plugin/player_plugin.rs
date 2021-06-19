use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use data::client_db::save_player;
use protocol::data::player_data::{PlayerData, PlayerListData};

use crate::engine::plugin::network_plugin::{SynEntity, PLAYER};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(setup.system())
            .add_event::<PlayerUpdateEvent>()
            .add_system(event_listener_system.system())
            .add_system(player_ctrl_system.system())
            .add_system(animate_system.system())
            .add_system(player_movement.system());
    }
}

pub struct PlayerUpdateEvent {
    pub player_list_data: PlayerListData,
}

fn event_listener_system(
    mut event_reader: EventReader<PlayerUpdateEvent>,
    mut player_bar_query: Query<(&mut PlayerData, &mut Transform), With<PlayerData>>,
) {
    for event in event_reader.iter() {
        'player: for player_data in &event.player_list_data.players {
            let _ = save_player(player_data.clone());
            for (mut old_player_data, mut transform) in player_bar_query.iter_mut() {
                if player_data.uid == old_player_data.uid {
                    *old_player_data = *player_data;
                    // println!("{:?}", &player_data);
                    let blood_len = 12. * (player_data.hp as f32 / player_data.max_hp as f32);
                    *transform = Transform {
                        translation: Vec3::new((blood_len / 2.) - 6., 12., 99.0),
                        scale: Vec3::new(blood_len / 4., 0.1, 0.),
                        ..Default::default()
                    };
                    continue 'player;
                }
            }
        }
    }
}

fn setup(
    mut commands: Commands,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let player_texture_index: u32 = rand::Rng::gen_range(&mut rand::thread_rng(), 1..24);
    let mut rigid_body_state = protocol::data::update_data::EntityState {
        id: 0,
        translation: (0., 0.),
        rotation: 0.,
        linvel: (0., 0.),
        angvel: (0., 0.),
        texture: (player_texture_index, 4, 3),
        entity_type: protocol::data::update_data::EntityType::Player,
        animate: 0,
    };
    unsafe {
        rigid_body_state.id = PLAYER.uid as u64;
    }
    let texture_handle = asset_server
        .load(format!("textures/prime/char/{}.png", rigid_body_state.texture.0).as_str());
    let tile_size = Vec2::new(16f32, 17f32);
    let scale = Vec3::new(64f32 / tile_size.x, 64f32 / tile_size.y, 0.);
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
            .load("textures/rpg/2d misc/prehistoric-platformer/hud/health-bar-backgound.png")
            .into(),
    );
    let blood_handle = materials.add(
        asset_server
            .load("textures/rpg/2d misc/prehistoric-platformer/hud/bar-middle.png")
            .into(),
    );
    let mut player = protocol::data::player_data::PlayerData {
        uid: rigid_body_state.id as u32,
        hp: 0,
        mp: 0,
        max_hp: 100,
        max_mp: 100,
    };
    if let Ok(player_db) = data::client_db::find_player(rigid_body_state.id as u32) {
        player = player_db;
    }
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
            bevy_rapier2d::rapier::dynamics::RigidBodyBuilder::new_dynamic().translation(vector![
                rigid_body_state.translation.0,
                rigid_body_state.translation.1
            ]),
        )
        .with_children(|parent| {
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
        })
        .insert(Timer::from_seconds(0.1, true))
        .insert(SynEntity {
            id: rigid_body_state.id,
            entity_type: rigid_body_state.entity_type,
            health: std::time::SystemTime::now()
                .duration_since(std::time::SystemTime::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            animate_type: rigid_body_state.animate,
            animate_index: 0,
        })
        .insert(player);
}

fn player_movement() {}

fn animate_system() {}

fn player_ctrl_system() {}
