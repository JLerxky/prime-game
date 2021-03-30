use super::Data;

// 状态同步数据
#[derive(Debug)]
pub struct UpdateData {
    // 长度16b[2..17]
    pub frame: u128,
    // 长度16b[18..33]
    pub id: u128,
    // 长度8b[34..41]
    pub translation: [f32; 2],
    // 长度8b[42..49]
    pub rotation: [f32; 2],
}

impl Data for UpdateData {
    fn data(&self) -> Vec<u8> {
        let mut route: Vec<u8> = Vec::new();
        route.append(&mut self.frame.to_ne_bytes().to_vec());
        route.append(&mut self.id.to_ne_bytes().to_vec());
        route.append(&mut self.translation[0].to_ne_bytes().to_vec());
        route.append(&mut self.translation[1].to_ne_bytes().to_vec());
        route.append(&mut self.rotation[0].to_ne_bytes().to_vec());
        route.append(&mut self.rotation[1].to_ne_bytes().to_vec());
        route
    }
}

impl UpdateData {
    pub fn from(data: Vec<u8>) -> Self {
        let ptr: *const u8 = data[0..15].as_ptr();
        let ptr: *const u128 = ptr as *const u128;
        let frame = unsafe { *ptr };

        let ptr: *const u8 = data[16..31].as_ptr();
        let ptr: *const u128 = ptr as *const u128;
        let id = unsafe { *ptr };

        let ptr: *const u8 = data[32..35].as_ptr();
        let ptr: *const f32 = ptr as *const f32;
        let translation0 = unsafe { *ptr };

        let ptr: *const u8 = data[36..39].as_ptr();
        let ptr: *const f32 = ptr as *const f32;
        let translation1 = unsafe { *ptr };

        let ptr: *const u8 = data[40..43].as_ptr();
        let ptr: *const f32 = ptr as *const f32;
        let rotation0 = unsafe { *ptr };

        let ptr: *const u8 = data[44..47].as_ptr();
        let ptr: *const f32 = ptr as *const f32;
        let rotation1 = unsafe { *ptr };
        UpdateData {
            frame,
            id,
            translation: [translation0, translation1],
            rotation: [rotation0, rotation1],
        }
    }
}
