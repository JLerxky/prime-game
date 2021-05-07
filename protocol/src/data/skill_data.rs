use serde::{Deserialize, Serialize};

// 状态同步数据
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
// 13b
pub struct SkillData {
    pub uid: u32,
    pub direction: (f32, f32),
    pub skill_type: SkillType,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum SkillType {
    Shot = 0,
}

impl From<u8> for SkillType {
    fn from(num: u8) -> Self {
        match num {
            0 => SkillType::Shot,
            _ => SkillType::Shot,
        }
    }
}
