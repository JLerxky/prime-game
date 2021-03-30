use crate::data::{account_data::AccountData, update_data::UpdateData};

// 数据包二级路由[1]
// 心跳包路由
#[derive(Debug, Clone, Copy)]
pub enum HeartbeatRoute {
    In,
    Out,
    Keep,
}
// 账号中心路由
#[derive(Debug, Clone, Copy)]
pub enum AccountRoute {
    Login(AccountData),
    Logout(AccountData),
}
// 游戏路由
#[derive(Debug, Clone)]
pub enum GameRoute {
    Update(UpdateData),
}
