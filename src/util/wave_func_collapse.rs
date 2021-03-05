use std::io::Error;

use rand::Rng;

// 坐标
#[derive(Copy, Clone, Debug)]
pub struct Position {
    x: u8,
    y: u8,
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

impl Slot {
    pub fn new() -> Slot {
        Slot{
            position: Position{ x: 0, y: 0},
            is_collapsed: false,
            superposition: vec![],
            entropy: 0,
            tile: None,
            
        }
    }
}

pub fn init() -> [[Slot; 10]; 10] {
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

    let mut slots: [[Slot; 10]; 10] = [[Slot::new(), Slot::new(),Slot::new(),Slot::new(),Slot::new(),Slot::new(),Slot::new(),Slot::new(),Slot::new(),Slot::new(),],[Slot::new(),Slot::new(),Slot::new(),Slot::new(),Slot::new(),Slot::new(),Slot::new(),Slot::new(),Slot::new(),Slot::new()],[Slot::new(),Slot::new(),Slot::new(),Slot::new(),Slot::new(),Slot::new(),Slot::new(),Slot::new(),Slot::new(),Slot::new()],[Slot::new(),Slot::new(),Slot::new(),Slot::new(),Slot::new(),Slot::new(),Slot::new(),Slot::new(),Slot::new(),Slot::new()],[Slot::new(),Slot::new(),Slot::new(),Slot::new(),Slot::new(),Slot::new(),Slot::new(),Slot::new(),Slot::new(),Slot::new()],[Slot::new(),Slot::new(),Slot::new(),Slot::new(),Slot::new(),Slot::new(),Slot::new(),Slot::new(),Slot::new(),Slot::new()],[Slot::new(),Slot::new(),Slot::new(),Slot::new(),Slot::new(),Slot::new(),Slot::new(),Slot::new(),Slot::new(),Slot::new()],[Slot::new(),Slot::new(),Slot::new(),Slot::new(),Slot::new(),Slot::new(),Slot::new(),Slot::new(),Slot::new(),Slot::new()],[Slot::new(),Slot::new(),Slot::new(),Slot::new(),Slot::new(),Slot::new(),Slot::new(),Slot::new(),Slot::new(),Slot::new()],[Slot::new(),Slot::new(),Slot::new(),Slot::new(),Slot::new(),Slot::new(),Slot::new(),Slot::new(),Slot::new(),Slot::new()],];
    for i in 0..10 {
        for j in 0..10 {
            let position = Position { x: i, y: j };
            slots[usize::from(i)][usize::from(j)] = Slot {
                position,
                is_collapsed: false,
                superposition: tiles.clone(),
                entropy: tiles.len(),
                tile: None,
                
            };
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

pub fn collapse(position: Position, slots: &mut [[Slot; 10]; 10]) -> Result<(),String>{
    if position.x < 0 || position.x >= 10 || position.y < 0 || position.y >= 10 {
        return Err("位置不合法".to_string());
    }
    let mut rng = rand::thread_rng();
    let mut slot_center = slots[usize::from(position.x)][usize::from(position.y)].clone();
    // TODO 计算熵
    if position.x - 1 >= 0 {
        let slot_top = slots[usize::from(position.x - 1)][usize::from(position.y)].clone();
        for tile_top in slot_top.superposition {
            let iter = slot_center.superposition.iter_mut();
            // while let Some(tile_center) = iter.next() {
            //     if tile_top.down != tile_center.top {
        //     }
            // }
        }
    }
    let slot_down = slots[usize::from(position.x)][usize::from(position.y)].clone();
    let slot_left = slots[usize::from(position.x)][usize::from(position.y)].clone();
    let slot_right = slots[usize::from(position.x)][usize::from(position.y)].clone();

    slots[usize::from(position.x)][usize::from(position.y)] = slot_center;
    Ok(())
}

#[test]
fn test() {
    use std::time::Instant;
    let start_time = Instant::now();

    let slots = init();

    let time = start_time.elapsed().as_secs_f64();
    println!("{:?}", slots);
    println!("{}", time);
}
