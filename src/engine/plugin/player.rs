use bevy::{diagnostic::Diagnostics, prelude::*};
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
}

fn setup(
    commands: &mut Commands,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    asset_server: Res<AssetServer>,
) {
    let texture_handle = asset_server.load("textures/chars/slime-orange.png");
    let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(12.0, 24.0), 4, 1);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    commands
        .spawn(SpriteSheetBundle {
            texture_atlas: texture_atlas_handle,
            transform: Transform::from_scale(Vec3::splat(6.0)),
            ..Default::default()
        })
        .with(Player {
            velocity: Vec3::new(0f32, 0f32, 0f32),
        })
        .with(Timer::from_seconds(0.1, true));
}

fn player_move_system(
    time: Res<Time>,
    mut player_query: Query<(&mut Transform, &mut Player), With<Player>>,
) {
    let delta = time.delta_seconds();
    let (mut player_transform, mut player) = player_query.iter_mut().next().unwrap();
    player_transform.translation += delta * player.velocity * 100f32;
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

fn player_ctrl_system(
    diagnostics: Res<Diagnostics>,
    mut query: Query<&mut Text, With<PlayerPlugin>>,
) {
}
