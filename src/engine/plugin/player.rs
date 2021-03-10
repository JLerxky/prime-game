use bevy::prelude::*;

use crate::engine::event::map_event::MapEvent;

use super::camera_ctrl::CameraCtrl;
pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(setup.system())
            .add_system(player_ctrl_system.system())
            .add_system(animate_system.system())
            .add_system(player_move_system.system());
    }
}

pub struct Player {
    pub velocity: Vec3,
    pub show_size: Vec2,
}

fn setup(
    commands: &mut Commands,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    asset_server: Res<AssetServer>,
) {
    let texture_handle = asset_server.load("textures/chars/player.png");
    let tile_size = Vec2::new(100.0, 120.0);
    let scale = 1f32;
    let texture_atlas = TextureAtlas::from_grid(texture_handle, tile_size, 1, 1);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    commands
        .spawn(SpriteSheetBundle {
            texture_atlas: texture_atlas_handle,
            transform: Transform {
                translation: Vec3::new(0.0, 0.0, 1.0),
                scale: Vec3::splat(scale),
                ..Default::default()
            },
            ..Default::default()
        })
        .with(Player {
            velocity: Vec3::new(0f32, 0f32, 0f32),
            show_size: tile_size * scale,
        })
        .with(Timer::from_seconds(0.1, true));
}

fn player_move_system(
    time: Res<Time>,
    mut player_query: Query<(&mut Transform, &mut Player), With<Player>>,
    mut camera_transform_query: Query<&mut Transform, With<CameraCtrl>>,
    mut map_events: ResMut<Events<MapEvent>>,
    window: Res<WindowDescriptor>,
) {
    let delta = time.delta_seconds();
    let (mut player_transform, player) = player_query.iter_mut().next().unwrap();
    player_transform.translation += delta * player.velocity * 300f32;

    let mut camera_transform = camera_transform_query.iter_mut().next().unwrap();

    // 屏幕可见范围偏移量
    let w = (window.width / 2f32) - (3f32 * (player.show_size.x / 2f32));
    let h = (window.height / 2f32) - (3f32 * (player.show_size.y / 2f32));

    if player_transform.translation.x >= (camera_transform.translation.x + w)
        || player_transform.translation.x <= (camera_transform.translation.x - w)
        || player_transform.translation.y >= (camera_transform.translation.y + h)
        || player_transform.translation.y <= (camera_transform.translation.y - h)
    {
        camera_transform.translation += delta * player.velocity * 300f32;
        map_events.send(MapEvent::Add);
    }
}

fn animate_system(
    time: Res<Time>,
    mut player_query: Query<
        (
            &Player,
            &mut Timer,
            &mut TextureAtlasSprite,
            &Handle<TextureAtlas>,
        ),
        With<Player>,
    >,
    texture_atlases: Res<Assets<TextureAtlas>>,
) {
    let (player, mut timer, mut sprite, texture_atlas_handle) =
        player_query.iter_mut().next().unwrap();
    if player.velocity.distance(Vec3::new(0.0, 0.0, 0.0)) != 0f32 {
        timer.tick(time.delta_seconds());
        if timer.finished() {
            let texture_atlas = texture_atlases.get(texture_atlas_handle).unwrap();
            sprite.index = ((sprite.index as usize + 1) % texture_atlas.textures.len()) as u32;
        }
    } else {
        sprite.index = 0u32;
    }
}

fn player_ctrl_system(// diagnostics: Res<Diagnostics>,
    // mut query: Query<&mut Text, With<PlayerPlugin>>,
) {
}
