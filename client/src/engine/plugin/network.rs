use std::net::SocketAddr;

use bevy::prelude::*;
use common::GameEvent;
use tokio::{
    sync::mpsc::{self, Receiver, Sender},
};

pub struct NetworkPlugin;

impl Plugin for NetworkPlugin {
    fn build(&self, app: &mut AppBuilder) {
        let (tx, rx) = mpsc::channel::<GameEvent>(1024);
        tokio::spawn(test(tx, rx));
        let (tx, rx) = mpsc::channel::<GameEvent>(1024);
        app.add_resource(SenderState { tx, rx });
    }

    fn name(&self) -> &str {
        std::any::type_name::<Self>()
    }
}

async fn test(tx: Sender<GameEvent>, rx: Receiver<GameEvent>) {
    println!("{:?}", tx);
    println!("{:?}", rx);
}

pub struct SenderState {
    tx: Sender<GameEvent>,
    rx: Receiver<GameEvent>,
}
