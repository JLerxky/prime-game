pub const SERVER_PORT: u16 = 2101;
pub const PACKET_SIZE: usize = 2048;
/// 服务器地址
// PC
// pub const SERVER_ADDR: &str = "192.168.101.198:2101";
// ROG JLer-0
// pub const SERVER_ADDR: &str = "192.168.101.17:2101";
// ROG JLer
// pub const SERVER_ADDR: &str = "192.168.101.220:2101";
// ROG 网线
pub const SERVER_ADDR: &str = "192.168.101.228:2101";
// 野兽
// pub const SERVER_ADDR: &str = "180.115.194.174:2101";
// 发布 server
// pub const SERVER_ADDR: &str = "0.0.0.0:2101";
// Mac USB
// pub const SERVER_ADDR: &str = "172.20.10.5:2101";
// 阿里云
// pub const SERVER_ADDR: &str = "47.114.179.240:2101";
// 腾讯云
// pub const SERVER_ADDR: &str = "jler.vip:2101";
/// 客户端地址
pub const CLIENT_ADDR: &str = "0.0.0.0:0";
/// 帧间时间
pub const INTER_FRAME_TIME: f64 = 1f64 / 60f64;
/// 服务器数据库文件目录
pub const DB_PATH_SERVER: &str = "db_data/db_server";
/// 客户端数据库文件目录
pub const DB_PATH_CLIENT: &str = "db_data/db_client";
