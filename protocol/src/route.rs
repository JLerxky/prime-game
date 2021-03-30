use crate::data::update_data::UpdateData;

// 数据包二级路由[1]
// 心跳包路由
pub enum HeartbeatRoute {
    In,
    Out,
    Keep,
}
// 账号中心路由
pub enum AccountRoute {
    Login,
    Logout,
}
// 游戏路由
pub enum GameRoute {
    Update(UpdateData),
}
