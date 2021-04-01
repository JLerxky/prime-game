use serde::{Deserialize, Serialize};

// 状态同步数据
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct AccountData {
    // 4b[0..3]
    pub uid: u32,
    // 4b[4..7]
    pub group: u32,
}

