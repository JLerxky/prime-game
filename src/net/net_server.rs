use crate::data::game_db::{self, GameData};
use serde::{Deserialize, Serialize};
use tokio::io;
use tokio::net::UdpSocket;
use tokio_stream::StreamExt;
use tokio_util::codec::BytesCodec;
use tokio_util::udp::UdpFramed;

use bytes::Bytes;
use futures::SinkExt;
use std::error::Error;
use std::net::SocketAddr;

#[derive(Copy, Clone, Deserialize, Serialize)]
struct Packet {
    uid: u32,
    event: GameEvent,
}

#[derive(Debug, Copy, Clone, Deserialize, Serialize)]
enum GameEvent {
    Default,
    // 玩家登录
    Login,
    Logout,
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

pub async fn start_server() -> Result<(), Box<dyn Error>> {
    let game_server_socket = UdpSocket::bind("127.0.0.1:2101").await?;

    let game_server_addr = game_server_socket.local_addr()?;

    println!("游戏服务器地址: {:?}", game_server_addr);

    let mut game_server_framed = UdpFramed::new(game_server_socket, BytesCodec::new());

    let game_server_future = start_listening(&mut game_server_framed);

    tokio::join!(game_server_future);

    Ok(())
}

async fn send(packet: String, recv_addr: SocketAddr) -> Result<(), io::Error> {
    let send_socket = UdpSocket::bind("127.0.0.1:0").await?;
    let mut send_framed = UdpFramed::new(send_socket, BytesCodec::new());

    send_framed.send((Bytes::from(packet), recv_addr)).await?;

    Ok(())
}

async fn start_listening(socket: &mut UdpFramed<BytesCodec>) {
    loop {
        if let Some(Ok((bytes, addr))) = socket.next().await {
            println!("recv: {}", String::from_utf8_lossy(&bytes));
            let packet =
                serde_json::from_str(&String::from_utf8_lossy(&bytes)[..]).unwrap_or(Packet {
                    uid: 0,
                    event: GameEvent::Default,
                });
            // TODO 转发事件
            match packet.event {
                GameEvent::Login => {
                    match game_db::find(GameData {
                        table: "player".to_string(),
                        key: "online".to_string(),
                        data: None,
                    }) {
                        Some(data) => {
                            if data.len() > 0 {
                                let _ = game_db::save(GameData::player(format!(
                                    "{},{}",
                                    data, packet.uid
                                )));
                            } else {
                                let _ = game_db::save(GameData::player(format!("{}", packet.uid)));
                            }
                        }
                        None => {
                            let _ = game_db::save(GameData {
                                table: "player".to_string(),
                                key: "online".to_string(),
                                data: Some(format!("{}", packet.uid)),
                            });
                        }
                    }
                }
                _ => println!("收到事件未处理: {:?}", packet.event),
            }

            // socket.send((Bytes::from("收到！"), addr)).await.unwrap();
        }
    }
}
