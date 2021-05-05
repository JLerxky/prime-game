use bevy::prelude::*;

use super::network_plugin::SynEntity;

pub struct AnimatePlugin;

impl Plugin for AnimatePlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system(animate_system.system());
    }
}

fn animate_system(
    time: Res<Time>,
    mut animate_entity_query: Query<
        (
            &mut Timer,
            &mut TextureAtlasSprite,
            &Handle<TextureAtlas>,
            &mut SynEntity,
        ),
        With<SynEntity>,
    >,
    texture_atlases: Res<Assets<TextureAtlas>>,
) {
    for (mut timer, mut sprite, texture_atlas_handle, mut syn_entity) in
        animate_entity_query.iter_mut()
    {
        if syn_entity.animate_type == 0 {
            continue;
        }
        timer.tick(time.delta());
        if timer.finished() {
            if let Some(texture_atlas) = texture_atlases.get(texture_atlas_handle) {
                // 特定动画组(玩家)
                if syn_entity.entity_type == 1 {
                    // 默认不动
                    let mut animate_list: Vec<u32> = [0].to_vec();
                    match syn_entity.animate_type {
                        // 走-前
                        1 => {
                            animate_list = [0, 4, 4, 0, 8, 8].to_vec();
                        }
                        // 走-后
                        2 => {
                            animate_list = [2, 6, 6, 2, 10, 10].to_vec();
                        }
                        // 走-右
                        3 => {
                            animate_list = [1, 5, 5, 1, 9, 9].to_vec();
                        }
                        // 走-左
                        4 => {
                            animate_list = [3, 7, 7, 3, 11, 11].to_vec();
                        }
                        _ => {}
                    }

                    next_animate(animate_list, &mut sprite, &mut syn_entity.animate_index);
                } else {
                    sprite.index = (sprite.index + 1) % texture_atlas.textures.len() as u32;
                }
            }
        }
    }
}

fn next_animate(
    animate_list: Vec<u32>,
    sprite: &mut Mut<TextureAtlasSprite>,
    animate_index: &mut usize,
) {
    *animate_index += 2;
    if *animate_index >= animate_list.len() {
        *animate_index = 0;
    }
    sprite.index = animate_list[*animate_index];
}
