use crate::data::{
    account_data::AccountData,
    control_data::ControlData,
    player_data::PlayerData,
    tile_map_data::{TileData, TileMapData},
    update_data::UpdateData,
};
use serde::{Deserialize, Serialize};
// 数据包二级路由[1]
// 心跳包路由
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum HeartbeatRoute {
    In,
    Out,
    Keep(u128),
}
// 账号中心路由
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum AccountRoute {
    Login(AccountData),
    Logout(AccountData),
    GetInfo(AccountData),
}
// 游戏路由
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum GameRoute {
    Update(UpdateData),
    Control(ControlData),
    TileMap(TileMapData),
    Tile(TileData),
    Player(PlayerData),
}
