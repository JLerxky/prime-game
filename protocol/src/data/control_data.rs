use serde::{Deserialize, Serialize};

// 状态同步数据
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
// 13b
pub struct ControlData {
    // 4b[0..3]
    pub uid: u32,
    // 8b[4..11] 方向 模拟输入
    pub direction: (f32, f32),
    // 1b[12] 动作 0停止, 1移动, 2跳跃
    pub action: u8,
}
