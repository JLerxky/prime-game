use bevy::prelude::*;
use common::GameEvent;
use tokio::sync::mpsc::{self, Receiver, Sender};

pub struct NetworkPlugin;

impl Plugin for NetworkPlugin {
    fn build(&self, app: &mut AppBuilder) {
        let (net_tx, net_rx) = mpsc::channel::<GameEvent>(1024);
        let (engine_tx, engine_rx) = mpsc::channel::<GameEvent>(1024);
        tokio::spawn(test(net_tx, engine_rx));
        app.add_resource(SenderState { engine_tx, net_rx });
    }

    fn name(&self) -> &str {
        std::any::type_name::<Self>()
    }
}

async fn test(tx: Sender<GameEvent>, mut rx: Receiver<GameEvent>) {
    while let Some(game_event) = rx.recv().await {
        println!("网络模块收到: {:?}", game_event);
    }
}

pub struct SenderState {
    pub engine_tx: Sender<GameEvent>,
    pub net_rx: Receiver<GameEvent>,
}
