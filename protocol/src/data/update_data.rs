use super::Data;

// 状态同步数据
#[derive(Debug)]
pub struct UpdateData {
    // 16b[0..15]
    pub frame: u128,
    // x40b[16..]
    pub states: Vec<RigidBodyState>,
}

#[derive(Debug)]
// 40b
pub struct RigidBodyState {
    // 8b[0..7]
    pub id: u64,
    // 8b[8..15]
    pub translation: (f32, f32),
    // 8b[16..23]
    pub rotation: (f32, f32),
    // 8b[24..31]
    pub linvel: (f32, f32),
    // 8b[32..39]
    pub angvel: (f32, f32),
}

impl Data for UpdateData {
    fn data(&self) -> Vec<u8> {
        let mut route: Vec<u8> = Vec::new();
        route.append(&mut self.frame.to_ne_bytes().to_vec());
        for rb in &self.states {
            route.append(&mut rb.id.to_ne_bytes().to_vec());
            route.append(&mut rb.translation.0.to_ne_bytes().to_vec());
            route.append(&mut rb.translation.1.to_ne_bytes().to_vec());
            route.append(&mut rb.rotation.0.to_ne_bytes().to_vec());
            route.append(&mut rb.rotation.1.to_ne_bytes().to_vec());
            route.append(&mut rb.linvel.0.to_ne_bytes().to_vec());
            route.append(&mut rb.linvel.1.to_ne_bytes().to_vec());
            route.append(&mut rb.angvel.0.to_ne_bytes().to_vec());
            route.append(&mut rb.angvel.1.to_ne_bytes().to_vec());
        }
        route
    }
}

impl UpdateData {
    pub fn from(data: Vec<u8>) -> Self {
        let ptr: *const u8 = data[0..15].as_ptr();
        let ptr: *const u128 = ptr as *const u128;
        let frame = unsafe { *ptr };
        let mut states = Vec::new();

        let state_no = (data.len() - 16) / 40;

        for i in 0..state_no {
            let i = i + (i * 39) + 16;
            let ptr: *const u8 = data[i..(i + 7)].as_ptr();
            let ptr: *const u64 = ptr as *const u64;
            let id = unsafe { *ptr };

            let i = i + 8;
            let ptr: *const u8 = data[i..(i + 3)].as_ptr();
            let ptr: *const f32 = ptr as *const f32;
            let translation_x = unsafe { *ptr };

            let i = i + 4;
            let ptr: *const u8 = data[i..(i + 3)].as_ptr();
            let ptr: *const f32 = ptr as *const f32;
            let translation_y = unsafe { *ptr };

            let i = i + 4;
            let ptr: *const u8 = data[i..(i + 3)].as_ptr();
            let ptr: *const f32 = ptr as *const f32;
            let rotation0 = unsafe { *ptr };

            let i = i + 4;
            let ptr: *const u8 = data[i..(i + 3)].as_ptr();
            let ptr: *const f32 = ptr as *const f32;
            let rotation1 = unsafe { *ptr };

            let i = i + 4;
            let ptr: *const u8 = data[i..(i + 3)].as_ptr();
            let ptr: *const f32 = ptr as *const f32;
            let linvel_x = unsafe { *ptr };

            let i = i + 4;
            let ptr: *const u8 = data[i..(i + 3)].as_ptr();
            let ptr: *const f32 = ptr as *const f32;
            let linvel_y = unsafe { *ptr };

            let i = i + 4;
            let ptr: *const u8 = data[i..(i + 3)].as_ptr();
            let ptr: *const f32 = ptr as *const f32;
            let angvel0 = unsafe { *ptr };

            let i = i + 4;
            let ptr: *const u8 = data[i..(i + 3)].as_ptr();
            let ptr: *const f32 = ptr as *const f32;
            let angvel1 = unsafe { *ptr };

            states.push(RigidBodyState {
                id,
                translation: (translation_x, translation_y),
                rotation: (rotation0, rotation1),
                linvel: (linvel_x, linvel_y),
                angvel: (angvel0, angvel1),
            })
        }

        UpdateData { frame, states }
    }
}
