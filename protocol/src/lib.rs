use data::{account_data::AccountData, update_data::UpdateData, Data};
use route::{AccountRoute, GameRoute, HeartbeatRoute};

pub mod data;
pub mod route;

// 数据包一级路由[0]
#[derive(Debug, Clone)]
pub enum Packet {
    Heartbeat(HeartbeatRoute),
    Account(AccountRoute),
    Game(GameRoute),
}

impl Packet {
    pub fn to_bytes(&self) -> Vec<u8> {
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
                    AccountRoute::Login(data) => {
                        route.push(0);
                        route.append(&mut data.data());
                    }
                    AccountRoute::Logout(data) => {
                        route.push(1);
                        route.append(&mut data.data());
                    }
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
    pub fn decode(data: &[u8]) -> Option<Packet> {
        let route_1 = data[0];
        let route_2 = data[1];
        match route_1 {
            0 => match route_2 {
                0 => {
                    return Some(Packet::Heartbeat(HeartbeatRoute::In));
                }
                1 => {
                    return Some(Packet::Heartbeat(HeartbeatRoute::Out));
                }
                2 => {
                    return Some(Packet::Heartbeat(HeartbeatRoute::Keep));
                }
                _ => {}
            },
            1 => match route_2 {
                0 => {
                    return Some(Packet::Account(AccountRoute::Login(AccountData::from(
                        data[2..].to_vec(),
                    ))));
                }
                1 => {
                    return Some(Packet::Account(AccountRoute::Logout(AccountData::from(
                        data[2..].to_vec(),
                    ))));
                }
                _ => {}
            },
            2 => match route_2 {
                0 => {
                    return Some(Packet::Game(GameRoute::Update(UpdateData::from(
                        data[2..].to_vec(),
                    ))));
                }
                _ => {}
            },
            _ => {}
        }
        None
    }
}

#[test]
fn test() {
    // let i: u128 = 340282366920938463463374607431768211455;
    // println!("{}{}", Packet::Heartbeat as u8, i / 120);
    let mut states = Vec::new();
    states.push(data::update_data::EntityState {
        id: 1,
        translation: (1., 1.),
        rotation: (1., 1.),
        linvel: (1., 1.),
        angvel: (1., 1.),
        texture: (1, 1),
        entity_type: 1,
    });
    states.push(data::update_data::EntityState {
        id: 2,
        translation: (2., 2.),
        rotation: (2., 2.),
        linvel: (2., 2.),
        angvel: (2., 2.),
        texture: (2, 2),
        entity_type: 2,
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

#[test]
fn test_1() {
    let user_data_b = 0u128.to_be_bytes();
    println!("数据长度{}", user_data_b.len());
    // 以大端（网络）字节顺序的字节数组形式返回此整数的内存表示形式
    println!("be: {:?}", 10u128.to_be_bytes());
    // 以字节数组的形式返回这个整数的内存表示形式，以小的字节顺序
    println!("le: {:?}", 10u128.to_le_bytes());
    // 以本机字节顺序的字节数组形式返回此整数的内存表示形式。
    // 由于使用了目标平台的本机endianness，因此可移植代码应该使用to_be_bytes或to_le_bytes（视情况而定）。
    println!("ne: {:?}", 10u128.to_ne_bytes());
}
