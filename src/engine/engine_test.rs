use bevy::{prelude::*, wgpu::WgpuPlugin, winit::WinitPlugin};
use bevy_rapier2d::rapier::dynamics::{RigidBodyBuilder, RigidBodySet};
use bevy_rapier2d::rapier::geometry::ColliderBuilder;
use bevy_rapier2d::rapier::na::Vector2;
use bevy_rapier2d::{
    physics::{RapierConfiguration, RapierPhysicsPlugin, RigidBodyHandleComponent},
    render::RapierRenderPlugin,
};

pub fn engine_start() {
    App::build()
        .add_resource(ClearColor(Color::rgb(
            0xF9 as f32 / 255.0,
            0xF9 as f32 / 255.0,
            0xFF as f32 / 255.0,
        )))
        .add_resource(Msaa { samples: 4 })
        .add_plugins(DefaultPlugins)
        .add_plugin(WinitPlugin::default())
        .add_plugin(WgpuPlugin::default())
        .add_plugin(RapierPhysicsPlugin)
        .add_plugin(RapierRenderPlugin)
        .add_startup_system(spawn_player.system())
        .add_system(player_movement.system())
        .run();
}

struct Player(f32);
struct CameraCtrl;

fn spawn_player(
    commands: &mut Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut rapier_config: ResMut<RapierConfiguration>,
) {
    rapier_config.gravity = Vector2::zeros();
    commands.spawn(Camera2dBundle::default()).with(CameraCtrl);

    let sprite_size_x = 40.0;
    let sprite_size_y = 40.0;

    rapier_config.scale = 20.0;
    let collider_size_x = sprite_size_x / rapier_config.scale;
    let collider_size_y = sprite_size_y / rapier_config.scale;

    /*
     * The ground
     */
    let ground_size = 5.0;
    let ground_height = 0.1;

    let rigid_body = RigidBodyBuilder::new_static().translation(0.0, -ground_height);
    let collider = ColliderBuilder::cuboid(ground_size, ground_height);
    commands.spawn((rigid_body, collider));

    commands
        .spawn(SpriteBundle {
            material: materials.add(Color::rgb(0.0, 0.0, 0.0).into()),
            sprite: Sprite::new(Vec2::new(sprite_size_x, sprite_size_y)),
            ..Default::default()
        })
        .with(RigidBodyBuilder::new_dynamic())
        .with(ColliderBuilder::cuboid(
            collider_size_x / 2.0,
            collider_size_y / 2.0,
        ))
        .with(Player(300.0));
}

fn player_movement(
    keyboard_input: Res<Input<KeyCode>>,
    rapier_parameters: Res<RapierConfiguration>,
    mut rigid_bodies: ResMut<RigidBodySet>,
    player_info: Query<(&Player, &Transform, &RigidBodyHandleComponent)>,
    mut camera_query: Query<(&CameraCtrl, &mut Transform)>,
) {
    let (_camera_ctrl, mut camera_transform) = camera_query.iter_mut().next().unwrap();
    for (player, player_transform, rigid_body_component) in player_info.iter() {
        let x_axis = -(keyboard_input.pressed(KeyCode::A) as i8)
            + (keyboard_input.pressed(KeyCode::D) as i8);
        let y_axis = -(keyboard_input.pressed(KeyCode::S) as i8)
            + (keyboard_input.pressed(KeyCode::W) as i8);

        let mut move_delta = Vector2::new(x_axis as f32, y_axis as f32);
        if move_delta != Vector2::zeros() {
            move_delta /= move_delta.magnitude() * rapier_parameters.scale;
        }

        if let Some(rb) = rigid_bodies.get_mut(rigid_body_component.handle()) {
            rb.set_linvel(move_delta * player.0, true);
        }

        if player_transform.translation.x >= (camera_transform.translation.x + 300.0)
            || player_transform.translation.x <= (camera_transform.translation.x - 300.0)
            || player_transform.translation.y >= (camera_transform.translation.y + 300.0)
            || player_transform.translation.y <= (camera_transform.translation.y - 300.0)
        {
            camera_transform.translation += Vec3::new(move_delta.x, move_delta.y, 0.0) * player.0;
        }
    }
}
