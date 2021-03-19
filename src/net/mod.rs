pub mod net_client;
pub mod net_server;
use std::net::SocketAddr;

use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Deserialize, Serialize)]
struct Packet {
    uid: u32,
    event: GameEvent
}

#[derive(Debug, Copy, Clone, Deserialize, Serialize)]
enum GameEvent {
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
struct LoginData {
    group: u32,
    addr: SocketAddr,
}
