use data::Data;
use route::{AccountRoute, GameRoute, HeartbeatRoute};

pub mod data;
pub mod route;

// 数据包一级路由[0]
pub enum Packet {
    Heartbeat(HeartbeatRoute),
    Account(AccountRoute),
    Game(GameRoute),
}

impl Packet {
    fn to_bytes(&self) -> Vec<u8> {
        let mut route = Vec::new();
        match self {
            Packet::Heartbeat(r) => {
                route.push(0);
                match r {
                    HeartbeatRoute::In => route.push(0),
                    HeartbeatRoute::Out => {}
                    HeartbeatRoute::Keep => {}
                }
            }
            Packet::Account(r) => {
                route.push(1);
                match r {
                    AccountRoute::Login => route.push(0),
                    AccountRoute::Logout => route.push(1),
                }
            }
            Packet::Game(r) => {
                route.push(2);
                match r {
                    GameRoute::Update(data) => {
                        route.push(0);
                        route.append(&mut data.data());
                    }
                }
            }
        }
        route
    }
}

#[test]
fn test() {
    // let i: u128 = 340282366920938463463374607431768211455;
    // println!("{}{}", Packet::Heartbeat as u8, i / 120);
    let mut states = Vec::new();
    states.push(data::update_data::RigidBodyState {
        id: 1,
        translation: (1., 1.),
        rotation: (1., 1.),
        linvel: (1., 1.),
        angvel: (1., 1.),
    });
    states.push(data::update_data::RigidBodyState {
        id: 2,
        translation: (2., 2.),
        rotation: (2., 2.),
        linvel: (2., 2.),
        angvel: (2., 2.),
    });
    states.push(data::update_data::RigidBodyState {
        id: 3,
        translation: (3., 3.),
        rotation: (3., 3.),
        linvel: (3., 3.),
        angvel: (3., 3.),
    });
    states.push(data::update_data::RigidBodyState {
        id: 4,
        translation: (4., 4.),
        rotation: (4., 4.),
        linvel: (4., 4.),
        angvel: (4., 4.),
    });
    states.push(data::update_data::RigidBodyState {
        id: 5,
        translation: (5., 5.),
        rotation: (5., 5.),
        linvel: (5., 5.),
        angvel: (5., 5.),
    });
    states.push(data::update_data::RigidBodyState {
        id: 6,
        translation: (6., 6.),
        rotation: (6., 6.),
        linvel: (6., 6.),
        angvel: (6., 6.),
    });
    states.push(data::update_data::RigidBodyState {
        id: 7,
        translation: (7., 7.),
        rotation: (7., 7.),
        linvel: (7., 7.),
        angvel: (7., 7.),
    });
    states.push(data::update_data::RigidBodyState {
        id: 8,
        translation: (8., 8.),
        rotation: (8., 8.),
        linvel: (8., 8.),
        angvel: (8., 8.),
    });
    states.push(data::update_data::RigidBodyState {
        id: 9,
        translation: (9., 9.),
        rotation: (9., 9.),
        linvel: (9., 9.),
        angvel: (9., 9.),
    });
    states.push(data::update_data::RigidBodyState {
        id: 10,
        translation: (10., 10.),
        rotation: (10., 10.),
        linvel: (10., 10.),
        angvel: (10., 10.),
    });
    let packet: Packet = Packet::Game(GameRoute::Update(data::update_data::UpdateData {
        frame: 1,
        states,
    }));
    let packet = packet.to_bytes();
    println!("{}", packet.len());
    println!("{:?}", packet);
    println!(
        "{:?}",
        data::update_data::UpdateData::from(packet[2..].to_vec())
    );
}
