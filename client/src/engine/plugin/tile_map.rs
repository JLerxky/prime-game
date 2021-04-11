use bevy::{asset::LoadState, prelude::*, sprite::TextureAtlasBuilder, utils::HashSet};
use bevy_tilemap::prelude::*;

use super::camera_ctrl::CameraCtrl;

pub struct TileMapPlugin;

impl Plugin for TileMapPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.init_resource::<SpriteHandles>()
            .init_resource::<TileMapState>()
            .add_plugins(TilemapDefaultPlugins)
            .add_startup_system(setup.system())
            .add_system(load.system())
            .add_system(build_map.system())
            .add_system(update_map.system());
    }
}

#[derive(Default, Clone)]
struct SpriteHandles {
    handles: Vec<HandleUntyped>,
    atlas_loaded: bool,
}

#[derive(Default, Clone)]
struct TileMapState {
    map_loaded: bool,
    spawned: bool,
    collisions: HashSet<(i32, i32)>,
}

fn setup(mut tile_sprite_handles: ResMut<SpriteHandles>, asset_server: Res<AssetServer>) {
    tile_sprite_handles.handles = asset_server.load_folder("textures/tile_map").unwrap();
}

fn load(
    mut commands: Commands,
    mut sprite_handles: ResMut<SpriteHandles>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut textures: ResMut<Assets<Texture>>,
    asset_server: Res<AssetServer>,
) {
    if sprite_handles.atlas_loaded {
        return;
    }

    // Lets load all our textures from our folder!
    let mut texture_atlas_builder = TextureAtlasBuilder::default();
    if let LoadState::Loaded =
        asset_server.get_group_load_state(sprite_handles.handles.iter().map(|handle| handle.id))
    {
        for handle in sprite_handles.handles.iter() {
            let texture = textures.get(handle).unwrap();
            texture_atlas_builder.add_texture(handle.clone_weak().typed::<Texture>(), &texture);
        }

        let texture_atlas = texture_atlas_builder.finish(&mut textures).unwrap();
        let atlas_handle = texture_atlases.add(texture_atlas);

        let tilemap = Tilemap::builder()
            .auto_chunk()
            .topology(GridTopology::Square)
            .chunk_dimensions(2, 2, 1)
            .texture_dimensions(32, 32)
            .z_layers(10)
            .texture_atlas(atlas_handle)
            .finish()
            .unwrap();
        let tilemap_components = TilemapBundle {
            tilemap,
            visible: Visible {
                is_visible: true,
                is_transparent: true,
            },
            transform: Default::default(),
            global_transform: Default::default(),
        };

        commands
            .spawn()
            .insert_bundle(OrthographicCameraBundle::new_2d());
        commands
            .spawn()
            .insert_bundle(tilemap_components)
            .insert(Timer::from_seconds(0.075, true));

        sprite_handles.atlas_loaded = true;
    }
}

fn build_map(
    mut tile_map_state: ResMut<TileMapState>,
    texture_atlases: Res<Assets<TextureAtlas>>,
    asset_server: Res<AssetServer>,
    mut tilemap_query: Query<&mut Tilemap, Without<CameraCtrl>>,
    // camera_query: Query<&Transform, With<CameraCtrl>>,
    window: Res<WindowDescriptor>,
) {
    if tile_map_state.map_loaded {
        return;
    }

    if let Ok(mut map) = tilemap_query.single_mut() {
        let width = window.width as i32 / 64i32 + 5;
        let height = window.height as i32 / 64i32 + 5;
        let chunk_width = width * map.chunk_width() as i32;
        let chunk_height = height * map.chunk_height() as i32;

        // let center_point = (0, 0);

        // if let Ok(camera_transform) = camera_query.single() {
        //     center_point = map.point_to_chunk_point((
        //         camera_transform.translation.x as i32,
        //         camera_transform.translation.y as i32,
        //     ));
        // }

        let floor: Handle<Texture> = asset_server.get_handle("textures/tile_map/square-floor.png");
        let texture_atlas = texture_atlases.get(map.texture_atlas()).unwrap();
        let floor_index = texture_atlas.get_texture_index(&floor).unwrap();

        let mut tiles = Vec::new();
        for y in 0..chunk_height {
            for x in 0..chunk_width {
                let y = y - chunk_height / 2;
                let x = x - chunk_width / 2;
                let tile = Tile {
                    point: (x, y),
                    sprite_index: floor_index,
                    ..Default::default()
                };
                tiles.push(tile);
            }
        }
        map.insert_tiles(tiles).unwrap();

        for x in -width / 2..=width / 2 {
            for y in -height / 2..=height / 2 {
                // if y == 0 || x == 0 {
                let _ = map.spawn_chunk((x, y));
                // }
            }
        }

        tile_map_state.map_loaded = true;
    }
}

fn update_map(
    texture_atlases: Res<Assets<TextureAtlas>>,
    asset_server: Res<AssetServer>,
    mut tilemap_query: Query<&mut Tilemap, Without<CameraCtrl>>,
    camera_query: Query<&Transform, With<CameraCtrl>>,
    window: Res<WindowDescriptor>,
) {
    if let Ok(mut map) = tilemap_query.single_mut() {
        let width = window.width as i32 / 64i32 + 3;
        let height = window.height as i32 / 64i32 + 3;
        let chunk_width = width * map.chunk_width() as i32;
        let chunk_height = height * map.chunk_height() as i32;

        let mut center_point = (0, 0);

        if let Ok(camera_transform) = camera_query.single() {
            center_point = map.point_to_chunk_point((
                camera_transform.translation.x as i32,
                camera_transform.translation.y as i32,
            ));
        }

        let floor: Handle<Texture> = asset_server.get_handle("textures/tile_map/square-floor.png");
        let texture_atlas = texture_atlases.get(map.texture_atlas()).unwrap();
        let floor_index = texture_atlas.get_texture_index(&floor).unwrap();

        let mut tiles = Vec::new();
        for y in 0..chunk_height {
            for x in 0..chunk_width {
                let y = y - chunk_height / 2 + center_point.0;
                let x = x - chunk_width / 2 + center_point.1;
                // if (x - center_point.0).abs() > width || (y - center_point.1).abs() > height {
                //     if map.contains_chunk((x, y)) {
                //         let _ = map.remove_chunk((x, y));
                //         continue;
                //     }
                // } else {
                if map.contains_chunk((x, y)) {
                    continue;
                }
                let tile = Tile {
                    point: (x, y),
                    sprite_index: floor_index,
                    ..Default::default()
                };
                tiles.push(tile);
                // }
            }
        }
        map.insert_tiles(tiles).unwrap();

        for x in -width / 2..=width / 2 {
            for y in -height / 2..=height / 2 {
                if y == 0 || x == 0 {
                    let _ = map.spawn_chunk((x + center_point.0, y + center_point.1));
                }
            }
        }
    }
}
