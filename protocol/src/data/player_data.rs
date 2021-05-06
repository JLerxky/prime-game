use serde::{Deserialize, Serialize};

// 玩家状态同步数据
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct PlayerData {
    pub uid: u32,
    // 血量
    pub hp: u32,
    // 魔力值
    pub mp: u32,
    // 最大血量
    pub max_hp: u32,
    // 最大魔力值
    pub max_mp: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerListData {
    pub frame: u128,
    pub players: Vec<PlayerData>,
}
