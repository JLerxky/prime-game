use std::{
    io,
    sync::{Arc, Mutex},
};

use bevy::prelude::*;
use protocol::{
    data::{account_data::AccountData, update_data::RigidBodyState},
    route::AccountRoute,
    Packet,
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
        let rb_states: Vec<RigidBodyState> = Vec::new();
        let rb_states = Arc::new(Mutex::new(rb_states));
        let rb_states_c = rb_states.clone();
        tokio::spawn(net_client_start(net_tx, engine_rx, rb_states_c));
        app.add_resource(NetWorkState {
            engine_tx,
            net_rx,
            rb_states,
        });
    }

    fn name(&self) -> &str {
        std::any::type_name::<Self>()
    }
}

async fn net_client_start(
    tx: Sender<Packet>,
    mut rx: Receiver<Packet>,
    mut rb_states: Arc<Mutex<Vec<RigidBodyState>>>,
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
        &Packet::Account(AccountRoute::Login(AccountData {
            uid: 4721,
            group: 0,
        }))
        .to_bytes()[0..],
    )
    .await
    .unwrap();

    tokio::spawn(async move {
        while let Some(game_event) = rx.recv().await {
            println!("网络模块收到引擎事件: {:?}", game_event);
            let len = s.send(b"1").await.unwrap();
            println!("网络客户端发送: {}", "1");
        }
    });

    // let mut interval = tokio::time::interval(tokio::time::Duration::from_secs_f64(1f64 / 5f64));
    let mut buf = [0; 1024];
    loop {
        // interval.tick().await;
        // println!("接收ing");
        let mut rb_states = rb_states.lock().unwrap();
        if let Ok(len) = r.try_recv(&mut buf) {
            // println!("接收来自服务器的 {:?} bytes", len);
            // let data_str = String::from_utf8_lossy(&buf[..len]);
            let packet = Packet::decode(&buf[..len]);
            // 转发事件
            if let Some(packet) = packet {
                let packet_c = packet.clone();
                match packet {
                    Packet::Game(game_route) => match game_route {
                        protocol::route::GameRoute::Update(mut update_data) => {
                            // let _ = tokio::join!(tx.send(packet_c));
                            // println!("接收来自服务器的Update事件");
                            rb_states.append(&mut update_data.states);
                        }
                    },
                    _ => {}
                }
            }
        }
    }
}

pub struct NetWorkState {
    pub engine_tx: Sender<Packet>,
    pub net_rx: Receiver<Packet>,
    pub rb_states: Arc<Mutex<Vec<RigidBodyState>>>,
}
