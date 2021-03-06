use data::server_db::{self, find_all_tile, GameData};
use protocol::{
    data::{
        control_data::ControlData, player_data::PlayerData, skill_data::SkillData,
        tile_map_data::TileMapData,
    },
    packet::Packet,
    route::GameRoute,
};
use tokio::net::UdpSocket;
use tokio::sync::mpsc::{Receiver, Sender};

use std::str::FromStr;
use std::{net::SocketAddr, sync::Arc, time::SystemTime};

pub async fn net_server_start(net_tx: Sender<Packet>, engine_rx: Receiver<Packet>) {
    if let Ok(game_server_socket) = UdpSocket::bind(config::SERVER_ADDR).await {
        if let Ok(game_server_addr) = &game_server_socket.local_addr() {
            println!("网络服务器已启动: {:?}", game_server_addr);

            let r = Arc::new(game_server_socket);
            let s = r.clone();

            // let game_server_framed = UdpFramed::new(**r, BytesCodec::new());

            let game_server_future = start_listening(r, s.clone(), net_tx);

            let wait_for_send_future = wait_for_send(s, engine_rx);
            let clean_offline_user_future = clean_offline_user();

            tokio::spawn(clean_offline_user_future);
            tokio::spawn(wait_for_send_future);
            game_server_future.await;
        }
    }
}

pub async fn send(socket: Arc<UdpSocket>, packet: Vec<u8>, recv_addr: SocketAddr) {
    let _ = socket.send_to(&packet[..], recv_addr).await;
}

pub async fn clean_offline_user() {
    let clean_tick = 5000u128;
    let mut interval = tokio::time::interval(tokio::time::Duration::from_millis(clean_tick as u64));
    loop {
        interval.tick().await;
        match server_db::find(GameData::player_group_addr(0, None)) {
            Ok(data) => {
                // println!("在线玩家列表: [{}]", data);
                if data.len() > 0 {
                    for addr_db in data.split(",") {
                        // 检查玩家ip地址健康检查状态
                        if let Ok(health) =
                            server_db::find(GameData::player_addr_health(addr_db.to_string(), None))
                        {
                            // println!("{}健康值: {}", addr_db, health);
                            if let Ok(h) = health.parse::<u128>() {
                                let now = SystemTime::now()
                                    .duration_since(SystemTime::UNIX_EPOCH)
                                    .unwrap()
                                    .as_millis();
                                if now > (h + clean_tick) {
                                    // println!("now: {}", now);
                                    if let Ok(uid) = server_db::find(GameData::player_addr_uid(
                                        addr_db.to_string(),
                                        None,
                                    )) {
                                        // println!("uid: {}", uid);
                                        if let Ok(uid_offline) = uid.parse::<u32>() {
                                            // println!("uid_offline: {}", uid_offline);
                                            // 更新在线玩家表
                                            match server_db::find(GameData::player_online(None)) {
                                                Ok(data) => {
                                                    if data.len() > 0 {
                                                        let mut uid_list: Vec<&str> =
                                                            data.split(",").collect();
                                                        let mut rm_index = None;
                                                        for index in 0..uid_list.len() {
                                                            if uid_list[index]
                                                                .eq(&uid_offline.to_string())
                                                            {
                                                                rm_index = Some(index);
                                                                break;
                                                            }
                                                        }
                                                        if let Some(index) = rm_index {
                                                            println!(
                                                                "清理未在线玩家uid: {}",
                                                                uid_list[index]
                                                            );
                                                            uid_list.remove(index);
                                                            let _ = server_db::save(
                                                                GameData::player_online(Some(
                                                                    uid_list.join(","),
                                                                )),
                                                            );
                                                        }
                                                    }
                                                }
                                                Err(_) => {}
                                            }
                                            // 更新玩家组ip地址
                                            match server_db::find(GameData::player_group_addr(
                                                0, None,
                                            )) {
                                                Ok(data) => {
                                                    if data.len() > 0 {
                                                        let mut addr_list: Vec<&str> =
                                                            data.split(",").collect();
                                                        let mut rm_index = None;
                                                        for index in 0..addr_list.len() {
                                                            if addr_list[index]
                                                                .eq(&addr_db.to_string())
                                                            {
                                                                rm_index = Some(index);
                                                                break;
                                                            }
                                                        }
                                                        if let Some(index) = rm_index {
                                                            println!(
                                                                "清理未在线玩家IP: {}",
                                                                addr_list[index]
                                                            );
                                                            addr_list.remove(index);
                                                            let _ = server_db::save(
                                                                GameData::player_group_addr(
                                                                    0,
                                                                    Some(addr_list.join(",")),
                                                                ),
                                                            );
                                                        }
                                                    }
                                                }
                                                Err(_) => {}
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
            Err(_) => {}
        }
    }
}

pub async fn multicast(socket: Arc<UdpSocket>, group: u32, packet: Vec<u8>) {
    // let mut senders = Vec::new();
    match server_db::find(GameData::player_group_addr(group, None)) {
        Ok(data) => {
            // println!("1");
            if data.len() > 0 {
                // println!("在线玩家IP: {}", &data);
                let uid_list: Vec<&str> = data.split(",").collect();
                for index in 0..uid_list.len() {
                    let recv_addr = SocketAddr::from_str(uid_list[index]).unwrap();
                    let socket = socket.clone();
                    let packet = packet.clone();
                    tokio::spawn(send(socket, packet, recv_addr));
                    // senders.push(sender);
                }
            }
        }
        Err(_e) => {
            // println!("{}", _e);
            // println!("{}组无玩家在线!", group);
        }
    }
    // for sender in senders {
    //     tokio::spawn(sender);
    // sender.await;
    // println!("sended");
    // }
    // println!("sended");
}

pub async fn wait_for_send(socket: Arc<UdpSocket>, mut engine_rx: Receiver<Packet>) {
    // let mut interval = tokio::time::interval(tokio::time::Duration::from_millis(10));
    loop {
        // interval.tick().await;
        if let Some(packet) = engine_rx.recv().await {
            // println!("{:?}", packet);
            let socket = socket.clone();
            multicast(socket, 0, bincode::serialize(&packet).unwrap()).await;
            // let _ = tokio::join!(multicast(socket, 0, bincode::serialize(&packet).unwrap()));
        }
    }
}

pub async fn start_listening(
    socket: Arc<UdpSocket>,
    send_socket: Arc<UdpSocket>,
    net_tx: Sender<Packet>,
) {
    let mut buf = [0; config::PACKET_SIZE];
    // let mut interval = tokio::time::interval(tokio::time::Duration::from_nanos(10));
    loop {
        // interval.tick().await;
        // println!("接收ing");
        if let Ok((len, addr)) = socket.recv_from(&mut buf).await {
            // println!("服务器收到数据: {}", &len);
            if let Ok(packet) = bincode::deserialize::<Packet>(&buf[..len]) {
                // println!("服务器收到数据: {:?}", &packet);
                // 转发事件
                match packet {
                    Packet::Heartbeat(heartbeat_route) => match heartbeat_route {
                        protocol::route::HeartbeatRoute::In => {}
                        protocol::route::HeartbeatRoute::Out => {}
                        protocol::route::HeartbeatRoute::Keep(_) => {
                            // 回ping
                            let _ =
                                tokio::spawn(send(send_socket.clone(), buf[..len].to_vec(), addr));
                            // 添加玩家ip地址健康检查状态
                            let _ = server_db::save(GameData::player_addr_health(
                                addr.to_string(),
                                Some(format!(
                                    "{}",
                                    SystemTime::now()
                                        .duration_since(SystemTime::UNIX_EPOCH)
                                        .unwrap()
                                        .as_millis()
                                )),
                            ));
                        }
                    },
                    Packet::Account(account_route) => match account_route {
                        protocol::route::AccountRoute::Login(account_data) => {
                            println!("{}登录事件: {:?}", &addr, &account_data);
                            // 根据玩家ip注册或获取uid
                            let mut uid = account_data.uid;
                            match server_db::find(GameData::player_addr_uid(addr.to_string(), None))
                            {
                                Ok(data) => {
                                    if let Ok(id) = data.parse::<u32>() {
                                        uid = id;
                                    } else {
                                        if let Ok(id) =
                                            server_db::next_u64(GameData::player_queue_uid(None))
                                        {
                                            uid = id as u32;
                                            println!("新玩家注册: {}", uid);
                                            let _ = server_db::save(GameData::player_addr_uid(
                                                addr.to_string(),
                                                Some(uid.to_string()),
                                            ));
                                            let _ = server_db::save_player(PlayerData {
                                                uid,
                                                hp: 100,
                                                mp: 100,
                                                max_hp: 100,
                                                max_mp: 100,
                                            });
                                        }
                                    }
                                }
                                Err(_) => {
                                    if let Ok(id) =
                                        server_db::next_u64(GameData::player_queue_uid(None))
                                    {
                                        uid = id as u32;
                                        println!("新玩家注册: {}", uid);
                                        let _ = server_db::save(GameData::player_addr_uid(
                                            addr.to_string(),
                                            Some(uid.to_string()),
                                        ));
                                        let _ = server_db::save_player(PlayerData {
                                            uid,
                                            hp: 100,
                                            mp: 100,
                                            max_hp: 100,
                                            max_mp: 100,
                                        });
                                    }
                                }
                            }
                            // 更新在线玩家表
                            match server_db::find(GameData::player_online(None)) {
                                Ok(data) => {
                                    if data.len() > 0 {
                                        let mut exist = false;
                                        for uid_db in data.split(",") {
                                            if uid_db.eq(&uid.to_string()) {
                                                exist = true;
                                                break;
                                            }
                                        }
                                        if !exist {
                                            let _ = server_db::save(GameData::player_online(Some(
                                                format!("{},{}", data, uid),
                                            )));
                                        }
                                    } else {
                                        let _ = server_db::save(GameData::player_online(Some(
                                            format!("{}", uid),
                                        )));
                                    }
                                }
                                Err(_) => {
                                    let _ = server_db::save(GameData::player_online(Some(
                                        format!("{}", uid),
                                    )));
                                }
                            }
                            // 更新玩家组ip地址
                            match server_db::find(GameData::player_group_addr(
                                account_data.group,
                                None,
                            )) {
                                Ok(data) => {
                                    if data.len() > 0 {
                                        let mut exist = false;
                                        for addr_db in data.split(",") {
                                            if addr_db.eq(&addr.to_string()) {
                                                exist = true;
                                                break;
                                            }
                                        }
                                        if !exist {
                                            let _ = server_db::save(GameData::player_group_addr(
                                                account_data.group,
                                                Some(format!("{},{}", data, addr)),
                                            ));
                                        }
                                    } else {
                                        let _ = server_db::save(GameData::player_group_addr(
                                            account_data.group,
                                            Some(format!("{}", addr)),
                                        ));
                                    }
                                }
                                Err(_) => {
                                    let _ = server_db::save(GameData::player_group_addr(
                                        account_data.group,
                                        Some(format!("{}", addr)),
                                    ));
                                }
                            }
                            // 添加玩家ip地址健康检查状态
                            let _ = server_db::save(GameData::player_addr_health(
                                addr.to_string(),
                                Some(format!(
                                    "{}",
                                    SystemTime::now()
                                        .duration_since(SystemTime::UNIX_EPOCH)
                                        .unwrap()
                                        .as_millis()
                                )),
                            ));
                            // 发送生成玩家实体事件到引擎
                            let packet_login =
                                Packet::Account(protocol::route::AccountRoute::Login(
                                    protocol::data::account_data::AccountData {
                                        uid,
                                        group: account_data.group,
                                    },
                                ));
                            let _ = net_tx.try_send(packet_login.clone());
                            let _ = tokio::spawn(send(
                                send_socket.clone(),
                                bincode::serialize(&packet_login).unwrap(),
                                addr,
                            ));
                        }
                        protocol::route::AccountRoute::Logout(account_data) => {
                            println!("{}登出事件: {:?}", &addr, &account_data);
                            // 根据玩家ip获取uid
                            let uid;
                            match server_db::find(GameData::player_addr_uid(addr.to_string(), None))
                            {
                                Ok(data) => {
                                    if let Ok(id) = data.parse::<u32>() {
                                        uid = id;
                                    } else {
                                        continue;
                                    }
                                }
                                Err(e) => {
                                    println!("报错: {}", e);
                                    continue;
                                }
                            }
                            // 更新在线玩家表
                            match server_db::find(GameData::player_online(None)) {
                                Ok(data) => {
                                    if data.len() > 0 {
                                        let mut uid_list: Vec<&str> = data.split(",").collect();
                                        let mut rm_index = None;
                                        for index in 0..uid_list.len() {
                                            if uid_list[index].eq(&uid.to_string()) {
                                                rm_index = Some(index);
                                                break;
                                            }
                                        }
                                        if let Some(index) = rm_index {
                                            uid_list.remove(index);
                                            let _ = server_db::save(GameData::player_online(Some(
                                                uid_list.join(","),
                                            )));
                                        }
                                    }
                                }
                                Err(_) => {}
                            }
                            // 更新玩家组ip地址
                            match server_db::find(GameData::player_group_addr(
                                account_data.group,
                                None,
                            )) {
                                Ok(data) => {
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
                                            let _ = server_db::save(GameData::player_group_addr(
                                                account_data.group,
                                                Some(addr_list.join(",")),
                                            ));
                                        }
                                    }
                                }
                                Err(_) => {}
                            }
                        }
                        protocol::route::AccountRoute::GetInfo(account_data) => {
                            // 根据玩家ip获取uid
                            let mut uid = account_data.uid;
                            match server_db::find(GameData::player_addr_uid(addr.to_string(), None))
                            {
                                Ok(data) => {
                                    if let Ok(id) = data.parse::<u32>() {
                                        uid = id;
                                    }
                                }
                                Err(_) => {}
                            }
                            // 发送生成玩家实体事件到引擎
                            let packet_login =
                                Packet::Account(protocol::route::AccountRoute::Login(
                                    protocol::data::account_data::AccountData {
                                        uid,
                                        group: account_data.group,
                                    },
                                ));
                            let _ = tokio::spawn(send(
                                send_socket.clone(),
                                bincode::serialize(&packet_login).unwrap(),
                                addr,
                            ));
                        }
                    },
                    Packet::Game(game_route) => match game_route {
                        protocol::route::GameRoute::Control(control_data) => {
                            // println!("{}控制: {:?}", &addr, &control_data);
                            // 根据玩家ip注册或获取uid
                            let uid;
                            match server_db::find(GameData::player_addr_uid(addr.to_string(), None))
                            {
                                Ok(data) => {
                                    if let Ok(id) = data.parse::<u32>() {
                                        uid = id;
                                    } else {
                                        continue;
                                    }
                                }
                                Err(e) => {
                                    println!("{}报错: {}", &addr, e);
                                    continue;
                                }
                            }
                            let packet_control = Packet::Game(GameRoute::Control(ControlData {
                                uid,
                                direction: control_data.direction,
                                action: control_data.action,
                            }));
                            if let Ok(_) = net_tx.try_send(packet_control) {
                                // println!("{}转递控制: {:?}", &addr, &control_data);
                            }
                        }
                        protocol::route::GameRoute::Tile(tile_state) => {
                            if let Ok(tile) = server_db::find_tile_map(tile_state.point) {
                                let packet_tile =
                                    Packet::Game(protocol::route::GameRoute::Tile(tile));
                                // println!("send: {:?}", &packet_tile);
                                let _ = tokio::spawn(send(
                                    send_socket.clone(),
                                    bincode::serialize(&packet_tile).unwrap(),
                                    addr,
                                ));
                            }
                        }
                        GameRoute::Skill(skill_data) => {
                            let uid;
                            match server_db::find(GameData::player_addr_uid(addr.to_string(), None))
                            {
                                Ok(data) => {
                                    if let Ok(id) = data.parse::<u32>() {
                                        uid = id;
                                    } else {
                                        continue;
                                    }
                                }
                                Err(e) => {
                                    println!("{}报错: {}", &addr, e);
                                    continue;
                                }
                            }
                            let packet_control = Packet::Game(GameRoute::Skill(SkillData {
                                uid,
                                direction: skill_data.direction,
                                skill_type: skill_data.skill_type,
                                texture: skill_data.texture,
                            }));
                            if let Ok(_) = net_tx.try_send(packet_control) {
                                // println!("{}转递控制: {:?}", &addr, &control_data);
                            }
                        }
                        GameRoute::TileMap(_tile_map_data) => {
                            let tiles = find_all_tile();
                            let mut tiles_iter = tiles.chunks(40);
                            while let Some(t) = tiles_iter.next() {
                                let packet_tile = Packet::Game(
                                    protocol::route::GameRoute::TileMap(TileMapData {
                                        map_id: 0,
                                        tiles: t.to_vec(),
                                    }),
                                );
                                let _ = tokio::spawn(send(
                                    send_socket.clone(),
                                    bincode::serialize(&packet_tile).unwrap(),
                                    addr,
                                ));
                            }
                        }
                        GameRoute::Update(_) => {}
                        GameRoute::Player(_) => {}
                        GameRoute::PlayerList(_) => {}
                    },
                    // _ => println!("{}收到事件未处理: {:?}", &addr, &packet),
                }
                // socket.send((Bytes::from("收到！"), addr)).await.unwrap();
            }
        }
    }
}
