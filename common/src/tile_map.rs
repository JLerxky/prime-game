use std::collections::HashMap;

use glam::{IVec3, Vec3};
use protocol::data::tile_map_data::{Slot, Tile, TileCollider, TileJoint, TileMap};
use rand::Rng;
pub fn create_map(tile_map: &mut TileMap) {
    // 1. 计算地图边界值

    let min_x = tile_map.center_point.x - (tile_map.map_size.x as i32 / 2);
    let max_x = tile_map.center_point.x + (tile_map.map_size.x as i32 / 2);
    let min_y = tile_map.center_point.y - (tile_map.map_size.y as i32 / 2);
    let max_y = tile_map.center_point.y + (tile_map.map_size.y as i32 / 2);

    // 2. 按Z轴从小到大生成图层
    for z in 0..tile_map.map_size.z {
        for point_x in min_x..=max_x {
            for point_y in min_y..=max_y {
                let point = IVec3::new(point_x, point_y, z as i32);

                // 判断是否已初始化
                if tile_map.slot_map.contains_key(&point) {
                    continue;
                }

                // 初始化Slot: 填充叠加态, 初始化熵
                let superposition = load_default_superposition(z);
                let entropy = superposition.len();
                let slot = Slot {
                    point,
                    superposition,
                    entropy,
                    tile: None,
                };
                tile_map.slot_map.insert(point, slot);
            }
        }
    }
    // 4-2. 按照熵值从小到大坍缩
    // TODO 4-2-1. 填充当前地图块四周已坍缩的tile，以供计算边缘slot的叠加态与熵
    // 4-2-2. 递归坍缩
    tile_map.slot_map = collapse(tile_map.slot_map.clone());
}

// 世界坐标->地图索引
fn _pos_to_global_point(tile_map: &TileMap, pos: Vec3) -> IVec3 {
    let point =
        pos / ((tile_map.chunk_size * tile_map.texture_size).as_f32() / Vec3::new(2., 2., 1.));
    point.as_i32()
}

// 递归坍缩
fn collapse(mut slot_map: HashMap<IVec3, Slot>) -> HashMap<IVec3, Slot> {
    let mut slot_list: Vec<Slot> = Vec::new();

    // 取出当前所有未坍缩的slot
    for (_point, slot) in slot_map.iter() {
        if slot.entropy > 0 {
            slot_list.push(slot.clone());
        }
    }

    // 重新计算熵
    println!("{:?}", &slot_list.len());
    for slot in &mut slot_list {
        let superposition: Vec<Tile> = slot.superposition.clone();

        // 取得紧贴的slot连接限制条件tile_joint
        let mut joint_list = [
            TileJoint::All, // 0上
            TileJoint::All, // 1下
            TileJoint::All, // 2左
            TileJoint::All, // 3右
            TileJoint::All, // 4前
            TileJoint::All, // 5后
        ];
        // 上
        let point = IVec3::new(slot.point.x, slot.point.y + 1, slot.point.z);
        if let Some(slot) = slot_map.get(&point) {
            if let Some(tile) = &slot.tile {
                joint_list[0] = tile.joints[1].clone();
            }
        }
        // 下
        let point = IVec3::new(slot.point.x, slot.point.y - 1, slot.point.z);
        if let Some(slot) = slot_map.get(&point) {
            if let Some(tile) = &slot.tile {
                joint_list[1] = tile.joints[0].clone();
            }
        }
        // 左
        let point = IVec3::new(slot.point.x - 1, slot.point.y, slot.point.z);
        if let Some(slot) = slot_map.get(&point) {
            if let Some(tile) = &slot.tile {
                joint_list[2] = tile.joints[3].clone();
            }
        }
        // 右
        let point = IVec3::new(slot.point.x + 1, slot.point.y, slot.point.z);
        if let Some(slot) = slot_map.get(&point) {
            if let Some(tile) = &slot.tile {
                joint_list[3] = tile.joints[2].clone();
            }
        }
        // 前
        let point = IVec3::new(slot.point.x, slot.point.y, slot.point.z + 1);
        if let Some(slot) = slot_map.get(&point) {
            if let Some(tile) = &slot.tile {
                joint_list[4] = tile.joints[5].clone();
            }
        }
        // 后
        let point = IVec3::new(slot.point.x, slot.point.y, slot.point.z - 1);
        if let Some(slot) = slot_map.get(&point) {
            if let Some(tile) = &slot.tile {
                joint_list[5] = tile.joints[4].clone();
            }
        }

        // 剔除无效坍缩态
        let mut superposition_new = Vec::new();
        'tile: for tile in superposition.iter() {
            println!("-------------------{:?}", &tile.filename);
            for i in 0..6 as usize {
                match joint_list[i] {
                    TileJoint::None => {
                        superposition_new = Vec::new();
                        break 'tile;
                    }
                    TileJoint::TagOne(ref tag) => {
                        match &tile.joints[i] {
                            TileJoint::All => {}
                            TileJoint::None => {
                                continue 'tile;
                            }
                            TileJoint::TagOne(t_tag) => {
                                println!("{}:{:?}---{:?}:{}", i, tag, t_tag, i);
                                if tag.contains("空") && !tag.eq("空") {
                                    if !t_tag.eq("空") {
                                        continue 'tile;
                                    }
                                } else if tag.eq("空") {
                                    if !t_tag.contains("空") {
                                        continue 'tile;
                                    }
                                } else {
                                    if !t_tag.eq(tag) {
                                        continue 'tile;
                                    }
                                }
                            }
                        }
                        // continue 'tile;
                    }
                    _ => {}
                }
            }
            superposition_new.push(tile.clone());
        }

        // 更新slot
        slot.superposition = superposition_new;
        slot.entropy = slot.superposition.len();

        println!("{:?}", joint_list);
        println! {"{:?}-{:?}", &slot.point, &slot.entropy};
    }

    // 获取最小熵slot
    let mut min_entropy = usize::MAX;
    let mut min_slot = None;
    for slot in slot_list {
        if slot.entropy == 0 {
            slot_map.insert(slot.point, slot.clone());
            continue;
        }
        if slot.entropy < min_entropy && slot.entropy != 0 {
            min_entropy = slot.entropy;
            min_slot = Some(slot);
        }
    }

    // 执行slot坍缩
    if let Some(mut slot) = min_slot {
        let mut superposition_for_rng = Vec::new();
        for tile in &slot.superposition {
            for _ in 0..tile.rng_seed {
                superposition_for_rng.push(tile.clone());
            }
        }
        let i = rand::thread_rng().gen_range(0..superposition_for_rng.len());

        slot.tile = Some(superposition_for_rng[i].clone());
        slot.superposition = Vec::new();
        slot.entropy = 0;
        if let Some(_slot) = slot_map.insert(slot.point, slot.clone()) {
            println!("更新{:?}", slot);
        }
        // 判断是否完成坍缩, 完成则退出递归返回tile_map结果, 否则继续
        return collapse(slot_map);
    } else {
        return slot_map;
    }
}

#[test]
fn test_create_map() {
    use glam::UVec3;
    let mut tile_map = TileMap {
        center_point: IVec3::new(0, 0, 0),
        texture_size: UVec3::new(64, 64, 1),
        chunk_size: UVec3::new(1, 1, 1),
        map_size: UVec3::new(5, 5, 1),
        slot_map: HashMap::new(),
    };
    create_map(&mut tile_map);
    println!("{:?}", tile_map);
}

// 加载默认胶水tile初始化叠加态
pub fn load_glue_superposition() -> Vec<Tile> {
    let mut superposition = Vec::new();
    superposition.push(Tile {
        filename: "0-tileset_30.png".to_string(),
        layer: 0,
        rng_seed: 1,
        collider: TileCollider::Full,
        joints: [
            TileJoint::All, // 0上
            TileJoint::All, // 1下
            TileJoint::All, // 2左
            TileJoint::All, // 3右
            TileJoint::All, // 4前
            TileJoint::All, // 5后
        ],
    });
    superposition
}

pub fn load_default_superposition(layer: u32) -> Vec<Tile> {
    match layer {
        // 背景
        0 => load_background_superposition(layer as usize),
        // 地形
        1 => load_terrain_superposition(layer as usize),
        // 设施
        2 => load_item_superposition(layer as usize),
        _ => Vec::new(),
    }
}

pub fn load_background_superposition(layer: usize) -> Vec<Tile> {
    let mut superposition = Vec::new();
    superposition.push(Tile {
        filename: "0-tileset_30.png".to_string(),
        layer,
        rng_seed: 1,
        collider: TileCollider::Full,
        joints: [
            TileJoint::All, // 0上
            TileJoint::All, // 1下
            TileJoint::All, // 2左
            TileJoint::All, // 3右
            TileJoint::All, // 4前
            TileJoint::All, // 5后
        ],
    });
    superposition
}

// 地形
pub fn load_terrain_superposition(layer: usize) -> Vec<Tile> {
    let mut superposition = Vec::new();
    // 添加空地
    superposition.push(Tile {
        filename: "0-tileset_30.png".to_string(),
        layer,
        rng_seed: 40,
        collider: TileCollider::Full,
        joints: [
            TileJoint::TagOne("空".to_string()), // 0上
            TileJoint::TagOne("空".to_string()), // 1下
            TileJoint::TagOne("空".to_string()), // 2左
            TileJoint::TagOne("空".to_string()), // 3右
            TileJoint::All,                       // 4前
            TileJoint::All,                       // 5后
        ],
    });

    // 草地
    {
        superposition.push(Tile {
            filename: "0-tileset_01.png".to_string(),
            layer,
            rng_seed: 2,
            collider: TileCollider::Full,
            joints: [
                TileJoint::TagOne("草空".to_string()),    // 0上
                TileJoint::TagOne("x|边|草".to_string()), // 1下
                TileJoint::TagOne("草空".to_string()),    // 2左
                TileJoint::TagOne("y|边|草".to_string()), // 3右
                TileJoint::All,                             // 4前
                TileJoint::All,                             // 5后
            ],
        });
        superposition.push(Tile {
            filename: "0-tileset_02.png".to_string(),
            layer,
            rng_seed: 2,
            collider: TileCollider::Full,
            joints: [
                TileJoint::TagOne("草空".to_string()),    // 0上
                TileJoint::TagOne("草".to_string()),       // 1下
                TileJoint::TagOne("y|边|草".to_string()), // 2左
                TileJoint::TagOne("y|边|草".to_string()), // 3右
                TileJoint::All,                             // 4前
                TileJoint::All,                             // 5后
            ],
        });
        superposition.push(Tile {
            filename: "0-tileset_03.png".to_string(),
            layer,
            rng_seed: 2,
            collider: TileCollider::Full,
            joints: [
                TileJoint::TagOne("草空".to_string()),    // 0上
                TileJoint::TagOne("x|草|边".to_string()), // 1下
                TileJoint::TagOne("y|边|草".to_string()), // 2左
                TileJoint::TagOne("草空".to_string()),    // 3右
                TileJoint::All,                             // 4前
                TileJoint::All,                             // 5后
            ],
        });
        superposition.push(Tile {
            filename: "0-tileset_04.png".to_string(),
            layer,
            rng_seed: 2,
            collider: TileCollider::Full,
            joints: [
                TileJoint::TagOne("草".to_string()), // 0上
                TileJoint::TagOne("草".to_string()), // 1下
                TileJoint::TagOne("草".to_string()), // 2左
                TileJoint::TagOne("草".to_string()), // 3右
                TileJoint::All,                       // 4前
                TileJoint::All,                       // 5后
            ],
        });
        superposition.push(Tile {
            filename: "0-tileset_21.png".to_string(),
            layer,
            rng_seed: 2,
            collider: TileCollider::Full,
            joints: [
                TileJoint::TagOne("x|边|草".to_string()), // 0上
                TileJoint::TagOne("x|边|草".to_string()), // 1下
                TileJoint::TagOne("草空".to_string()),    // 2左
                TileJoint::TagOne("草".to_string()),       // 3右
                TileJoint::All,                             // 4前
                TileJoint::All,                             // 5后
            ],
        });
        superposition.push(Tile {
            filename: "0-tileset_23.png".to_string(),
            layer,
            rng_seed: 2,
            collider: TileCollider::Full,
            joints: [
                TileJoint::TagOne("x|草|边".to_string()), // 0上
                TileJoint::TagOne("x|草|边".to_string()), // 1下
                TileJoint::TagOne("草".to_string()),       // 2左
                TileJoint::TagOne("草空".to_string()),    // 3右
                TileJoint::All,                             // 4前
                TileJoint::All,                             // 5后
            ],
        });
        superposition.push(Tile {
            filename: "0-tileset_39.png".to_string(),
            layer,
            rng_seed: 2,
            collider: TileCollider::Full,
            joints: [
                TileJoint::TagOne("x|边|草".to_string()), // 0上
                TileJoint::TagOne("草空".to_string()),    // 1下
                TileJoint::TagOne("草空".to_string()),    // 2左
                TileJoint::TagOne("y|草|边".to_string()), // 3右
                TileJoint::All,                             // 4前
                TileJoint::All,                             // 5后
            ],
        });
        superposition.push(Tile {
            filename: "0-tileset_40.png".to_string(),
            layer,
            rng_seed: 2,
            collider: TileCollider::Full,
            joints: [
                TileJoint::TagOne("草".to_string()),       // 0上
                TileJoint::TagOne("草空".to_string()),    // 1下
                TileJoint::TagOne("y|草|边".to_string()), // 2左
                TileJoint::TagOne("y|草|边".to_string()), // 3右
                TileJoint::All,                             // 4前
                TileJoint::All,                             // 5后
            ],
        });
        superposition.push(Tile {
            filename: "0-tileset_41.png".to_string(),
            layer,
            rng_seed: 2,
            collider: TileCollider::Full,
            joints: [
                TileJoint::TagOne("x|草|边".to_string()), // 0上
                TileJoint::TagOne("草空".to_string()),    // 1下
                TileJoint::TagOne("y|草|边".to_string()), // 2左
                TileJoint::TagOne("草空".to_string()),    // 3右
                TileJoint::All,                             // 4前
                TileJoint::All,                             // 5后
            ],
        });
    }

    // 池塘
    {
        superposition.push(Tile {
            filename: "0-tileset_17.png".to_string(),
            layer,
            rng_seed: 1,
            collider: TileCollider::Full,
            joints: [
                TileJoint::TagOne("池空".to_string()),    // 0上
                TileJoint::TagOne("x|边|水".to_string()), // 1下
                TileJoint::TagOne("池空".to_string()),    // 2左
                TileJoint::TagOne("y|边|水".to_string()), // 3右
                TileJoint::None,                            // 4前
                TileJoint::None,                            // 5后
            ],
        });
        superposition.push(Tile {
            filename: "0-tileset_18.png".to_string(),
            layer,
            rng_seed: 1,
            collider: TileCollider::Full,
            joints: [
                TileJoint::TagOne("池空".to_string()),    // 0上
                TileJoint::TagOne("水".to_string()),       // 1下
                TileJoint::TagOne("y|边|水".to_string()), // 2左
                TileJoint::TagOne("y|边|水".to_string()), // 3右
                TileJoint::None,                            // 4前
                TileJoint::None,                            // 5后
            ],
        });
        superposition.push(Tile {
            filename: "0-tileset_19.png".to_string(),
            layer,
            rng_seed: 1,
            collider: TileCollider::Full,
            joints: [
                TileJoint::TagOne("池空".to_string()),    // 0上
                TileJoint::TagOne("x|水|边".to_string()), // 1下
                TileJoint::TagOne("y|边|水".to_string()), // 2左
                TileJoint::TagOne("池空".to_string()),    // 3右
                TileJoint::None,                            // 4前
                TileJoint::None,                            // 5后
            ],
        });
        superposition.push(Tile {
            filename: "0-tileset_36.png".to_string(),
            layer,
            rng_seed: 1,
            collider: TileCollider::Full,
            joints: [
                TileJoint::TagOne("x|边|水".to_string()), // 0上
                TileJoint::TagOne("x|边|水".to_string()), // 1下
                TileJoint::TagOne("池空".to_string()),    // 2左
                TileJoint::TagOne("水".to_string()),       // 3右
                TileJoint::None,                            // 4前
                TileJoint::None,                            // 5后
            ],
        });
        superposition.push(Tile {
            filename: "0-tileset_37.png".to_string(),
            layer,
            rng_seed: 1,
            collider: TileCollider::Full,
            joints: [
                TileJoint::TagOne("水".to_string()), // 0上
                TileJoint::TagOne("水".to_string()), // 1下
                TileJoint::TagOne("水".to_string()), // 2左
                TileJoint::TagOne("水".to_string()), // 3右
                TileJoint::None,                      // 4前
                TileJoint::None,                      // 5后
            ],
        });
        superposition.push(Tile {
            filename: "0-tileset_38.png".to_string(),
            layer,
            rng_seed: 1,
            collider: TileCollider::Full,
            joints: [
                TileJoint::TagOne("x|水|边".to_string()), // 0上
                TileJoint::TagOne("x|水|边".to_string()), // 1下
                TileJoint::TagOne("水".to_string()),       // 2左
                TileJoint::TagOne("池空".to_string()),    // 3右
                TileJoint::None,                            // 4前
                TileJoint::None,                            // 5后
            ],
        });
        superposition.push(Tile {
            filename: "0-tileset_54.png".to_string(),
            layer,
            rng_seed: 1,
            collider: TileCollider::Full,
            joints: [
                TileJoint::TagOne("x|边|水".to_string()), // 0上
                TileJoint::TagOne("池空".to_string()),    // 1下
                TileJoint::TagOne("池空".to_string()),    // 2左
                TileJoint::TagOne("y|水|边".to_string()), // 3右
                TileJoint::None,                            // 4前
                TileJoint::None,                            // 5后
            ],
        });
        superposition.push(Tile {
            filename: "0-tileset_55.png".to_string(),
            layer,
            rng_seed: 1,
            collider: TileCollider::Full,
            joints: [
                TileJoint::TagOne("水".to_string()),       // 0上
                TileJoint::TagOne("池空".to_string()),    // 1下
                TileJoint::TagOne("y|水|边".to_string()), // 2左
                TileJoint::TagOne("y|水|边".to_string()), // 3右
                TileJoint::None,                            // 4前
                TileJoint::None,                            // 5后
            ],
        });
        superposition.push(Tile {
            filename: "0-tileset_56.png".to_string(),
            layer,
            rng_seed: 1,
            collider: TileCollider::Full,
            joints: [
                TileJoint::TagOne("x|水|边".to_string()), // 0上
                TileJoint::TagOne("池空".to_string()),    // 1下
                TileJoint::TagOne("y|水|边".to_string()), // 2左
                TileJoint::TagOne("池空".to_string()),    // 3右
                TileJoint::None,                            // 4前
                TileJoint::None,                            // 5后
            ],
        });
    }

    // 泥地
    // {
    //     superposition.push(Tile {
    //         filename: "0-tileset_09.png".to_string(),
    //         layer,
    //         collider: TileCollider::Full,
    //         joints: [
    //             TileJoint::TagOne("边".to_string()),       // 0上
    //             TileJoint::TagOne("x|边|泥".to_string()), // 1下
    //             TileJoint::TagOne("边".to_string()),       // 2左
    //             TileJoint::TagOne("y|边|泥".to_string()), // 3右
    //             TileJoint::All,                             // 4前
    //             TileJoint::None,                            // 5后
    //         ],
    //     });
    //     superposition.push(Tile {
    //         filename: "0-tileset_10.png".to_string(),
    //         layer,
    //         collider: TileCollider::Full,
    //         joints: [
    //             TileJoint::TagOne("边".to_string()),       // 0上
    //             TileJoint::TagOne("泥".to_string()),       // 1下
    //             TileJoint::TagOne("y|边|泥".to_string()), // 2左
    //             TileJoint::TagOne("y|边|泥".to_string()), // 3右
    //             TileJoint::All,                             // 4前
    //             TileJoint::None,                            // 5后
    //         ],
    //     });
    //     superposition.push(Tile {
    //         filename: "0-tileset_11.png".to_string(),
    //         layer,
    //         collider: TileCollider::Full,
    //         joints: [
    //             TileJoint::TagOne("边".to_string()),       // 0上
    //             TileJoint::TagOne("x|泥|边".to_string()), // 1下
    //             TileJoint::TagOne("y|边|泥".to_string()), // 2左
    //             TileJoint::TagOne("边".to_string()),       // 3右
    //             TileJoint::All,                             // 4前
    //             TileJoint::None,                            // 5后
    //         ],
    //     });
    //     superposition.push(Tile {
    //         filename: "0-tileset_29.png".to_string(),
    //         layer,
    //         collider: TileCollider::Full,
    //         joints: [
    //             TileJoint::TagOne("x|边|泥".to_string()), // 0上
    //             TileJoint::TagOne("x|边|泥".to_string()), // 1下
    //             TileJoint::TagOne("边".to_string()),       // 2左
    //             TileJoint::TagOne("泥".to_string()),       // 3右
    //             TileJoint::All,                             // 4前
    //             TileJoint::None,                            // 5后
    //         ],
    //     });
    //     superposition.push(Tile {
    //         filename: "0-tileset_30.png".to_string(),
    //         layer,
    //         collider: TileCollider::Full,
    //         joints: [
    //             TileJoint::TagOne("泥".to_string()), // 0上
    //             TileJoint::TagOne("泥".to_string()), // 1下
    //             TileJoint::TagOne("泥".to_string()), // 2左
    //             TileJoint::TagOne("泥".to_string()), // 3右
    //             TileJoint::All,                       // 4前
    //             TileJoint::None,                      // 5后
    //         ],
    //     });
    //     superposition.push(Tile {
    //         filename: "0-tileset_31.png".to_string(),
    //         layer,
    //         collider: TileCollider::Full,
    //         joints: [
    //             TileJoint::TagOne("x|泥|边".to_string()), // 0上
    //             TileJoint::TagOne("x|泥|边".to_string()), // 1下
    //             TileJoint::TagOne("泥".to_string()),       // 2左
    //             TileJoint::TagOne("边".to_string()),       // 3右
    //             TileJoint::All,                             // 4前
    //             TileJoint::None,                            // 5后
    //         ],
    //     });
    //     superposition.push(Tile {
    //         filename: "0-tileset_47.png".to_string(),
    //         layer,
    //         collider: TileCollider::Full,
    //         joints: [
    //             TileJoint::TagOne("x|边|泥".to_string()), // 0上
    //             TileJoint::TagOne("边".to_string()),       // 1下
    //             TileJoint::TagOne("边".to_string()),       // 2左
    //             TileJoint::TagOne("y|泥|边".to_string()), // 3右
    //             TileJoint::All,                             // 4前
    //             TileJoint::None,                            // 5后
    //         ],
    //     });
    //     superposition.push(Tile {
    //         filename: "0-tileset_48.png".to_string(),
    //         layer,
    //         collider: TileCollider::Full,
    //         joints: [
    //             TileJoint::TagOne("泥".to_string()),       // 0上
    //             TileJoint::TagOne("边".to_string()),       // 1下
    //             TileJoint::TagOne("y|泥|边".to_string()), // 2左
    //             TileJoint::TagOne("y|泥|边".to_string()), // 3右
    //             TileJoint::All,                             // 4前
    //             TileJoint::None,                            // 5后
    //         ],
    //     });
    //     superposition.push(Tile {
    //         filename: "0-tileset_49.png".to_string(),
    //         layer,
    //         collider: TileCollider::Full,
    //         joints: [
    //             TileJoint::TagOne("x|泥|边".to_string()), // 0上
    //             TileJoint::TagOne("边".to_string()),       // 1下
    //             TileJoint::TagOne("y|泥|边".to_string()), // 2左
    //             TileJoint::TagOne("边".to_string()),       // 3右
    //             TileJoint::All,                             // 4前
    //             TileJoint::None,                            // 5后
    //         ],
    //     });
    // }

    // 砖地
    {
        superposition.push(Tile {
            filename: "0-tileset_13.png".to_string(),
            layer,
            rng_seed: 1,
            collider: TileCollider::Full,
            joints: [
                TileJoint::TagOne("砖空".to_string()),    // 0上
                TileJoint::TagOne("x|边|砖".to_string()), // 1下
                TileJoint::TagOne("砖空".to_string()),    // 2左
                TileJoint::TagOne("y|边|砖".to_string()), // 3右
                TileJoint::All,                             // 4前
                TileJoint::None,                            // 5后
            ],
        });
        superposition.push(Tile {
            filename: "0-tileset_14.png".to_string(),
            layer,
            rng_seed: 1,
            collider: TileCollider::Full,
            joints: [
                TileJoint::TagOne("砖空".to_string()),    // 0上
                TileJoint::TagOne("砖".to_string()),       // 1下
                TileJoint::TagOne("y|边|砖".to_string()), // 2左
                TileJoint::TagOne("y|边|砖".to_string()), // 3右
                TileJoint::All,                             // 4前
                TileJoint::None,                            // 5后
            ],
        });
        superposition.push(Tile {
            filename: "0-tileset_15.png".to_string(),
            layer,
            rng_seed: 1,
            collider: TileCollider::Full,
            joints: [
                TileJoint::TagOne("砖空".to_string()),    // 0上
                TileJoint::TagOne("x|砖|边".to_string()), // 1下
                TileJoint::TagOne("y|边|砖".to_string()), // 2左
                TileJoint::TagOne("砖空".to_string()),    // 3右
                TileJoint::All,                             // 4前
                TileJoint::None,                            // 5后
            ],
        });
        superposition.push(Tile {
            filename: "0-tileset_33.png".to_string(),
            layer,
            rng_seed: 1,
            collider: TileCollider::Full,
            joints: [
                TileJoint::TagOne("x|边|砖".to_string()), // 0上
                TileJoint::TagOne("x|边|砖".to_string()), // 1下
                TileJoint::TagOne("砖空".to_string()),    // 2左
                TileJoint::TagOne("砖".to_string()),       // 3右
                TileJoint::All,                             // 4前
                TileJoint::None,                            // 5后
            ],
        });
        superposition.push(Tile {
            filename: "0-tileset_34.png".to_string(),
            layer,
            rng_seed: 1,
            collider: TileCollider::Full,
            joints: [
                TileJoint::TagOne("砖".to_string()), // 0上
                TileJoint::TagOne("砖".to_string()), // 1下
                TileJoint::TagOne("砖".to_string()), // 2左
                TileJoint::TagOne("砖".to_string()), // 3右
                TileJoint::All,                       // 4前
                TileJoint::None,                      // 5后
            ],
        });
        superposition.push(Tile {
            filename: "0-tileset_35.png".to_string(),
            layer,
            rng_seed: 1,
            collider: TileCollider::Full,
            joints: [
                TileJoint::TagOne("x|砖|边".to_string()), // 0上
                TileJoint::TagOne("x|砖|边".to_string()), // 1下
                TileJoint::TagOne("砖".to_string()),       // 2左
                TileJoint::TagOne("砖空".to_string()),    // 3右
                TileJoint::All,                             // 4前
                TileJoint::None,                            // 5后
            ],
        });
        superposition.push(Tile {
            filename: "0-tileset_51.png".to_string(),
            layer,
            rng_seed: 1,
            collider: TileCollider::Full,
            joints: [
                TileJoint::TagOne("x|边|砖".to_string()), // 0上
                TileJoint::TagOne("砖空".to_string()),    // 1下
                TileJoint::TagOne("砖空".to_string()),    // 2左
                TileJoint::TagOne("y|砖|边".to_string()), // 3右
                TileJoint::All,                             // 4前
                TileJoint::None,                            // 5后
            ],
        });
        superposition.push(Tile {
            filename: "0-tileset_52.png".to_string(),
            layer,
            rng_seed: 1,
            collider: TileCollider::Full,
            joints: [
                TileJoint::TagOne("砖".to_string()),       // 0上
                TileJoint::TagOne("砖空".to_string()),    // 1下
                TileJoint::TagOne("y|砖|边".to_string()), // 2左
                TileJoint::TagOne("y|砖|边".to_string()), // 3右
                TileJoint::All,                             // 4前
                TileJoint::None,                            // 5后
            ],
        });
        superposition.push(Tile {
            filename: "0-tileset_53.png".to_string(),
            layer,
            rng_seed: 1,
            collider: TileCollider::Full,
            joints: [
                TileJoint::TagOne("x|砖|边".to_string()), // 0上
                TileJoint::TagOne("砖空".to_string()),    // 1下
                TileJoint::TagOne("y|砖|边".to_string()), // 2左
                TileJoint::TagOne("砖空".to_string()),    // 3右
                TileJoint::All,                             // 4前
                TileJoint::None,                            // 5后
            ],
        });
    }

    superposition
}

pub fn load_item_superposition(layer: usize) -> Vec<Tile> {
    let mut superposition = Vec::new();
    superposition.push(Tile {
        filename: "0-tileset_50.png".to_string(),
        layer,
        rng_seed: 1,
        collider: TileCollider::Full,
        joints: [
            TileJoint::TagOne("边".to_string()), // 0上
            TileJoint::TagOne("边".to_string()), // 1下
            TileJoint::TagOne("边".to_string()), // 2左
            TileJoint::TagOne("边".to_string()), // 3右
            TileJoint::All,                       // 4前
            TileJoint::All,                       // 5后
        ],
    });
    superposition
}

pub fn load_all_superposition(layer: usize) -> Vec<Tile> {
    let mut superposition = Vec::new();
    superposition.push(Tile {
        filename: "0-tileset_50.png".to_string(),
        layer,
        rng_seed: 1,
        collider: TileCollider::Full,
        joints: [
            TileJoint::TagOne("边".to_string()), // 0上
            TileJoint::TagOne("边".to_string()), // 1下
            TileJoint::TagOne("边".to_string()), // 2左
            TileJoint::TagOne("边".to_string()), // 3右
            TileJoint::All,                       // 4前
            TileJoint::All,                       // 5后
        ],
    });
    superposition.push(Tile {
        filename: "0-tileset_65.png".to_string(),
        layer,
        rng_seed: 1,
        collider: TileCollider::Full,
        joints: [
            TileJoint::TagOne("路".to_string()), // 0上
            TileJoint::TagOne("路".to_string()), // 1下
            TileJoint::TagOne("路".to_string()), // 2左
            TileJoint::TagOne("路".to_string()), // 3右
            TileJoint::All,                       // 4前
            TileJoint::All,                       // 5后
        ],
    });
    superposition.push(Tile {
        filename: "0-tileset_64.png".to_string(),
        layer,
        rng_seed: 1,
        collider: TileCollider::Full,
        joints: [
            TileJoint::TagOne("路".to_string()), // 0上
            TileJoint::TagOne("路".to_string()), // 1下
            TileJoint::TagOne("边".to_string()), // 2左
            TileJoint::TagOne("路".to_string()), // 3右
            TileJoint::All,                       // 4前
            TileJoint::All,                       // 5后
        ],
    });
    superposition.push(Tile {
        filename: "0-tileset_63.png".to_string(),
        layer,
        rng_seed: 1,
        collider: TileCollider::Full,
        joints: [
            TileJoint::TagOne("路".to_string()), // 0上
            TileJoint::TagOne("边".to_string()), // 1下
            TileJoint::TagOne("路".to_string()), // 2左
            TileJoint::TagOne("边".to_string()), // 3右
            TileJoint::All,                       // 4前
            TileJoint::All,                       // 5后
        ],
    });
    superposition.push(Tile {
        filename: "0-tileset_62.png".to_string(),
        layer,
        rng_seed: 1,
        collider: TileCollider::Full,
        joints: [
            TileJoint::TagOne("路".to_string()), // 0上
            TileJoint::TagOne("路".to_string()), // 1下
            TileJoint::TagOne("边".to_string()), // 2左
            TileJoint::TagOne("边".to_string()), // 3右
            TileJoint::All,                       // 4前
            TileJoint::All,                       // 5后
        ],
    });
    superposition.push(Tile {
        filename: "0-tileset_58.png".to_string(),
        layer,
        rng_seed: 1,
        collider: TileCollider::Full,
        joints: [
            TileJoint::TagOne("边".to_string()), // 0上
            TileJoint::TagOne("边".to_string()), // 1下
            TileJoint::TagOne("路".to_string()), // 2左
            TileJoint::TagOne("路".to_string()), // 3右
            TileJoint::All,                       // 4前
            TileJoint::All,                       // 5后
        ],
    });
    superposition.push(Tile {
        filename: "0-tileset_57.png".to_string(),
        layer,
        rng_seed: 1,
        collider: TileCollider::Full,
        joints: [
            TileJoint::TagOne("边".to_string()), // 0上
            TileJoint::TagOne("路".to_string()), // 1下
            TileJoint::TagOne("边".to_string()), // 2左
            TileJoint::TagOne("路".to_string()), // 3右
            TileJoint::All,                       // 4前
            TileJoint::All,                       // 5后
        ],
    });
    superposition.push(Tile {
        filename: "0-tileset_59.png".to_string(),
        layer,
        rng_seed: 1,
        collider: TileCollider::Full,
        joints: [
            TileJoint::TagOne("边".to_string()), // 0上
            TileJoint::TagOne("路".to_string()), // 1下
            TileJoint::TagOne("路".to_string()), // 2左
            TileJoint::TagOne("边".to_string()), // 3右
            TileJoint::All,                       // 4前
            TileJoint::All,                       // 5后
        ],
    });
    superposition.push(Tile {
        filename: "0-tileset_61.png".to_string(),
        layer,
        rng_seed: 1,
        collider: TileCollider::Full,
        joints: [
            TileJoint::TagOne("路".to_string()), // 0上
            TileJoint::TagOne("边".to_string()), // 1下
            TileJoint::TagOne("边".to_string()), // 2左
            TileJoint::TagOne("路".to_string()), // 3右
            TileJoint::All,                       // 4前
            TileJoint::All,                       // 5后
        ],
    });
    superposition.push(Tile {
        filename: "0-tileset_66.png".to_string(),
        layer,
        rng_seed: 1,
        collider: TileCollider::Full,
        joints: [
            TileJoint::TagOne("路".to_string()), // 0上
            TileJoint::TagOne("路".to_string()), // 1下
            TileJoint::TagOne("路".to_string()), // 2左
            TileJoint::TagOne("边".to_string()), // 3右
            TileJoint::All,                       // 4前
            TileJoint::All,                       // 5后
        ],
    });
    superposition.push(Tile {
        filename: "0-tileset_67.png".to_string(),
        layer,
        rng_seed: 1,
        collider: TileCollider::Full,
        joints: [
            TileJoint::TagOne("边".to_string()), // 0上
            TileJoint::TagOne("路".to_string()), // 1下
            TileJoint::TagOne("路".to_string()), // 2左
            TileJoint::TagOne("路".to_string()), // 3右
            TileJoint::All,                       // 4前
            TileJoint::All,                       // 5后
        ],
    });
    superposition.push(Tile {
        filename: "0-tileset_68.png".to_string(),
        layer,
        rng_seed: 1,
        collider: TileCollider::Full,
        joints: [
            TileJoint::TagOne("路".to_string()), // 0上
            TileJoint::TagOne("边".to_string()), // 1下
            TileJoint::TagOne("路".to_string()), // 2左
            TileJoint::TagOne("路".to_string()), // 3右
            TileJoint::All,                       // 4前
            TileJoint::All,                       // 5后
        ],
    });
    superposition.push(Tile {
        filename: "0-tileset_69.png".to_string(),
        layer,
        rng_seed: 1,
        collider: TileCollider::Full,
        joints: [
            TileJoint::TagOne("路".to_string()), // 0上
            TileJoint::TagOne("路".to_string()), // 1下
            TileJoint::TagOne("路".to_string()), // 2左
            TileJoint::TagOne("路".to_string()), // 3右
            TileJoint::All,                       // 4前
            TileJoint::All,                       // 5后
        ],
    });
    // 草地
    superposition.push(Tile {
        filename: "0-tileset_01.png".to_string(),
        layer,
        rng_seed: 1,
        collider: TileCollider::Full,
        joints: [
            TileJoint::TagOne("边".to_string()),       // 0上
            TileJoint::TagOne("x|边|草".to_string()), // 1下
            TileJoint::TagOne("边".to_string()),       // 2左
            TileJoint::TagOne("y|边|草".to_string()), // 3右
            TileJoint::All,                             // 4前
            TileJoint::All,                             // 5后
        ],
    });
    superposition.push(Tile {
        filename: "0-tileset_02.png".to_string(),
        layer,
        rng_seed: 1,
        collider: TileCollider::Full,
        joints: [
            TileJoint::TagOne("边".to_string()),       // 0上
            TileJoint::TagOne("草".to_string()),       // 1下
            TileJoint::TagOne("y|边|草".to_string()), // 2左
            TileJoint::TagOne("y|边|草".to_string()), // 3右
            TileJoint::All,                             // 4前
            TileJoint::All,                             // 5后
        ],
    });
    superposition.push(Tile {
        filename: "0-tileset_03.png".to_string(),
        layer,
        rng_seed: 1,
        collider: TileCollider::Full,
        joints: [
            TileJoint::TagOne("边".to_string()),       // 0上
            TileJoint::TagOne("x|草|边".to_string()), // 1下
            TileJoint::TagOne("y|边|草".to_string()), // 2左
            TileJoint::TagOne("边".to_string()),       // 3右
            TileJoint::All,                             // 4前
            TileJoint::All,                             // 5后
        ],
    });
    superposition.push(Tile {
        filename: "0-tileset_04.png".to_string(),
        layer,
        rng_seed: 1,
        collider: TileCollider::Full,
        joints: [
            TileJoint::TagOne("草".to_string()), // 0上
            TileJoint::TagOne("草".to_string()), // 1下
            TileJoint::TagOne("草".to_string()), // 2左
            TileJoint::TagOne("草".to_string()), // 3右
            TileJoint::All,                       // 4前
            TileJoint::All,                       // 5后
        ],
    });
    superposition.push(Tile {
        filename: "0-tileset_21.png".to_string(),
        layer,
        rng_seed: 1,
        collider: TileCollider::Full,
        joints: [
            TileJoint::TagOne("x|边|草".to_string()), // 0上
            TileJoint::TagOne("x|边|草".to_string()), // 1下
            TileJoint::TagOne("边".to_string()),       // 2左
            TileJoint::TagOne("草".to_string()),       // 3右
            TileJoint::All,                             // 4前
            TileJoint::All,                             // 5后
        ],
    });
    superposition.push(Tile {
        filename: "0-tileset_23.png".to_string(),
        layer,
        rng_seed: 1,
        collider: TileCollider::Full,
        joints: [
            TileJoint::TagOne("x|草|边".to_string()), // 0上
            TileJoint::TagOne("x|草|边".to_string()), // 1下
            TileJoint::TagOne("草".to_string()),       // 2左
            TileJoint::TagOne("边".to_string()),       // 3右
            TileJoint::All,                             // 4前
            TileJoint::All,                             // 5后
        ],
    });
    superposition.push(Tile {
        filename: "0-tileset_39.png".to_string(),
        layer,
        rng_seed: 1,
        collider: TileCollider::Full,
        joints: [
            TileJoint::TagOne("x|边|草".to_string()), // 0上
            TileJoint::TagOne("边".to_string()),       // 1下
            TileJoint::TagOne("边".to_string()),       // 2左
            TileJoint::TagOne("y|草|边".to_string()), // 3右
            TileJoint::All,                             // 4前
            TileJoint::All,                             // 5后
        ],
    });
    superposition.push(Tile {
        filename: "0-tileset_40.png".to_string(),
        layer,
        rng_seed: 1,
        collider: TileCollider::Full,
        joints: [
            TileJoint::TagOne("草".to_string()),       // 0上
            TileJoint::TagOne("边".to_string()),       // 1下
            TileJoint::TagOne("y|草|边".to_string()), // 2左
            TileJoint::TagOne("y|草|边".to_string()), // 3右
            TileJoint::All,                             // 4前
            TileJoint::All,                             // 5后
        ],
    });
    superposition.push(Tile {
        filename: "0-tileset_41.png".to_string(),
        layer,
        rng_seed: 1,
        collider: TileCollider::Full,
        joints: [
            TileJoint::TagOne("x|草|边".to_string()), // 0上
            TileJoint::TagOne("边".to_string()),       // 1下
            TileJoint::TagOne("y|草|边".to_string()), // 2左
            TileJoint::TagOne("边".to_string()),       // 3右
            TileJoint::All,                             // 4前
            TileJoint::All,                             // 5后
        ],
    });
    superposition
}
