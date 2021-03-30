pub mod config;
use serde::{Deserialize, Serialize};

#[derive(Debug, Copy, Clone, Deserialize, Serialize)]
pub struct Packet {
    pub uid: u32,
    pub event: GameEvent,
}

#[derive(Debug, Copy, Clone, Deserialize, Serialize)]
pub enum GameEvent {
    Default,
    // 玩家登录
    Login(LoginData),
    Logout(LoginData),
    // 玩家移动
    MoveLeft,
    MoveRight,
    JumpUp,
    JumpDown,
    FlyUp,
    FlyDown,
    FlyLeft,
    FlyRight,
    // 物体状态更新
    Update(UpdateData),
}

#[derive(Debug, Copy, Clone, Deserialize, Serialize)]
pub struct LoginData {
    pub group: u32,
}

#[derive(Debug, Copy, Clone, Deserialize, Serialize)]
pub struct UpdateData {
    pub frame_no: u128,
    pub id: u128,
    pub translation: [f32; 2],
    pub rotation: [f32; 2],
}
