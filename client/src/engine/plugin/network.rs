use std::{
    io,
    sync::{Arc, Mutex},
};

use bevy::prelude::*;
use protocol::{
    data::{account_data::AccountData, control_data::ControlData, update_data::UpdateData},
    packet::Packet,
    route::{AccountRoute, GameRoute},
};
use tokio::{
    net::UdpSocket,
    sync::mpsc::{self, Receiver, Sender},
};

pub struct NetworkPlugin;

impl Plugin for NetworkPlugin {
    fn build(&self, app: &mut AppBuilder) {
        let (net_tx, net_rx) = mpsc::channel::<Packet>(1);
        let (engine_tx, engine_rx) = mpsc::channel::<Packet>(1);

        let update_data_list: Vec<UpdateData> = Vec::new();
        let update_data_list = Arc::new(Mutex::new(update_data_list));
        let update_data_list_c = update_data_list.clone();

        let control_queue: Vec<ControlData> = Vec::new();
        let control_queue = Arc::new(Mutex::new(control_queue));
        let control_queue_c = control_queue.clone();

        tokio::spawn(net_client_start(
            net_tx,
            engine_rx,
            update_data_list_c,
            control_queue_c,
        ));
        app.add_resource(NetWorkState {
            engine_tx,
            net_rx,
            update_data_list,
            control_queue,
        });
    }

    fn name(&self) -> &str {
        std::any::type_name::<Self>()
    }
}

async fn net_client_start(
    _net_tx: Sender<Packet>,
    _engine_rx: Receiver<Packet>,
    update_data_list: Arc<Mutex<Vec<UpdateData>>>,
    control_queue: Arc<Mutex<Vec<ControlData>>>,
) -> io::Result<()> {
    // 连接服务器
    println!("客户端网络连接ing...");
    let sock = UdpSocket::bind("0.0.0.0:0").await?;
    let remote_addr = common::config::SERVER_ADDR;
    sock.connect(remote_addr).await?;
    println!("客户端网络连接成功: {:?}", sock.local_addr());
    let r = Arc::new(sock);
    let s = r.clone();

    // 登录服务器
    s.send(
        &bincode::serialize(&Packet::Account(AccountRoute::Login(AccountData {
            uid: 4721,
            group: 0,
        })))
        .unwrap()[0..],
    )
    .await
    .unwrap();

    // tokio::spawn(async move {
    //     loop {
    //         if let Ok(control_queue) = control_queue.lock() {
    //             for control_data in control_queue.iter() {
    //                 s.send(
    //                     &bincode::serialize(&Packet::Game(GameRoute::Control(*control_data)))
    //                         .unwrap()[0..],
    //                 )
    //                 .await.unwrap();
    //             }
    //         }
    //     }
    // });

    // tokio::join!()(async move {
    //     while let Some(game_event) = engine_rx.recv().await {
    //         println!("网络模块收到引擎事件: {:?}", game_event);
    //         let len = s
    //             .send(&bincode::serialize(&game_event).unwrap()[0..])
    //             .await
    //             .unwrap();
    //         println!("网络客户端发送: {}", len);
    //     }
    // });

    // let mut interval = tokio::time::interval(tokio::time::Duration::from_secs_f64(1f64 / 5f64));
    let mut buf = [0; 1024];
    loop {
        // interval.tick().await;
        // println!("接收ing");
        if let Ok(len) = r.recv(&mut buf).await {
            // println!("接收来自服务器的 {:?} bytes", len);
            // let data_str = String::from_utf8_lossy(&buf[..len]);
            let packet = bincode::deserialize(&buf[..len]);
            // 转发事件
            if let Ok(packet) = packet {
                // let packet_c = packet.clone();
                match packet {
                    Packet::Game(game_route) => match game_route {
                        protocol::route::GameRoute::Update(update_data) => {
                            // let _ = tokio::join!(tx.send(packet_c));
                            // println!("接收来自服务器的Update事件");
                            if let Ok(mut update_data_list) = update_data_list.lock() {
                                if update_data_list.len() >= 10 {
                                    update_data_list.remove(0);
                                }
                                update_data_list.push(update_data);
                            }
                        }
                        _ => {}
                    },
                    _ => {}
                }
            }
        }

        if let Ok(mut control_queue) = control_queue.lock() {
            let control_queue_c = control_queue.clone();
            control_queue.clear();
            if let Some((last, elements)) = control_queue_c.split_last() {
                control_queue.push(*last);
                for control_data in elements.iter() {
                    let s = s.clone();
                    let control_data = control_data.clone();
                    tokio::spawn(async move {
                        s.send(
                            &bincode::serialize(&Packet::Game(GameRoute::Control(control_data)))
                                .unwrap()[0..],
                        )
                        .await
                        .unwrap();
                    });
                }
            }
        }
    }
}

pub struct NetWorkState {
    pub engine_tx: Sender<Packet>,
    pub net_rx: Receiver<Packet>,
    pub update_data_list: Arc<Mutex<Vec<UpdateData>>>,
    pub control_queue: Arc<Mutex<Vec<ControlData>>>,
}
