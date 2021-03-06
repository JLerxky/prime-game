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
    let texture_handle = asset_server.load("textures/chars/gabe-idle-run.png");
    let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(24.0, 24.0), 7, 1);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    commands
        .spawn(SpriteSheetBundle {
            texture_atlas: texture_atlas_handle,
            transform: Transform::from_scale(Vec3::splat(2.0)),
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
    player_transform.translation += delta * player.velocity;
}

fn animate_system(
    time: Res<Time>,
    texture_atlases: Res<Assets<TextureAtlas>>,
    mut query: Query<(&mut Timer, &mut TextureAtlasSprite, &Handle<TextureAtlas>)>,
) {
    for (mut timer, mut sprite, texture_atlas_handle) in query.iter_mut() {
        timer.tick(time.delta_seconds());
        if timer.finished() {
            let texture_atlas = texture_atlases.get(texture_atlas_handle).unwrap();
            sprite.index = ((sprite.index as usize + 1) % texture_atlas.textures.len()) as u32;
        }
    }
}

fn player_ctrl_system(
    diagnostics: Res<Diagnostics>,
    mut query: Query<&mut Text, With<PlayerPlugin>>,
) {
}
