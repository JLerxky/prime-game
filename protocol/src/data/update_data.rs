use serde::{Deserialize, Serialize};

// 状态同步数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateData {
    // 16b[0..15]
    pub frame: u128,
    // x40b[16..]
    pub states: Vec<EntityState>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
// 46b
pub struct EntityState {
    // 8b[0..7]
    pub id: u64,
    // 8b[8..15]
    pub translation: (f32, f32),
    // 8b[16..23]
    pub rotation: f32,
    // 8b[24..31]
    pub linvel: (f32, f32),
    // 8b[32..39]
    pub angvel: (f32, f32),
    // 5b[40..44]
    pub texture: (u32, u8, u8),
    // 1b[45]
    pub entity_type: EntityType,
    pub animate: u8,
}

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

        let texture2 = data[13];

        let entity_type = data[14];
        let animate = data[15];

        self.id = id;
        self.texture = (texture0, texture1, texture2);
        self.entity_type = EntityType::from(entity_type);
        self.animate = animate;
    }
    pub fn get_data(&self) -> u128 {
        let mut data = self.id.to_le_bytes().to_vec();
        data.append(&mut self.texture.0.to_le_bytes().to_vec());
        data.append(&mut self.texture.1.to_le_bytes().to_vec());
        data.append(&mut self.texture.2.to_le_bytes().to_vec());
        data.append(&mut (self.entity_type as u8).to_le_bytes().to_vec());
        data.append(&mut self.animate.to_le_bytes().to_vec());
        data.append(&mut [0u8; 2].to_vec());
        let ptr: *const u8 = data[0..15].as_ptr();
        let ptr: *const u128 = ptr as *const u128;
        unsafe { *ptr }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum EntityType {
    Static = 0,
    Moveable = 1,
    Player = 2,
}

impl From<u8> for EntityType {
    fn from(num: u8) -> Self {
        match num {
            0 => EntityType::Static,
            1 => EntityType::Moveable,
            2 => EntityType::Player,
            _ => EntityType::Static,
        }
    }
}
