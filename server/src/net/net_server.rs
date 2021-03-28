use crate::data::game_db::{self, GameData};
use common::{GameEvent, Packet};
use tokio::net::UdpSocket;
use tokio::sync::mpsc::{Receiver, Sender};

use bytes::Bytes;
use std::{error::Error, str::FromStr};
use std::{net::SocketAddr, sync::Arc};

pub async fn start_server(
    net_tx: Sender<GameEvent>,
    engine_rx: Receiver<GameEvent>,
) -> Result<(), Box<dyn Error>> {
    let game_server_socket = UdpSocket::bind(common::config::SERVER_ADDR).await?;
    let game_server_addr = &game_server_socket.local_addr()?;

    println!("网络服务器已启动: {:?}", game_server_addr);

    let r = Arc::new(game_server_socket);
    let s = r.clone();

    // let game_server_framed = UdpFramed::new(**r, BytesCodec::new());

    let game_server_future = start_listening(r, s.clone(), net_tx);

    let wait_for_send_future = wait_for_send(s, engine_rx);

    tokio::join!(game_server_future, wait_for_send_future);

    Ok(())
}

pub async fn send(
    socket: Arc<UdpSocket>,
    packet: String,
    recv_addr: SocketAddr,
) -> Result<(), Box<dyn Error>> {
    socket.send_to(&Bytes::from(packet), recv_addr).await?;
    // println!("send ok");

    Ok(())
}

pub async fn multicast(
    socket: Arc<UdpSocket>,
    group: u32,
    packet: String,
) -> Result<(), Box<dyn Error>> {
    match game_db::find(GameData::player_group_addr(group, None)) {
        Some(data) => {
            if data.len() > 0 {
                let uid_list: Vec<&str> = data.split(",").collect();
                for index in 0..uid_list.len() {
                    let recv_addr = SocketAddr::from_str(uid_list[index])?;
                    let socket = socket.clone();
                    let _ = tokio::join!(send(socket, packet.clone(), recv_addr));
                }
            }
        }
        None => {
            // println!("{}组无玩家在线!", group);
        }
    }
    Ok(())
}

async fn wait_for_send(socket: Arc<UdpSocket>, mut engine_rx: Receiver<GameEvent>) {
    loop {
        while let Some(game_event) = engine_rx.recv().await {
            // println!("{:?}", game_event);
            let packet = Packet {
                uid: 0,
                event: game_event,
            };
            let socket = socket.clone();
            let _ = tokio::join!(multicast(
                socket,
                0,
                serde_json::to_string(&packet).unwrap()
            ));
        }
    }
}

async fn start_listening(
    socket: Arc<UdpSocket>,
    send_socket: Arc<UdpSocket>,
    net_tx: Sender<GameEvent>,
) {
    let mut buf = [0; 1024];
    loop {
        if let Ok((len, addr)) = socket.recv_from(&mut buf).await {
            let data_str = String::from_utf8_lossy(&buf[..len]);
            println!("服务器收到数据: {:?}", &data_str);
            let packet = serde_json::from_slice(data_str.as_bytes()).unwrap_or(Packet {
                uid: 0,
                event: GameEvent::Default,
            });
            // 转发事件
            match packet.event {
                GameEvent::Login(login_data) => {
                    println!("{}登录事件: {:?}", &addr, &login_data);
                    let _ = net_tx.try_send(packet.event);
                    let _ = tokio::join!(send(send_socket.clone(), "packet".to_string(), addr));
                    // 更新在线玩家表
                    match game_db::find(GameData::player_online(None)) {
                        Some(data) => {
                            if data.len() > 0 {
                                let mut exist = false;
                                for uid_db in data.split(",") {
                                    if uid_db.eq(&packet.uid.to_string()) {
                                        exist = true;
                                        break;
                                    }
                                }
                                if !exist {
                                    let _ = game_db::save(GameData::player_online(Some(format!(
                                        "{},{}",
                                        data, packet.uid
                                    ))));
                                }
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
                                let mut exist = false;
                                for addr_db in data.split(",") {
                                    if addr_db.eq(&addr.to_string()) {
                                        exist = true;
                                        break;
                                    }
                                }
                                if !exist {
                                    let _ = game_db::save(GameData::player_group_addr(
                                        login_data.group,
                                        Some(format!("{},{}", data, addr)),
                                    ));
                                }
                            } else {
                                let _ = game_db::save(GameData::player_group_addr(
                                    login_data.group,
                                    Some(format!("{}", addr)),
                                ));
                            }
                        }
                        None => {
                            let _ = game_db::save(GameData::player_group_addr(
                                login_data.group,
                                Some(format!("{}", addr)),
                            ));
                        }
                    }
                }
                GameEvent::Logout(login_data) => {
                    println!("{}登出事件: {:?}", &addr, &login_data);
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
                                    if addr_list[index].eq(&addr.to_string()) {
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
                _ => println!("{}收到事件未处理: {:?}", &addr, packet.event),
            }
            // socket.send((Bytes::from("收到！"), addr)).await.unwrap();
        }
    }
}
