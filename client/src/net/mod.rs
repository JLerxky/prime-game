pub mod net_client;
use std::net::SocketAddr;

use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Deserialize, Serialize)]
pub struct Packet {
    pub uid: u32,
    pub event: GameEvent
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
}

#[derive(Debug, Copy, Clone, Deserialize, Serialize)]
pub struct LoginData {
    pub group: u32,
    pub addr: SocketAddr,
}
