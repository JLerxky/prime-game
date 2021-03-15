use crate::data::game_db::{self, GameData};
use bevy::{core::FixedTimestep, prelude::*};
use bevy_rapier2d::{
    na::Vector2,
    physics::{RapierConfiguration, RapierPhysicsPlugin},
    rapier::{dynamics::RigidBodyBuilder, geometry::ColliderBuilder, pipeline::PhysicsPipeline},
};

use super::plugin::player::Player;

pub fn engine_start() {
    App::build()
        // 默认插件
        // .add_plugins(DefaultPlugins)
        .add_plugins(MinimalPlugins)
        // 物理插件
        .add_plugin(RapierPhysicsPlugin)
        .add_startup_system(setup_graphics.system())
        // 地图初始化
        .add_startup_system(setup_tile_map.system())
        // 事件
        // .add_plugin(MapEventPlugin)
        // 地图初始化
        // .add_plugin(TileMapPlugin)
        // 玩家
        // .add_plugin(PlayerPlugin)
        // 玩家注册
        .add_stage_after(
            stage::UPDATE,
            "player_register_fixed_update",
            SystemStage::parallel()
                .with_run_criteria(
                    FixedTimestep::step(2.0).with_label("player_register_fixed_timestep"),
                )
                .with_system(player_register_system.system()),
        )
        // 帧同步
        .add_stage_after(
            stage::UPDATE,
            "sync_data_fixed_update",
            SystemStage::parallel()
                .with_run_criteria(FixedTimestep::step(2.0).with_label("sync_data_fixed_timestep"))
                .with_system(sync_data_system.system()),
        )
        .run();
}

fn setup_graphics(
    mut rapier_config: ResMut<RapierConfiguration>,
    mut pipeline: ResMut<PhysicsPipeline>,
) {
    // rapier_config.scale = 40.0;

    rapier_config.gravity = Vector2::new(0.0, -1.0);
    pipeline.counters.enable()
}

fn player_register_system(commands: &mut Commands, player_list: Query<&Player>) {
    match game_db::find(GameData {
        table: "player".to_string(),
        key: "online".to_string(),
        data: None,
    }) {
        Some(data) => {
            let uid_list: Vec<u32> = data
                .split(",")
                .map(|uid| uid.parse::<u32>().unwrap())
                .collect();
            for uid in uid_list {
                let mut exist = false;
                for player in player_list.iter() {
                    if uid == player.uid {
                        exist = true;
                        break;
                    }
                }
                if exist {
                    break;
                } else {
                    commands
                        .spawn(SpriteSheetBundle {
                            transform: Transform {
                                scale: Vec3::splat(1f32),
                                ..Default::default()
                            },
                            ..Default::default()
                        })
                        .with(
                            RigidBodyBuilder::new_dynamic()
                                .translation(0.0, 0.0)
                                .gravity_scale(1.0)
                                .lock_rotations(),
                        )
                        .with(ColliderBuilder::capsule_y(11f32, 16f32).friction(0.0))
                        .with(Player {
                            uid,
                            velocity: Vec3::new(40f32, 12000f32, 0f32),
                            show_size: Vec2::new(64f32, 64f32),
                            jumped: false,
                        })
                        .with(Timer::from_seconds(0.1, true));
                }
            }
        }
        None => {}
    }
    println!("在线人数: {}", player_list.iter().count());
}

fn setup_tile_map(commands: &mut Commands) {
    // 生成地图
    let tile_center = Vec3::new(0.0, 0.0, 0.0);
    let tile_size = Vec2::new(64f32, 64f32);
    println!("瓷砖大小: {:?}", tile_size);
    // 长21格，高13格
    let x_size: i32 = 21i32;
    let y_size: i32 = 13i32;

    // 波函数坍缩生成场景
    // let slots = wave_func_collapse(
    //     Vec3::new(0.0, 0.0, 0.0),
    //     Vec3::new(add_x as f32, add_y as f32, 0f32),
    //     tile_size,
    // );

    /*
     * The ground
     */
    commands
        .spawn(SpriteBundle {
            sprite: Sprite::new(Vec2::new(300.0, 10.0)),
            ..Default::default()
        })
        .with(RigidBodyBuilder::new_static().translation(400.0, -tile_size.y / 2f32))
        .with(ColliderBuilder::cuboid(150.0, 5.0).friction(0.0));

    commands
        .spawn(SpriteBundle {
            sprite: Sprite::new(Vec2::new(300.0, 10.0)),
            ..Default::default()
        })
        .with(RigidBodyBuilder::new_static().translation(0.0, -tile_size.y / 2f32 * 3f32 + 1f32))
        .with(ColliderBuilder::cuboid(150.0, 5.0).friction(0.0));

    // commands
    //     .spawn(SpriteBundle {
    //         material: materials.add(Color::rgb(0.0, 0.0, 0.0).into()),
    //         sprite: Sprite::new(Vec2::new(300.0, 10.0)),
    //         transform: Transform::from_translation(Vec3::new(0.0, tile_size.y, 10.0)),
    //         ..Default::default()
    //     })
    //     .with(RigidBodyBuilder::new_static())
    //     .with(ColliderBuilder::cuboid(150.0, 5.0));

    // 天空背景
    for x in -x_size / 2..=x_size / 2 {
        let x_pos = x as f32 * tile_size.x;
        for y in -y_size / 2..=y_size / 2 {
            for z in 0..=1 {
                let tile_position =
                    Vec3::new(x_pos, y as f32 * tile_size.y, z as f32 * 10f32) + tile_center;

                // let slot_option = slots.get(&vec3_to_key(tile_position));
                // if let Some(slot) = slot_option {
                //     if let Some(tile) = slot.tile {
                //         texture_handle = materials.add(
                //             asset_server
                //                 .load(format!("textures/tiles/{}.png", tile.id).as_str())
                //                 .into(),
                //         );
                //     }
                // }
                if y <= -2 && y >= -6 && z >= 1 {
                    let rigid_body = RigidBodyBuilder::new_static()
                        .translation(tile_position.x, tile_position.y);
                    let collider = ColliderBuilder::cuboid(tile_size.x / 2f32, tile_size.y / 2f32);

                    commands
                        .spawn(SpriteBundle {
                            sprite: Sprite::new(tile_size),
                            transform: Transform::from_translation(tile_position),
                            ..Default::default()
                        })
                        .with(rigid_body)
                        .with(collider.friction(0.0));
                }
            }
        }
    }
}

fn sync_data_system(player_query: Query<(&Player, &Transform)>) {
    for (player, player_transform) in player_query.iter() {
        println!(
            "玩家: {}, 位置: {}",
            player.uid, player_transform.translation
        );
    }
}
