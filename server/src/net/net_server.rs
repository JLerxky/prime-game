use crate::data::game_db::{self, GameData};
use crate::net;
use net::{GameEvent, Packet};
use tokio::io;
use tokio::net::UdpSocket;
use tokio_stream::StreamExt;
use tokio_util::codec::BytesCodec;
use tokio_util::udp::UdpFramed;

use bytes::Bytes;
use futures::SinkExt;
use std::net::SocketAddr;
use std::{error::Error, str::FromStr};

pub async fn start_server() -> Result<(), Box<dyn Error>> {
    let game_server_socket = UdpSocket::bind("0.0.0.0:2101").await?;

    let game_server_addr = game_server_socket.local_addr()?;

    println!("游戏服务器地址: {:?}", game_server_addr);

    let mut game_server_framed = UdpFramed::new(game_server_socket, BytesCodec::new());

    let game_server_future = start_listening(&mut game_server_framed);

    tokio::join!(game_server_future);

    Ok(())
}

pub async fn send(packet: String, recv_addr: SocketAddr) -> Result<(), io::Error> {
    let send_socket = UdpSocket::bind("127.0.0.1:0").await?;
    let mut send_framed = UdpFramed::new(send_socket, BytesCodec::new());

    send_framed.send((Bytes::from(packet), recv_addr)).await?;

    Ok(())
}

pub async fn multicast(group: u32, packet: String) -> Result<(), Box<dyn Error>> {
    match game_db::find(GameData::player_group_addr(group, None)) {
        Some(data) => {
            if data.len() > 0 {
                let uid_list: Vec<&str> = data.split(",").collect();
                for index in 0..uid_list.len() {
                    let recv_addr = SocketAddr::from_str(uid_list[index])?;
                    send(packet.clone(), recv_addr).await?;
                }
            }
        }
        None => {
            println!("{}组无玩家在线!", group);
        }
    }
    Ok(())
}

async fn start_listening(socket: &mut UdpFramed<BytesCodec>) {
    loop {
        if let Some(Ok((bytes, _addr))) = socket.next().await {
            // println!("recv: {:?}", &bytes);
            let data_str = String::from_utf8_lossy(&bytes);
            println!("recv: {}", &data_str);
            let packet = serde_json::from_slice(data_str.as_bytes()).unwrap_or(Packet {
                uid: 0,
                event: GameEvent::Default,
            });
            // 转发事件
            match packet.event {
                GameEvent::Login(login_data) => {
                    println!("登录事件: {:?}", &login_data);
                    // 更新在线玩家表
                    match game_db::find(GameData::player_online(None)) {
                        Some(data) => {
                            if data.len() > 0 {
                                let _ = game_db::save(GameData::player_online(Some(format!(
                                    "{},{}",
                                    data, packet.uid
                                ))));
                            } else {
                                let _ = game_db::save(GameData::player_online(Some(format!(
                                    "{}",
                                    packet.uid
                                ))));
                            }
                        }
                        None => {
                            let _ = game_db::save(GameData::player_online(Some(format!(
                                "{}",
                                packet.uid
                            ))));
                        }
                    }
                    // 更新玩家组ip地址
                    match game_db::find(GameData::player_group_addr(login_data.group, None)) {
                        Some(data) => {
                            if data.len() > 0 {
                                let _ = game_db::save(GameData::player_group_addr(
                                    login_data.group,
                                    Some(format!("{},{}", data, login_data.addr)),
                                ));
                            } else {
                                let _ = game_db::save(GameData::player_group_addr(
                                    login_data.group,
                                    Some(format!("{}", login_data.addr)),
                                ));
                            }
                        }
                        None => {
                            let _ = game_db::save(GameData::player_group_addr(
                                login_data.group,
                                Some(format!("{}", login_data.addr)),
                            ));
                        }
                    }
                }
                GameEvent::Logout(login_data) => {
                    println!("登出事件: {:?}", &login_data);
                    // 更新在线玩家表
                    match game_db::find(GameData::player_online(None)) {
                        Some(data) => {
                            if data.len() > 0 {
                                let mut uid_list: Vec<&str> = data.split(",").collect();
                                let mut rm_index = None;
                                for index in 0..uid_list.len() {
                                    if uid_list[index].eq(&packet.uid.to_string()) {
                                        rm_index = Some(index);
                                        break;
                                    }
                                }
                                if let Some(index) = rm_index {
                                    uid_list.remove(index);
                                    let _ = game_db::save(GameData::player_online(Some(
                                        uid_list.join(","),
                                    )));
                                }
                            }
                        }
                        None => {}
                    }
                    // 更新玩家组ip地址
                    match game_db::find(GameData::player_group_addr(login_data.group, None)) {
                        Some(data) => {
                            if data.len() > 0 {
                                let mut addr_list: Vec<&str> = data.split(",").collect();
                                let mut rm_index = None;
                                for index in 0..addr_list.len() {
                                    if addr_list[index].eq(&login_data.addr.to_string()) {
                                        rm_index = Some(index);
                                        break;
                                    }
                                }
                                if let Some(index) = rm_index {
                                    addr_list.remove(index);
                                    let _ = game_db::save(GameData::player_group_addr(
                                        login_data.group,
                                        Some(addr_list.join(",")),
                                    ));
                                }
                            }
                        }
                        None => {}
                    }
                }
                _ => println!("收到事件未处理: {:?}", packet.event),
            }
            // socket.send((Bytes::from("收到！"), addr)).await.unwrap();
        }
    }
}
