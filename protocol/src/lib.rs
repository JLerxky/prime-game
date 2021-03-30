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
    let packet: Packet = Packet::Game(GameRoute::Update(data::update_data::UpdateData {
        frame: 0,
        id: 1,
        translation: (1.534563456f32, 0.132412f32),
        rotation: (2f32, 2f32),
        linvel: (3f32, 3f32),
        angvel: (4f32, 4f32),
    }));
    println!("{:?}", packet.to_bytes().len());
    println!("{:?}", packet.to_bytes()[2..].len());
    println!(
        "{:?}",
        data::update_data::UpdateData::from(packet.to_bytes()[2..].to_vec())
    );
}
