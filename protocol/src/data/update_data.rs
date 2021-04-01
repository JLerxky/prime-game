use super::Data;

// 状态同步数据
#[derive(Debug, Clone)]
pub struct UpdateData {
    // 16b[0..15]
    pub frame: u128,
    // x40b[16..]
    pub states: Vec<EntityState>,
}

const HEAD_LENGTH: usize = 16;

#[derive(Debug, Clone, Copy)]
// 46b
pub struct EntityState {
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
    // 5b[40..44]
    pub texture: (u32, u8),
    // 1b[45]
    pub entity_type: u8,
}

const ENTITY_STATE_LENGTH: usize = 46;

impl EntityState {
    pub fn make_up_data(&mut self, user_data: u128) {
        let data: [u8; 16] = user_data.to_le_bytes();

        let ptr: *const u8 = data[0..7].as_ptr();
        let ptr: *const u64 = ptr as *const u64;
        let id = unsafe { *ptr };

        let ptr: *const u8 = data[8..11].as_ptr();
        let ptr: *const u32 = ptr as *const u32;
        let texture0 = unsafe { *ptr };

        let texture1 = data[12];

        let entity_type = data[13];

        self.id = id;
        self.texture = (texture0, texture1);
        self.entity_type = entity_type;
    }
    pub fn get_data(&self) -> u128 {
        let mut data = self.id.to_le_bytes().to_vec();
        data.append(&mut self.texture.0.to_le_bytes().to_vec());
        data.append(&mut self.texture.1.to_le_bytes().to_vec());
        data.append(&mut self.entity_type.to_le_bytes().to_vec());
        data.append(&mut [0u8; 2].to_vec());

        let ptr: *const u8 = data[0..15].as_ptr();
        let ptr: *const u128 = ptr as *const u128;
        unsafe { *ptr }
    }
}

impl Data for UpdateData {
    fn data(&self) -> Vec<u8> {
        let mut route: Vec<u8> = Vec::new();
        route.append(&mut self.frame.to_le_bytes().to_vec());
        for rb in &self.states {
            route.append(&mut rb.id.to_le_bytes().to_vec());
            route.append(&mut rb.translation.0.to_le_bytes().to_vec());
            route.append(&mut rb.translation.1.to_le_bytes().to_vec());
            route.append(&mut rb.rotation.0.to_le_bytes().to_vec());
            route.append(&mut rb.rotation.1.to_le_bytes().to_vec());
            route.append(&mut rb.linvel.0.to_le_bytes().to_vec());
            route.append(&mut rb.linvel.1.to_le_bytes().to_vec());
            route.append(&mut rb.angvel.0.to_le_bytes().to_vec());
            route.append(&mut rb.angvel.1.to_le_bytes().to_vec());
            route.append(&mut rb.texture.0.to_le_bytes().to_vec());
            route.append(&mut rb.texture.1.to_le_bytes().to_vec());
            route.append(&mut rb.entity_type.to_le_bytes().to_vec());
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

        let state_no = (data.len() - HEAD_LENGTH) / ENTITY_STATE_LENGTH;

        for i in 0..state_no {
            let i = (i * ENTITY_STATE_LENGTH) + HEAD_LENGTH;
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

            let i = i + 4;
            let ptr: *const u8 = data[i..(i + 3)].as_ptr();
            let ptr: *const u32 = ptr as *const u32;
            let texture0 = unsafe { *ptr };

            let i = i + 4;
            let texture1 = data[i];

            let i = i + 1;
            let entity_type = data[i];

            states.push(EntityState {
                id,
                translation: (translation_x, translation_y),
                rotation: (rotation0, rotation1),
                linvel: (linvel_x, linvel_y),
                angvel: (angvel0, angvel1),
                texture: (texture0, texture1),
                entity_type,
            })
        }

        UpdateData { frame, states }
    }
}
