use protocol::data::tile_map_data::TileCollider;

use crate::plugin::tile_map::{Tile, TileJoint};

pub const SIDE: &str = "边";

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
            TileJoint::TagOne(SIDE.to_string()), // 0上
            TileJoint::TagOne(SIDE.to_string()), // 1下
            TileJoint::TagOne(SIDE.to_string()), // 2左
            TileJoint::TagOne(SIDE.to_string()), // 3右
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
                TileJoint::TagOne(SIDE.to_string()),       // 0上
                TileJoint::TagOne("x|边|草".to_string()), // 1下
                TileJoint::TagOne(SIDE.to_string()),       // 2左
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
                TileJoint::TagOne(SIDE.to_string()),       // 0上
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
                TileJoint::TagOne(SIDE.to_string()),       // 0上
                TileJoint::TagOne("x|草|边".to_string()), // 1下
                TileJoint::TagOne("y|边|草".to_string()), // 2左
                TileJoint::TagOne(SIDE.to_string()),       // 3右
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
                TileJoint::TagOne(SIDE.to_string()),       // 2左
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
                TileJoint::TagOne(SIDE.to_string()),       // 3右
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
                TileJoint::TagOne(SIDE.to_string()),       // 1下
                TileJoint::TagOne(SIDE.to_string()),       // 2左
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
                TileJoint::TagOne(SIDE.to_string()),       // 1下
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
                TileJoint::TagOne(SIDE.to_string()),       // 1下
                TileJoint::TagOne("y|草|边".to_string()), // 2左
                TileJoint::TagOne(SIDE.to_string()),       // 3右
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
                TileJoint::TagOne(SIDE.to_string()),       // 0上
                TileJoint::TagOne("x|边|水".to_string()), // 1下
                TileJoint::TagOne(SIDE.to_string()),       // 2左
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
                TileJoint::TagOne(SIDE.to_string()),       // 0上
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
                TileJoint::TagOne(SIDE.to_string()),       // 0上
                TileJoint::TagOne("x|水|边".to_string()), // 1下
                TileJoint::TagOne("y|边|水".to_string()), // 2左
                TileJoint::TagOne(SIDE.to_string()),       // 3右
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
                TileJoint::TagOne(SIDE.to_string()),       // 2左
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
                TileJoint::TagOne(SIDE.to_string()),       // 3右
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
                TileJoint::TagOne(SIDE.to_string()),       // 1下
                TileJoint::TagOne(SIDE.to_string()),       // 2左
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
                TileJoint::TagOne(SIDE.to_string()),       // 1下
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
                TileJoint::TagOne(SIDE.to_string()),       // 1下
                TileJoint::TagOne("y|水|边".to_string()), // 2左
                TileJoint::TagOne(SIDE.to_string()),       // 3右
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
    //             TileJoint::TagOne(SIDE.to_string()),       // 0上
    //             TileJoint::TagOne("x|边|泥".to_string()), // 1下
    //             TileJoint::TagOne(SIDE.to_string()),       // 2左
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
    //             TileJoint::TagOne(SIDE.to_string()),       // 0上
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
    //             TileJoint::TagOne(SIDE.to_string()),       // 0上
    //             TileJoint::TagOne("x|泥|边".to_string()), // 1下
    //             TileJoint::TagOne("y|边|泥".to_string()), // 2左
    //             TileJoint::TagOne(SIDE.to_string()),       // 3右
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
    //             TileJoint::TagOne(SIDE.to_string()),       // 2左
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
    //             TileJoint::TagOne(SIDE.to_string()),       // 3右
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
    //             TileJoint::TagOne(SIDE.to_string()),       // 1下
    //             TileJoint::TagOne(SIDE.to_string()),       // 2左
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
    //             TileJoint::TagOne(SIDE.to_string()),       // 1下
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
    //             TileJoint::TagOne(SIDE.to_string()),       // 1下
    //             TileJoint::TagOne("y|泥|边".to_string()), // 2左
    //             TileJoint::TagOne(SIDE.to_string()),       // 3右
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
                TileJoint::TagOne(SIDE.to_string()),       // 0上
                TileJoint::TagOne("x|边|砖".to_string()), // 1下
                TileJoint::TagOne(SIDE.to_string()),       // 2左
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
                TileJoint::TagOne(SIDE.to_string()),       // 0上
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
                TileJoint::TagOne(SIDE.to_string()),       // 0上
                TileJoint::TagOne("x|砖|边".to_string()), // 1下
                TileJoint::TagOne("y|边|砖".to_string()), // 2左
                TileJoint::TagOne(SIDE.to_string()),       // 3右
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
                TileJoint::TagOne(SIDE.to_string()),       // 2左
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
                TileJoint::TagOne(SIDE.to_string()),       // 3右
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
                TileJoint::TagOne(SIDE.to_string()),       // 1下
                TileJoint::TagOne(SIDE.to_string()),       // 2左
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
                TileJoint::TagOne(SIDE.to_string()),       // 1下
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
                TileJoint::TagOne(SIDE.to_string()),       // 1下
                TileJoint::TagOne("y|砖|边".to_string()), // 2左
                TileJoint::TagOne(SIDE.to_string()),       // 3右
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
            TileJoint::TagOne(SIDE.to_string()), // 0上
            TileJoint::TagOne(SIDE.to_string()), // 1下
            TileJoint::TagOne(SIDE.to_string()), // 2左
            TileJoint::TagOne(SIDE.to_string()), // 3右
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
            TileJoint::TagOne(SIDE.to_string()), // 0上
            TileJoint::TagOne(SIDE.to_string()), // 1下
            TileJoint::TagOne(SIDE.to_string()), // 2左
            TileJoint::TagOne(SIDE.to_string()), // 3右
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
            TileJoint::TagOne(SIDE.to_string()), // 2左
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
            TileJoint::TagOne(SIDE.to_string()), // 1下
            TileJoint::TagOne("路".to_string()), // 2左
            TileJoint::TagOne(SIDE.to_string()), // 3右
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
            TileJoint::TagOne(SIDE.to_string()), // 2左
            TileJoint::TagOne(SIDE.to_string()), // 3右
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
            TileJoint::TagOne(SIDE.to_string()), // 0上
            TileJoint::TagOne(SIDE.to_string()), // 1下
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
            TileJoint::TagOne(SIDE.to_string()), // 0上
            TileJoint::TagOne("路".to_string()), // 1下
            TileJoint::TagOne(SIDE.to_string()), // 2左
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
            TileJoint::TagOne(SIDE.to_string()), // 0上
            TileJoint::TagOne("路".to_string()), // 1下
            TileJoint::TagOne("路".to_string()), // 2左
            TileJoint::TagOne(SIDE.to_string()), // 3右
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
            TileJoint::TagOne(SIDE.to_string()), // 1下
            TileJoint::TagOne(SIDE.to_string()), // 2左
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
            TileJoint::TagOne(SIDE.to_string()), // 3右
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
            TileJoint::TagOne(SIDE.to_string()), // 0上
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
            TileJoint::TagOne(SIDE.to_string()), // 1下
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
            TileJoint::TagOne(SIDE.to_string()),       // 0上
            TileJoint::TagOne("x|边|草".to_string()), // 1下
            TileJoint::TagOne(SIDE.to_string()),       // 2左
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
            TileJoint::TagOne(SIDE.to_string()),       // 0上
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
            TileJoint::TagOne(SIDE.to_string()),       // 0上
            TileJoint::TagOne("x|草|边".to_string()), // 1下
            TileJoint::TagOne("y|边|草".to_string()), // 2左
            TileJoint::TagOne(SIDE.to_string()),       // 3右
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
            TileJoint::TagOne(SIDE.to_string()),       // 2左
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
            TileJoint::TagOne(SIDE.to_string()),       // 3右
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
            TileJoint::TagOne(SIDE.to_string()),       // 1下
            TileJoint::TagOne(SIDE.to_string()),       // 2左
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
            TileJoint::TagOne(SIDE.to_string()),       // 1下
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
            TileJoint::TagOne(SIDE.to_string()),       // 1下
            TileJoint::TagOne("y|草|边".to_string()), // 2左
            TileJoint::TagOne(SIDE.to_string()),       // 3右
            TileJoint::All,                             // 4前
            TileJoint::All,                             // 5后
        ],
    });
    superposition
}