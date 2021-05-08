use bevy_tilemap::prelude::*;
use protocol::{
    data::tile_map_data::{Tile, TileData},
    packet::Packet,
    route::GameRoute,
};

use bevy::{asset::LoadState, prelude::*, sprite::TextureAtlasBuilder};

use super::network_plugin::NetWorkState;

const CHUNK_WIDTH: u32 = 16;
const CHUNK_HEIGHT: u32 = 16;
const TILEMAP_WIDTH: i32 = CHUNK_WIDTH as i32 * 100;
const TILEMAP_HEIGHT: i32 = CHUNK_HEIGHT as i32 * 100;

pub struct TileMapPlugin;

#[derive(Debug, Hash, PartialEq, Eq, Clone, StageLabel)]
struct UpdateTileFixedUpdateStage;

#[derive(Debug, Hash, PartialEq, Eq, Clone, StageLabel)]
struct CleanMapFixedUpdateStage;

impl Plugin for TileMapPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.init_resource::<TileMapState>()
            .init_resource::<TileSpriteHandles>()
            .add_plugins(TilemapDefaultPlugins)
            .add_startup_system(setup.system())
            .add_system(load.system())
            .add_system(build.system());
    }
}

#[derive(Default, Clone)]
struct TileMapState {
    map_loaded: bool,
    spawned: bool,
}

#[derive(Default, Clone)]
struct TileSpriteHandles {
    handles: Vec<HandleUntyped>,
    atlas_loaded: bool,
}

fn get_tile(point: IVec3, net_state: &ResMut<NetWorkState>) -> Option<Tile> {
    if let Ok(tile) = data::client_db::find_tile_map(point) {
        return Some(tile);
    } else {
        if let Ok(mut to_be_sent_queue) = net_state.to_be_sent_queue.lock() {
            to_be_sent_queue.push(Packet::Game(GameRoute::Tile(TileData {
                point: (point.x, point.y, point.z),
                tile: None,
            })));
        }
    }
    None
}

fn setup(mut tile_sprite_handles: ResMut<TileSpriteHandles>, asset_server: Res<AssetServer>) {
    tile_sprite_handles.handles = asset_server.load_folder("textures/prime/tiles").unwrap();
}

fn load(
    mut commands: Commands,
    mut sprite_handles: ResMut<TileSpriteHandles>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut textures: ResMut<Assets<Texture>>,
    asset_server: Res<AssetServer>,
) {
    if sprite_handles.atlas_loaded {
        return;
    }

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
            .dimensions(TILEMAP_WIDTH as u32, TILEMAP_HEIGHT as u32)
            .chunk_dimensions(CHUNK_WIDTH, CHUNK_HEIGHT, 1)
            .texture_dimensions(64, 64)
            .auto_chunk()
            .auto_spawn(2, 2)
            .add_layer(
                TilemapLayer {
                    kind: LayerKind::Dense,
                    ..Default::default()
                },
                0,
            )
            .texture_atlas(atlas_handle)
            .finish()
            .unwrap();

        let tilemap_components = TilemapBundle {
            tilemap,
            visible: Visible {
                is_visible: true,
                is_transparent: true,
            },
            transform: Transform {
                translation: Vec3::new(-32., -32., 0.),
                ..Default::default()
            },
            global_transform: Default::default(),
        };
        commands
            .spawn()
            .insert_bundle(tilemap_components)
            .insert(Timer::from_seconds(0.075, true));

        sprite_handles.atlas_loaded = true;
    }
}

fn build(
    mut map_state: ResMut<TileMapState>,
    texture_atlases: Res<Assets<TextureAtlas>>,
    asset_server: Res<AssetServer>,
    mut query: Query<&mut Tilemap>,
    net_state: ResMut<NetWorkState>,
) {
    if map_state.map_loaded {
        return;
    }

    for mut map in query.iter_mut() {
        let texture_atlas = texture_atlases.get(map.texture_atlas()).unwrap();
        let mut tiles = Vec::new();

        let min_x = -TILEMAP_WIDTH / 2;
        let max_x = TILEMAP_WIDTH / 2;
        let min_y = -TILEMAP_HEIGHT / 2;
        let max_y = TILEMAP_HEIGHT / 2;

        for x in min_x..=max_x {
            for y in min_y..=max_y {
                let point = IVec3::new(x as i32, y as i32, 1i32);
                let tile_point = (x, y);

                if let Some(tile) = get_tile(point, &net_state) {
                    // 若最上层也为泥地则不创建精灵
                    if tile.filename.eq("0-tileset_30.png") {
                        continue;
                    }

                    let tile_sprite: Handle<Texture> = asset_server
                        .get_handle(format!("textures/prime/tiles/{}", tile.filename).as_str());
                    let tile_idx = texture_atlas.get_texture_index(&tile_sprite).unwrap();

                    let tile = bevy_tilemap::tile::Tile {
                        point: tile_point,
                        sprite_index: tile_idx,
                        ..Default::default()
                    };
                    tiles.push(tile);
                }
            }
        }

        println!("{}", map.tile_height());
        map.insert_tiles(tiles).unwrap();
        map_state.map_loaded = true;
    }
}
