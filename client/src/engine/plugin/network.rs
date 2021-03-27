use std::{io, sync::Arc};

use bevy::prelude::*;
use common::{GameEvent, LoginData, Packet, UpdateData};
use tokio::{
    net::UdpSocket,
    sync::mpsc::{self, Receiver, Sender},
};

pub struct NetworkPlugin;

impl Plugin for NetworkPlugin {
    fn build(&self, app: &mut AppBuilder) {
        let (net_tx, net_rx) = mpsc::channel::<GameEvent>(1024);
        let (engine_tx, engine_rx) = mpsc::channel::<GameEvent>(1024);
        tokio::spawn(net_client_start(net_tx, engine_rx));
        app.add_resource(NetWorkState { engine_tx, net_rx });
    }

    fn name(&self) -> &str {
        std::any::type_name::<Self>()
    }
}

async fn net_client_start(tx: Sender<GameEvent>, mut rx: Receiver<GameEvent>) -> io::Result<()> {
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
        serde_json::to_string(&Packet {
            uid: 21,
            event: GameEvent::Login(LoginData { group: 0 }),
        })
        .unwrap()
        .as_bytes(),
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

    let mut buf = [0; 1024];
    loop {
        println!("开始接收服务端数据ing");
        let len = r.recv(&mut buf).await?;
        println!("接收来自服务器的 {:?} bytes", len);
        // tx.send(GameEvent::Update(UpdateData {
        //     id: 21,
        //     translation: [1., 1.],
        //     rotation: [0., 0.],
        // }))
        // .await
        // .unwrap();
    }
}

pub struct NetWorkState {
    pub engine_tx: Sender<GameEvent>,
    pub net_rx: Receiver<GameEvent>,
}
