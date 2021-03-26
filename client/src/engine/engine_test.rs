use bevy::{
    app::{App, EventReader, Events, ScheduleRunnerSettings},
    core::Time,
    ecs::prelude::*,
    MinimalPlugins,
};
use bevy_networking_turbulence::{
    ConnectionHandle, NetworkEvent, NetworkResource, NetworkingPlugin, Packet,
};
use common::{GameEvent, LoginData};

use std::{net::SocketAddr, time::Duration};

const SERVER_PORT: u16 = 2101;

pub struct Args {
    pub is_server: bool,
}

pub fn parse_args() -> Args {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        panic!("Need to select to run as either a server (--server) or a client (--client).");
    }

    let connection_type = &args[1];

    let is_server = match connection_type.as_str() {
        "--server" | "-s" => true,
        "--client" | "-c" => false,
        _ => panic!("Need to select to run as either a server (--server) or a client (--client)."),
    };

    Args { is_server }
}

pub fn engine_start() {
    App::build()
        // minimal plugins necessary for timers + headless loop
        .add_resource(ScheduleRunnerSettings::run_loop(Duration::from_secs_f64(
            1.0 / 60.0,
        )))
        .add_plugins(MinimalPlugins)
        // The NetworkingPlugin
        .add_plugin(NetworkingPlugin::default())
        .add_event::<NetEvent>()
        // Our networking
        .add_resource(parse_args())
        .add_startup_system(startup.system())
        .add_system(send_packets.system())
        .init_resource::<NetworkReader>()
        .add_system(handle_packets.system())
        .run();
}

fn startup(mut net: ResMut<NetworkResource>, args: Res<Args>) {
    let socket_address = SocketAddr::new(
        std::net::IpAddr::V4(std::net::Ipv4Addr::new(192, 168, 101, 198)),
        SERVER_PORT,
    );
    if args.is_server {
        println!("Starting server");
        net.listen(socket_address);
    }
    if !args.is_server {
        println!("Starting client");
        net.connect(socket_address);
    }
}

fn send_packets(mut net: ResMut<NetworkResource>, time: Res<Time>, args: Res<Args>) {
    if !args.is_server {
        if (time.seconds_since_startup() * 60.) as i64 % 60 == 0 {
            // println!("PING");
            let packet = common::Packet {
                uid: 21,
                event: GameEvent::Login(LoginData { group: 0 }),
            };
            net.broadcast(Packet::from(serde_json::to_string(&packet).unwrap()));
        }
    }
}

#[derive(Default)]
struct NetworkReader {
    network_events: EventReader<NetworkEvent>,
}

#[derive(Debug)]
pub enum NetEvent {
    Connected(ConnectionHandle),
    Disconnected(ConnectionHandle),
    Packet(ConnectionHandle, common::Packet),
}

fn handle_packets(
    mut net: ResMut<NetworkResource>,
    time: Res<Time>,
    mut state: ResMut<NetworkReader>,
    network_events: Res<Events<NetworkEvent>>,
) {
    for event in state.network_events.iter(&network_events) {
        println!("event [{:?}]", event);
        match event {
            NetworkEvent::Packet(handle, packet) => {
                let message = String::from_utf8_lossy(packet);
                println!("Got packet on [{}]: {}", handle, message);
                // let message = format!("PONG @ {}", time.seconds_since_startup());
                // match net.send(*handle, Packet::from(message)) {
                //     Ok(()) => {
                //         println!("Sent PONG");
                //     }
                //     Err(error) => {
                //         println!("PONG send error: {}", error);
                //     }
                // }
            }
            
            NetworkEvent::Connected(_) => {}
            NetworkEvent::Disconnected(_) => {}
        }
    }
}
