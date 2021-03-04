use std::io::Error;

use rand::Rng;

// 坐标
#[derive(Copy, Clone, Debug)]
struct Position {
    x: u32,
    y: u32,
}

// 瓷砖
#[derive(Clone, Debug)]
struct Tile {
    // 文件名作为name
    name: String,
    // 旋转 0-0 1-90 2-180 3-270
    rotation: u8,
    // 可连接id
    top: u32,
    down: u32,
    left: u32,
    right: u32,
}

// 位置
#[derive(Clone, Debug)]
pub struct Slot {
    // 位置
    position: Position,
    // 是否坍缩
    is_collapsed: bool,
    // 叠加态（可选瓷砖集合）
    superposition: Vec<Tile>,
    // 熵
    entropy: usize,
    // 确定态（当前瓷砖）
    tile: Option<Tile>,
}

pub fn init(x: u32, y: u32) -> Vec<Slot> {
    let mut tiles: Vec<Tile> = Vec::new();

    // 草地
    tiles.push(Tile {
        name: "generic-rpg-Slice".to_string(),
        rotation: 0,
        top: 0,
        down: 0,
        left: 0,
        right: 0,
    });

    tiles.push(Tile {
        name: "generic-rpg-tile02".to_string(),
        rotation: 0,
        top: 0,
        down: 1,
        left: 1,
        right: 11,
    });

    tiles.push(Tile {
        name: "generic-rpg-tile04".to_string(),
        rotation: 0,
        top: 0,
        down: 1,
        left: 11,
        right: 1,
    });

    tiles.push(Tile {
        name: "generic-rpg-tile31".to_string(),
        rotation: 0,
        top: 0,
        down: 1,
        left: 1,
        right: 0,
    });

    tiles.push(Tile {
        name: "generic-rpg-tile71".to_string(),
        rotation: 0,
        top: 1,
        down: 1,
        left: 1,
        right: 1,
    });

    tiles.push(Tile {
        name: "generic-rpg-tile61".to_string(),
        rotation: 0,
        top: 1,
        down: 0,
        left: 0,
        right: 1,
    });

    let mut slots: Vec<Slot> = Vec::new();
    for i in 0..x {
        for j in 0..y {
            let position = Position { x: i, y: j };
            slots.push(Slot {
                position,
                is_collapsed: false,
                superposition: tiles.clone(),
                entropy: tiles.len(),
                tile: None,
            });
        }
    }
    slots
}

pub fn random_collapse(slots: &mut Vec<Slot>) -> Result<(), ()> {
    let mut rng = rand::thread_rng();
    for slot in slots {
        if slot.position.x == 0 && slot.position.y == 0 {
            if let Some(tile) = slot
                .superposition
                .get(rng.gen_range(0, slot.superposition.len()))
            {
                slot.tile = Some(tile.clone());
                slot.superposition = Vec::new();
                slot.entropy = 0;
                slot.is_collapsed = true;
                return Ok(());
            };
        }
    }
    Err(())
}

pub fn collapse(slots: &mut Vec<Slot>) {
    let mut rng = rand::thread_rng();
    // TODO 计算熵 for entropy in 0..x
    for slot in slots {

    }
}

#[test]
fn test() {
    use std::time::Instant;
    let start_time = Instant::now();

    let slots = init(10, 10);

    let time = start_time.elapsed().as_secs_f64();
    println!("{:?}", slots);
    println!("{}", time);
}
