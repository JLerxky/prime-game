use crate::data::game_db::{self, GameData};
use protocol::packet::Packet;
use tokio::net::UdpSocket;
use tokio::sync::mpsc::{Receiver, Sender};

use std::{error::Error, str::FromStr};
use std::{net::SocketAddr, sync::Arc};

pub async fn start_server(
    net_tx: Sender<Packet>,
    engine_rx: Receiver<Packet>,
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
    packet: &[u8],
    recv_addr: SocketAddr,
) -> Result<(), Box<dyn Error>> {
    socket.send_to(packet, recv_addr).await?;
    // println!("send ok");

    Ok(())
}

pub async fn multicast(
    socket: Arc<UdpSocket>,
    group: u32,
    packet: Vec<u8>,
) -> Result<(), Box<dyn Error>> {
    match game_db::find(GameData::player_group_addr(group, None)) {
        Some(data) => {
            if data.len() > 0 {
                let uid_list: Vec<&str> = data.split(",").collect();
                for index in 0..uid_list.len() {
                    let recv_addr = SocketAddr::from_str(uid_list[index])?;
                    let socket = socket.clone();
                    let _ = tokio::join!(send(socket, &packet, recv_addr));
                }
            }
        }
        None => {
            // println!("{}组无玩家在线!", group);
        }
    }
    Ok(())
}

async fn wait_for_send(socket: Arc<UdpSocket>, mut engine_rx: Receiver<Packet>) {
    loop {
        while let Some(packet) = engine_rx.recv().await {
            // println!("{:?}", game_event);
            let socket = socket.clone();
            let _ = tokio::join!(multicast(socket, 0, bincode::serialize(&packet).unwrap()));
        }
    }
}

async fn start_listening(
    socket: Arc<UdpSocket>,
    send_socket: Arc<UdpSocket>,
    net_tx: Sender<Packet>,
) {
    let mut buf = [0; 1024];
    loop {
        if let Ok((len, addr)) = socket.recv_from(&mut buf).await {
            // println!("服务器收到数据: {}", &len);
            if let Ok(packet) = bincode::deserialize::<Packet>(&buf[..len]) {
                // 转发事件
                let packet_1 = packet.clone();
                let packet_2 = packet.clone();
                match packet {
                    Packet::Heartbeat(heartbeat_route) => match heartbeat_route {
                        protocol::route::HeartbeatRoute::In => {}
                        protocol::route::HeartbeatRoute::Out => {}
                        protocol::route::HeartbeatRoute::Keep(_) => {
                            let _ = tokio::join!(send(send_socket.clone(), &buf[..len], addr));
                        }
                    },
                    Packet::Account(account_route) => match account_route {
                        protocol::route::AccountRoute::Login(account_data) => {
                            println!("{}登录事件: {:?}", &addr, &account_data);
                            let _ = net_tx.try_send(packet_1);
                            let _ = tokio::join!(send(send_socket.clone(), &buf[..len], addr));
                            // 更新在线玩家表
                            match game_db::find(GameData::player_online(None)) {
                                Some(data) => {
                                    if data.len() > 0 {
                                        let mut exist = false;
                                        for uid_db in data.split(",") {
                                            if uid_db.eq(&account_data.uid.to_string()) {
                                                exist = true;
                                                break;
                                            }
                                        }
                                        if !exist {
                                            let _ = game_db::save(GameData::player_online(Some(
                                                format!("{},{}", data, account_data.uid),
                                            )));
                                        }
                                    } else {
                                        let _ = game_db::save(GameData::player_online(Some(
                                            format!("{}", account_data.uid),
                                        )));
                                    }
                                }
                                None => {
                                    let _ = game_db::save(GameData::player_online(Some(format!(
                                        "{}",
                                        account_data.uid
                                    ))));
                                }
                            }
                            // 更新玩家组ip地址
                            match game_db::find(GameData::player_group_addr(
                                account_data.group,
                                None,
                            )) {
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
                                                account_data.group,
                                                Some(format!("{},{}", data, addr)),
                                            ));
                                        }
                                    } else {
                                        let _ = game_db::save(GameData::player_group_addr(
                                            account_data.group,
                                            Some(format!("{}", addr)),
                                        ));
                                    }
                                }
                                None => {
                                    let _ = game_db::save(GameData::player_group_addr(
                                        account_data.group,
                                        Some(format!("{}", addr)),
                                    ));
                                }
                            }
                        }
                        protocol::route::AccountRoute::Logout(account_data) => {
                            println!("{}登出事件: {:?}", &addr, &account_data);
                            // 更新在线玩家表
                            match game_db::find(GameData::player_online(None)) {
                                Some(data) => {
                                    if data.len() > 0 {
                                        let mut uid_list: Vec<&str> = data.split(",").collect();
                                        let mut rm_index = None;
                                        for index in 0..uid_list.len() {
                                            if uid_list[index].eq(&account_data.uid.to_string()) {
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
                            match game_db::find(GameData::player_group_addr(
                                account_data.group,
                                None,
                            )) {
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
                                                account_data.group,
                                                Some(addr_list.join(",")),
                                            ));
                                        }
                                    }
                                }
                                None => {}
                            }
                        }
                    },
                    Packet::Game(game_route) => match game_route {
                        protocol::route::GameRoute::Control(control_data) => {
                            println!("{}控制: {:?}", &addr, &control_data);
                            let _ = net_tx.try_send(packet_2);
                        }
                        _ => {}
                    },
                    // _ => println!("{}收到事件未处理: {:?}", &addr, &packet),
                }
                // socket.send((Bytes::from("收到！"), addr)).await.unwrap();
            }
        }
    }
}
