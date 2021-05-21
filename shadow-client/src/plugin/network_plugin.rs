use std::{
    io,
    sync::{Arc, Mutex},
    time::SystemTime,
};

use bevy::prelude::*;
use protocol::{
    data::{account_data::AccountData, player_data::PlayerData, update_data::EntityType},
    packet::Packet,
    route::{AccountRoute, GameRoute, HeartbeatRoute},
};
use tokio::net::UdpSocket;

use crate::event::sync_event::SyncEvent;

// 当前玩家
pub static mut PLAYER: PlayerData = PlayerData {
    uid: 0,
    hp: 0,
    mp: 0,
    max_hp: 100,
    max_mp: 100,
};

pub struct NetWorkState {
    pub packet_queue: Arc<Mutex<Vec<Packet>>>,
    pub to_be_sent_queue: Arc<Mutex<Vec<Packet>>>,
}

pub struct SynEntity {
    pub id: u64,
    pub entity_type: EntityType,
    pub health: u64,
    pub animate_type: u8,
    pub animate_index: usize,
}

pub struct NetworkPlugin;

impl Plugin for NetworkPlugin {
    fn build(&self, app: &mut AppBuilder) {
        let packet_queue: Vec<Packet> = Vec::new();
        let packet_queue = Arc::new(Mutex::new(packet_queue));
        let packet_queue_c = packet_queue.clone();

        let to_be_sent_queue: Vec<Packet> = Vec::new();
        let to_be_sent_queue = Arc::new(Mutex::new(to_be_sent_queue));
        let to_be_sent_queue_c = to_be_sent_queue.clone();

        tokio::spawn(net_client_start(packet_queue_c, to_be_sent_queue_c));
        app.insert_resource(NetWorkState {
            packet_queue,
            to_be_sent_queue,
        })
        .add_system(net_handler_system.system());
    }
}

fn net_handler_system(
    net_state: ResMut<NetWorkState>,
    mut sync_event_writer: EventWriter<SyncEvent>,
) {
    if let Ok(mut packet_queue) = net_state.packet_queue.lock() {
        // println!("packet_queue: {}", packet_queue.len());
        for _ in 0..10 {
            if packet_queue.is_empty() {
                return;
            }
            let packet = packet_queue[0].clone();
            packet_queue.remove(0);

            match packet {
                Packet::Game(game_route) => match game_route {
                    GameRoute::Update(update_data) => {
                        println!(
                            "数据包: {:?}, 位置: {:?}",
                            update_data.frame, update_data.states[0].translation
                        );
                        sync_event_writer.send(SyncEvent { update_data });
                    }
                    _ => {}
                },
                _ => {}
            }
        }
    }
}

async fn net_client_start(
    packet_queue: Arc<Mutex<Vec<Packet>>>,
    to_be_sent_queue: Arc<Mutex<Vec<Packet>>>,
) -> io::Result<()> {
    // 连接服务器
    println!("客户端网络连接ing...");
    let sock = UdpSocket::bind(config::CLIENT_ADDR).await?;
    let remote_addr = config::SERVER_ADDR;
    sock.connect(remote_addr).await?;
    println!("客户端网络连接成功: {:?}", sock.local_addr());
    let r = Arc::new(sock);
    let s = r.clone();

    // 登录服务器
    s.send(
        &bincode::serialize(&Packet::Account(AccountRoute::Login(AccountData {
            uid: 0,
            group: 0,
        })))
        .unwrap()[0..],
    )
    .await
    .unwrap();

    tokio::spawn(async move {
        let mut interval = tokio::time::interval(tokio::time::Duration::from_secs_f64(2f64));
        loop {
            interval.tick().await;
            s.send(
                &bincode::serialize(&Packet::Heartbeat(HeartbeatRoute::Keep(
                    SystemTime::now()
                        .duration_since(SystemTime::UNIX_EPOCH)
                        .unwrap()
                        .as_millis(),
                )))
                .unwrap()[0..],
            )
            .await
            .unwrap();

            unsafe {
                if PLAYER.uid == 0 {
                    s.send(
                        &bincode::serialize(&Packet::Account(AccountRoute::GetInfo(AccountData {
                            uid: 0,
                            group: 0,
                        })))
                        .unwrap()[0..],
                    )
                    .await
                    .unwrap();
                }
            }
            // println!("发送Heartbeat");
        }
    });

    let mut buf = [0; config::PACKET_SIZE];
    loop {
        // println!("接收ing");
        if let Ok(len) = r.recv(&mut buf).await {
            // println!("接收来自服务器的 {:?} bytes", len);
            let packet = bincode::deserialize(&buf[..len]);
            // 转发事件
            if let Ok(packet) = packet {
                if let Ok(mut packet_queue) = packet_queue.lock() {
                    if packet_queue.len() > 512 {
                        packet_queue.remove(0);
                    }
                    packet_queue.push(packet);
                }
            }
        }

        if let Ok(mut to_be_sent_queue) = to_be_sent_queue.lock() {
            let to_be_sent_queue_c = to_be_sent_queue.clone();
            to_be_sent_queue.clear();
            for to_be_sent_packet in to_be_sent_queue_c.iter() {
                let s = r.clone();
                let to_be_sent_packet = to_be_sent_packet.clone();
                tokio::spawn(async move {
                    s.send(&bincode::serialize(&to_be_sent_packet).unwrap()[0..])
                        .await
                        .unwrap();
                });
            }
        }
    }
}
