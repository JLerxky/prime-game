use serde::{Deserialize, Serialize};

// TileMap数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TileMapData {
    pub map_id: u128,
    pub tiles: Vec<TileState>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TileState {
    pub point: (i32, i32, i32),
    pub filename: String,
    pub collider: TileCollider,
}

// 地形碰撞体类型
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TileCollider {
    Full,
    HalfTop,
    HalfBottom,
    HalfLeft,
    HalfRight,
    HalfFront,
    HalfBack,
    HalfCenter,
    HalfCenterX,
    HalfCenterY,
}
