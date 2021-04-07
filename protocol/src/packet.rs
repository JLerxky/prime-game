use crate::route::{AccountRoute, GameRoute, HeartbeatRoute};
use serde::{Deserialize, Serialize};
// 数据包一级路由[0]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Packet {
    Heartbeat(HeartbeatRoute),
    Account(AccountRoute),
    Game(GameRoute),
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

#[test]
fn test_bincode() {
    use crate::data::control_data::ControlData;
    let packet: Packet = Packet::Game(GameRoute::Control(ControlData {
        uid: 0,
        direction: (-1., 0.11),
        action: 1,
    }));
    let packet = bincode::serialize(&packet).unwrap();
    println!("{}", packet.len());
    println!("{:?}", packet);
    println!("{:?}", bincode::deserialize::<Packet>(&packet[..]).unwrap());
}
