// 坐标
#[derive(Copy, Clone, Debug)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}

// 瓷砖
#[derive(Clone, Debug)]
pub struct Tile {
    // 文件名作为name
    pub name: String,
    // 旋转 0-0 1-90 2-180 3-270
    pub rotation: u8,
    // 可连接id
    pub top: u32,
    pub down: u32,
    pub left: u32,
    pub right: u32,
}

// 位置
#[derive(Clone, Debug)]
pub struct Slot {
    // 位置
    pub position: Position,
    // 是否坍缩
    pub is_collapsed: bool,
    // 叠加态（可选瓷砖集合）
    pub superposition: Vec<Tile>,
    // 熵
    pub entropy: u64,
    // 确定态（当前瓷砖）
    pub tile: Option<Tile>,
}

impl Slot {
    pub fn new() -> Slot {
        Slot {
            position: Position { x: 0, y: 0 },
            is_collapsed: false,
            superposition: vec![],
            entropy: 0,
            tile: None,
        }
    }
}