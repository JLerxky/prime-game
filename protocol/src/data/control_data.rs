use super::Data;

// 状态同步数据
#[derive(Debug, Clone, Copy)]
// 13b
pub struct ControlData {
    // 4b[0..3]
    pub uid: u32,
    // 8b[4..11] 方向 模拟输入
    pub direction: (f32, f32),
    // 1b[12] 动作 0停止, 1移动, 2跳跃
    pub action: u8,
}

impl Data for ControlData {
    fn data(&self) -> Vec<u8> {
        let mut route: Vec<u8> = Vec::new();
        route.append(&mut self.uid.to_le_bytes().to_vec());
        route.append(&mut self.direction.0.to_le_bytes().to_vec());
        route.append(&mut self.direction.1.to_le_bytes().to_vec());
        route.append(&mut self.action.to_le_bytes().to_vec());
        route
    }
}

impl ControlData {
    pub fn from(data: Vec<u8>) -> Self {
        let ptr: *const u8 = data[0..3].as_ptr();
        let ptr: *const u32 = ptr as *const u32;
        let uid = unsafe { *ptr };

        let ptr: *const u8 = data[4..7].as_ptr();
        let ptr: *const f32 = ptr as *const f32;
        let direction0 = unsafe { *ptr };

        let ptr: *const u8 = data[8..11].as_ptr();
        let ptr: *const f32 = ptr as *const f32;
        let direction1 = unsafe { *ptr };

        let action = data[12];

        ControlData {
            uid,
            direction: (direction0, direction1),
            action,
        }
    }
}
