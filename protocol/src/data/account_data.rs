use super::Data;

// 状态同步数据
#[derive(Debug, Clone, Copy)]
pub struct AccountData {
    // 4b[0..3]
    pub uid: u32,
    // 4b[4..7]
    pub group: u32,
}

impl Data for AccountData {
    fn data(&self) -> Vec<u8> {
        let mut route: Vec<u8> = Vec::new();
        route.append(&mut self.uid.to_le_bytes().to_vec());
        route.append(&mut self.group.to_le_bytes().to_vec());
        route
    }
}

impl AccountData {
    pub fn from(data: Vec<u8>) -> Self {
        let ptr: *const u8 = data[0..3].as_ptr();
        let ptr: *const u32 = ptr as *const u32;
        let uid = unsafe { *ptr };

        let ptr: *const u8 = data[4..7].as_ptr();
        let ptr: *const u32 = ptr as *const u32;
        let group = unsafe { *ptr };

        AccountData { uid, group }
    }
}
