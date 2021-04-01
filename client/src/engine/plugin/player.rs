use bevy::prelude::*;
use bevy_rapier2d::{
    na::Vector2,
    physics::{RapierConfiguration, RigidBodyHandleComponent},
    rapier::{
        dynamics::{RigidBodyBuilder, RigidBodySet},
        geometry::ColliderBuilder,
    },
};

use crate::engine::event::map_event::MapEvent;

use super::camera_ctrl::CameraCtrl;
pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(setup.system())
            .add_system(player_ctrl_system.system())
            .add_system(animate_system.system())
            .add_system(player_movement.system());
    }
}

pub struct Player {
    pub uid: u32,
    pub velocity: Vec3,
    pub show_size: Vec2,
    pub jumped: bool,
}

fn setup(
    commands: &mut Commands,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    asset_server: Res<AssetServer>,
    window: Res<WindowDescriptor>,
) {
    let texture_handle = asset_server.load("textures/chars/player.png");
    // let tile_size = Vec2::new(100.0, 120.0);
    let tile_size = Vec2::new(window.width / 21f32 * 2f32, window.height / 13f32 * 2f32);
    let scale = 1f32;
    let texture_atlas = TextureAtlas::from_grid(texture_handle, tile_size, 8, 1);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    commands
        .spawn(SpriteSheetBundle {
            texture_atlas: texture_atlas_handle,
            transform: Transform {
                translation: Vec3::new(0.0, 220.0, 10.0),
                scale: Vec3::splat(scale),
                ..Default::default()
            },
            ..Default::default()
        })
        .with(
            RigidBodyBuilder::new_dynamic()
                .gravity_scale(8.0)
                .lock_rotations(),
        )
        .with(ColliderBuilder::capsule_y(tile_size.y / 4.0 - 5f32, tile_size.x / 4.0).friction(0.0))
        .with(Player {
            uid: 0,
            velocity: Vec3::new(40f32, 12000f32, 0f32),
            show_size: tile_size * scale,
            jumped: false,
        })
        .with(Timer::from_seconds(0.1, true));
}

fn player_movement(
    keyboard_input: Res<Input<KeyCode>>,
    rapier_parameters: Res<RapierConfiguration>,
    mut rigid_bodies: ResMut<RigidBodySet>,
    mut player_info: Query<(&mut Player, &Transform, &RigidBodyHandleComponent)>,
    mut camera_query: Query<(&CameraCtrl, &mut Transform)>,
    // window: Res<WindowDescriptor>,
    mut map_events: ResMut<Events<MapEvent>>,
) {
    if let Some((_camera_ctrl, mut camera_transform)) = camera_query.iter_mut().next() {
        for (player, _player_transform, rigid_body_component) in player_info.iter_mut() {
            // player_transform.translation.y =

            let x_axis = -(keyboard_input.pressed(KeyCode::A) as i8)
                + (keyboard_input.pressed(KeyCode::D) as i8);
            let y_axis = keyboard_input.pressed(KeyCode::Space) as i8;

            // if !player.jumped {

            let mut move_delta = Vector2::new(x_axis as f32, y_axis as f32);
            if move_delta != Vector2::zeros() {
                move_delta /= move_delta.magnitude() * rapier_parameters.scale;
            }

            if let Some(rb) = rigid_bodies.get_mut(rigid_body_component.handle()) {
                // println!("{:?}", rb.linvel());
                if rb.linvel().y == 0f32 && !player.jumped {
                    rb.set_linvel(
                        Vector2::new(
                            move_delta.x * player.velocity.x,
                            move_delta.y * player.velocity.y,
                        ),
                        true,
                    );
                // player.jumped = true;
                } else {
                    rb.set_linvel(Vector2::new(move_delta.x * player.velocity.x, 0f32), true);
                }
                let player_pos = rb.position().translation;
                let old_pos = camera_transform.translation.clone();
                camera_transform.translation = Vec3::new(
                    player_pos.x,
                    camera_transform.translation.y,
                    camera_transform.translation.z,
                );
                if old_pos.distance(camera_transform.translation) > 0f32 {
                    map_events.send(MapEvent::Add);
                }
            }
            // }

            // 屏幕可见范围偏移量
            // let w = (window.width / 2f32) - (3f32 * (player.show_size.x / 2f32));
            // let h = (window.height / 2f32) - (3f32 * (player.show_size.y / 2f32));

            // if player_transform.translation.x > (camera_transform.translation.x + w)
            //     || player_transform.translation.x < (camera_transform.translation.x - w)
            //     || player_transform.translation.y > (camera_transform.translation.y + h)
            //     || player_transform.translation.y < (camera_transform.translation.y - h)
            // {
            //     camera_transform.translation +=
            //         Vec3::new(move_delta.x, move_delta.y, 0.0) * player.velocity.x;
            // }
        }
    }
}

fn animate_system(
    time: Res<Time>,
    mut rigid_bodies: ResMut<RigidBodySet>,
    mut player_query: Query<
        (
            &RigidBodyHandleComponent,
            &mut Timer,
            &mut TextureAtlasSprite,
            &Handle<TextureAtlas>,
        ),
        With<Player>,
    >,
    texture_atlases: Res<Assets<TextureAtlas>>,
) {
    if let Some((player_rb, mut timer, mut sprite, texture_atlas_handle)) =
        player_query.iter_mut().next()
    {
        if let Some(rb) = rigid_bodies.get_mut(player_rb.handle()) {
            if rb.linvel().x > 0.001f32
                || rb.linvel().y > 0.001f32
                || rb.linvel().x < -0.001f32
                || rb.linvel().y < -0.001f32
            {
                timer.tick(time.delta_seconds());
                if timer.finished() {
                    if let Some(texture_atlas) = texture_atlases.get(texture_atlas_handle) {
                        sprite.index =
                            ((sprite.index as usize + 1) % texture_atlas.textures.len()) as u32;
                    }
                }
            } else {
                sprite.index = 0u32;
            }
        }
    }
    // if player.velocity.distance(Vec3::new(0.0, 0.0, 0.0)) != 0f32 {
    //     timer.tick(time.delta_seconds());
    //     if timer.finished() {
    //         let texture_atlas = texture_atlases.get(texture_atlas_handle).unwrap();
    //         sprite.index = ((sprite.index as usize + 1) % texture_atlas.textures.len()) as u32;
    //     }
    // } else {
    //     sprite.index = 0u32;
    // }
}

fn player_ctrl_system(// diagnostics: Res<Diagnostics>,
    // mut query: Query<&mut Text, With<PlayerPlugin>>,
) {
}
