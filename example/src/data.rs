use protocol::data::tile_map_data::TileCollider;

use crate::plugin::tile_map::{Tile, TileJoint};

// 加载默认胶水tile初始化叠加态
pub fn load_glue_superposition() -> Vec<Tile> {
    let mut superposition = Vec::new();
    superposition.push(Tile {
        filename: "0-tileset_50.png".to_string(),
        layer: 0,
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

pub fn load_default_superposition() -> Vec<Tile> {
    let mut superposition = Vec::new();
    superposition.push(Tile {
        filename: "0-tileset_50.png".to_string(),
        layer: 0,
        collider: TileCollider::Full,
        joints: [
            TileJoint::TagOne("墙".to_string()), // 0上
            TileJoint::TagOne("墙".to_string()), // 1下
            TileJoint::TagOne("墙".to_string()), // 2左
            TileJoint::TagOne("墙".to_string()), // 3右
            TileJoint::All,                       // 4前
            TileJoint::All,                       // 5后
        ],
    });
    superposition.push(Tile {
        filename: "0-tileset_65.png".to_string(),
        layer: 0,
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
        layer: 0,
        collider: TileCollider::Full,
        joints: [
            TileJoint::TagOne("路".to_string()), // 0上
            TileJoint::TagOne("路".to_string()), // 1下
            TileJoint::TagOne("墙".to_string()), // 2左
            TileJoint::TagOne("路".to_string()), // 3右
            TileJoint::All,                       // 4前
            TileJoint::All,                       // 5后
        ],
    });
    superposition.push(Tile {
        filename: "0-tileset_63.png".to_string(),
        layer: 0,
        collider: TileCollider::Full,
        joints: [
            TileJoint::TagOne("路".to_string()), // 0上
            TileJoint::TagOne("墙".to_string()), // 1下
            TileJoint::TagOne("路".to_string()), // 2左
            TileJoint::TagOne("墙".to_string()), // 3右
            TileJoint::All,                       // 4前
            TileJoint::All,                       // 5后
        ],
    });
    superposition.push(Tile {
        filename: "0-tileset_62.png".to_string(),
        layer: 0,
        collider: TileCollider::Full,
        joints: [
            TileJoint::TagOne("路".to_string()), // 0上
            TileJoint::TagOne("路".to_string()), // 1下
            TileJoint::TagOne("墙".to_string()), // 2左
            TileJoint::TagOne("墙".to_string()), // 3右
            TileJoint::All,                       // 4前
            TileJoint::All,                       // 5后
        ],
    });
    superposition.push(Tile {
        filename: "0-tileset_58.png".to_string(),
        layer: 0,
        collider: TileCollider::Full,
        joints: [
            TileJoint::TagOne("墙".to_string()), // 0上
            TileJoint::TagOne("墙".to_string()), // 1下
            TileJoint::TagOne("路".to_string()), // 2左
            TileJoint::TagOne("路".to_string()), // 3右
            TileJoint::All,                       // 4前
            TileJoint::All,                       // 5后
        ],
    });
    superposition.push(Tile {
        filename: "0-tileset_57.png".to_string(),
        layer: 0,
        collider: TileCollider::Full,
        joints: [
            TileJoint::TagOne("墙".to_string()), // 0上
            TileJoint::TagOne("路".to_string()), // 1下
            TileJoint::TagOne("墙".to_string()), // 2左
            TileJoint::TagOne("路".to_string()), // 3右
            TileJoint::All,                       // 4前
            TileJoint::All,                       // 5后
        ],
    });
    superposition.push(Tile {
        filename: "0-tileset_59.png".to_string(),
        layer: 0,
        collider: TileCollider::Full,
        joints: [
            TileJoint::TagOne("墙".to_string()), // 0上
            TileJoint::TagOne("路".to_string()), // 1下
            TileJoint::TagOne("路".to_string()), // 2左
            TileJoint::TagOne("墙".to_string()), // 3右
            TileJoint::All,                       // 4前
            TileJoint::All,                       // 5后
        ],
    });
    superposition.push(Tile {
        filename: "0-tileset_61.png".to_string(),
        layer: 0,
        collider: TileCollider::Full,
        joints: [
            TileJoint::TagOne("路".to_string()), // 0上
            TileJoint::TagOne("墙".to_string()), // 1下
            TileJoint::TagOne("墙".to_string()), // 2左
            TileJoint::TagOne("路".to_string()), // 3右
            TileJoint::All,                       // 4前
            TileJoint::All,                       // 5后
        ],
    });
    superposition.push(Tile {
        filename: "0-tileset_66.png".to_string(),
        layer: 0,
        collider: TileCollider::Full,
        joints: [
            TileJoint::TagOne("路".to_string()), // 0上
            TileJoint::TagOne("路".to_string()), // 1下
            TileJoint::TagOne("路".to_string()), // 2左
            TileJoint::TagOne("墙".to_string()), // 3右
            TileJoint::All,                       // 4前
            TileJoint::All,                       // 5后
        ],
    });
    superposition.push(Tile {
        filename: "0-tileset_67.png".to_string(),
        layer: 0,
        collider: TileCollider::Full,
        joints: [
            TileJoint::TagOne("墙".to_string()), // 0上
            TileJoint::TagOne("路".to_string()), // 1下
            TileJoint::TagOne("路".to_string()), // 2左
            TileJoint::TagOne("路".to_string()), // 3右
            TileJoint::All,                       // 4前
            TileJoint::All,                       // 5后
        ],
    });
    superposition.push(Tile {
        filename: "0-tileset_68.png".to_string(),
        layer: 0,
        collider: TileCollider::Full,
        joints: [
            TileJoint::TagOne("路".to_string()), // 0上
            TileJoint::TagOne("墙".to_string()), // 1下
            TileJoint::TagOne("路".to_string()), // 2左
            TileJoint::TagOne("路".to_string()), // 3右
            TileJoint::All,                       // 4前
            TileJoint::All,                       // 5后
        ],
    });
    superposition.push(Tile {
        filename: "0-tileset_69.png".to_string(),
        layer: 0,
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
        layer: 0,
        collider: TileCollider::Full,
        joints: [
            TileJoint::TagOne("墙".to_string()), // 0上
            TileJoint::TagOne("x|墙|草".to_string()), // 1下
            TileJoint::TagOne("墙".to_string()), // 2左
            TileJoint::TagOne("y|墙|草".to_string()), // 3右
            TileJoint::All,                       // 4前
            TileJoint::All,                       // 5后
        ],
    });
    superposition.push(Tile {
        filename: "0-tileset_02.png".to_string(),
        layer: 0,
        collider: TileCollider::Full,
        joints: [
            TileJoint::TagOne("墙".to_string()), // 0上
            TileJoint::TagOne("草".to_string()), // 1下
            TileJoint::TagOne("y|墙|草".to_string()), // 2左
            TileJoint::TagOne("y|墙|草".to_string()), // 3右
            TileJoint::All,                       // 4前
            TileJoint::All,                       // 5后
        ],
    });
    superposition.push(Tile {
        filename: "0-tileset_03.png".to_string(),
        layer: 0,
        collider: TileCollider::Full,
        joints: [
            TileJoint::TagOne("墙".to_string()), // 0上
            TileJoint::TagOne("x|草|墙".to_string()), // 1下
            TileJoint::TagOne("y|墙|草".to_string()), // 2左
            TileJoint::TagOne("墙".to_string()), // 3右
            TileJoint::All,                       // 4前
            TileJoint::All,                       // 5后
        ],
    });
    superposition.push(Tile {
        filename: "0-tileset_04.png".to_string(),
        layer: 0,
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
        layer: 0,
        collider: TileCollider::Full,
        joints: [
            TileJoint::TagOne("x|墙|草".to_string()), // 0上
            TileJoint::TagOne("x|墙|草".to_string()), // 1下
            TileJoint::TagOne("墙".to_string()), // 2左
            TileJoint::TagOne("草".to_string()), // 3右
            TileJoint::All,                       // 4前
            TileJoint::All,                       // 5后
        ],
    });
    superposition.push(Tile {
        filename: "0-tileset_23.png".to_string(),
        layer: 0,
        collider: TileCollider::Full,
        joints: [
            TileJoint::TagOne("x|草|墙".to_string()), // 0上
            TileJoint::TagOne("x|草|墙".to_string()), // 1下
            TileJoint::TagOne("草".to_string()), // 2左
            TileJoint::TagOne("墙".to_string()), // 3右
            TileJoint::All,                       // 4前
            TileJoint::All,                       // 5后
        ],
    });
    superposition.push(Tile {
        filename: "0-tileset_39.png".to_string(),
        layer: 0,
        collider: TileCollider::Full,
        joints: [
            TileJoint::TagOne("x|墙|草".to_string()), // 0上
            TileJoint::TagOne("墙".to_string()), // 1下
            TileJoint::TagOne("墙".to_string()), // 2左
            TileJoint::TagOne("y|草|墙".to_string()), // 3右
            TileJoint::All,                       // 4前
            TileJoint::All,                       // 5后
        ],
    });
    superposition.push(Tile {
        filename: "0-tileset_40.png".to_string(),
        layer: 0,
        collider: TileCollider::Full,
        joints: [
            TileJoint::TagOne("草".to_string()), // 0上
            TileJoint::TagOne("墙".to_string()), // 1下
            TileJoint::TagOne("y|草|墙".to_string()), // 2左
            TileJoint::TagOne("y|草|墙".to_string()), // 3右
            TileJoint::All,                       // 4前
            TileJoint::All,                       // 5后
        ],
    });
    superposition.push(Tile {
        filename: "0-tileset_41.png".to_string(),
        layer: 0,
        collider: TileCollider::Full,
        joints: [
            TileJoint::TagOne("x|草|墙".to_string()), // 0上
            TileJoint::TagOne("墙".to_string()), // 1下
            TileJoint::TagOne("y|草|墙".to_string()), // 2左
            TileJoint::TagOne("墙".to_string()), // 3右
            TileJoint::All,                       // 4前
            TileJoint::All,                       // 5后
        ],
    });
    superposition
}

pub fn load_terrain_superposition() -> Vec<Tile> {
    let mut superposition = Vec::new();
    superposition.push(Tile {
        filename: "0-tileset_50.png".to_string(),
        layer: 0,
        collider: TileCollider::Full,
        joints: [
            TileJoint::TagOne("墙".to_string()), // 0上
            TileJoint::TagOne("墙".to_string()), // 1下
            TileJoint::TagOne("墙".to_string()), // 2左
            TileJoint::TagOne("墙".to_string()), // 3右
            TileJoint::All,                       // 4前
            TileJoint::All,                       // 5后
        ],
    });
    superposition
}