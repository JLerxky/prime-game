use rand::Rng;

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
    pub entropy: usize,
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

pub fn init() -> [[Slot; 10]; 10] {
    let mut tiles: Vec<Tile> = Vec::new();

    // 草地
    tiles.push(Tile {
        name: "1".to_string(),
        rotation: 0,
        top: 1,
        down: 0,
        left: 0,
        right: 1,
    });

    tiles.push(Tile {
        name: "2".to_string(),
        rotation: 0,
        top: 0,
        down: 0,
        left: 1,
        right: 1,
    });

    tiles.push(Tile {
        name: "3".to_string(),
        rotation: 0,
        top: 0,
        down: 1,
        left: 1,
        right: 0,
    });

    tiles.push(Tile {
        name: "4".to_string(),
        rotation: 0,
        top: 1,
        down: 1,
        left: 0,
        right: 0,
    });

    tiles.push(Tile {
        name: "5".to_string(),
        rotation: 0,
        top: 0,
        down: 1,
        left: 0,
        right: 1,
    });

    tiles.push(Tile {
        name: "6".to_string(),
        rotation: 0,
        top: 1,
        down: 0,
        left: 1,
        right: 0,
    });

    tiles.push(Tile {
        name: "7".to_string(),
        rotation: 0,
        top: 1,
        down: 1,
        left: 1,
        right: 0,
    });

    tiles.push(Tile {
        name: "8".to_string(),
        rotation: 0,
        top: 0,
        down: 0,
        left: 0,
        right: 0,
    });

    tiles.push(Tile {
        name: "9".to_string(),
        rotation: 0,
        top: 1,
        down: 1,
        left: 1,
        right: 1,
    });

    tiles.push(Tile {
        name: "10".to_string(),
        rotation: 0,
        top: 1,
        down: 1,
        left: 1,
        right: 1,
    });

    tiles.push(Tile {
        name: "11".to_string(),
        rotation: 0,
        top: 1,
        down: 0,
        left: 1,
        right: 1,
    });

    tiles.push(Tile {
        name: "12".to_string(),
        rotation: 0,
        top: 1,
        down: 1,
        left: 0,
        right: 1,
    });

    tiles.push(Tile {
        name: "13".to_string(),
        rotation: 0,
        top: 0,
        down: 1,
        left: 1,
        right: 1,
    });

    let mut slots: [[Slot; 10]; 10] = [
        [
            Slot::new(),
            Slot::new(),
            Slot::new(),
            Slot::new(),
            Slot::new(),
            Slot::new(),
            Slot::new(),
            Slot::new(),
            Slot::new(),
            Slot::new(),
        ],
        [
            Slot::new(),
            Slot::new(),
            Slot::new(),
            Slot::new(),
            Slot::new(),
            Slot::new(),
            Slot::new(),
            Slot::new(),
            Slot::new(),
            Slot::new(),
        ],
        [
            Slot::new(),
            Slot::new(),
            Slot::new(),
            Slot::new(),
            Slot::new(),
            Slot::new(),
            Slot::new(),
            Slot::new(),
            Slot::new(),
            Slot::new(),
        ],
        [
            Slot::new(),
            Slot::new(),
            Slot::new(),
            Slot::new(),
            Slot::new(),
            Slot::new(),
            Slot::new(),
            Slot::new(),
            Slot::new(),
            Slot::new(),
        ],
        [
            Slot::new(),
            Slot::new(),
            Slot::new(),
            Slot::new(),
            Slot::new(),
            Slot::new(),
            Slot::new(),
            Slot::new(),
            Slot::new(),
            Slot::new(),
        ],
        [
            Slot::new(),
            Slot::new(),
            Slot::new(),
            Slot::new(),
            Slot::new(),
            Slot::new(),
            Slot::new(),
            Slot::new(),
            Slot::new(),
            Slot::new(),
        ],
        [
            Slot::new(),
            Slot::new(),
            Slot::new(),
            Slot::new(),
            Slot::new(),
            Slot::new(),
            Slot::new(),
            Slot::new(),
            Slot::new(),
            Slot::new(),
        ],
        [
            Slot::new(),
            Slot::new(),
            Slot::new(),
            Slot::new(),
            Slot::new(),
            Slot::new(),
            Slot::new(),
            Slot::new(),
            Slot::new(),
            Slot::new(),
        ],
        [
            Slot::new(),
            Slot::new(),
            Slot::new(),
            Slot::new(),
            Slot::new(),
            Slot::new(),
            Slot::new(),
            Slot::new(),
            Slot::new(),
            Slot::new(),
        ],
        [
            Slot::new(),
            Slot::new(),
            Slot::new(),
            Slot::new(),
            Slot::new(),
            Slot::new(),
            Slot::new(),
            Slot::new(),
            Slot::new(),
            Slot::new(),
        ],
    ];
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

pub fn random_collapse(slot: &mut Slot) -> Result<(), ()> {
    let mut rng = rand::thread_rng();
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
    Err(())
}

pub fn collapse(position: Position, slots: &mut [[Slot; 10]; 10]) -> Result<(), String> {
    if position.x >= 10 || position.y >= 10 {
        return Err("位置不合法".to_string());
    }
    let mut slot_center = slots[usize::from(position.x)][usize::from(position.y)].clone();
    if slot_center.is_collapsed {
        return Ok(());
    }
    // 计算熵
    if position.x > 0 {
        fn cannected(slot_left: Slot, tile_center: &Tile) -> bool {
            if slot_left.is_collapsed {
                if let Some(tile_left) = slot_left.tile {
                    if tile_left.right == tile_center.left {
                        return true;
                    }
                }
            } else {
                for tile_left in slot_left.superposition {
                    if tile_left.right == tile_center.left {
                        return true;
                    }
                }
            }
            false
        }
        slot_center.superposition.retain(|tile_center| {
            cannected(
                slots[usize::from(position.x - 1)][usize::from(position.y)].clone(),
                tile_center,
            )
        });
    }
    if position.x < 9 {
        fn cannected(slot_right: Slot, tile_center: &Tile) -> bool {
            if slot_right.is_collapsed {
                if let Some(tile_right) = slot_right.tile {
                    if tile_right.left == tile_center.right {
                        return true;
                    }
                }
            } else {
                for tile_right in slot_right.superposition {
                    if tile_right.left == tile_center.right {
                        return true;
                    }
                }
            }
            false
        }
        slot_center.superposition.retain(|tile_center| {
            cannected(
                slots[usize::from(position.x + 1)][usize::from(position.y)].clone(),
                tile_center,
            )
        });
    }
    if position.y > 0 {
        fn cannected(slot_down: Slot, tile_center: &Tile) -> bool {
            if slot_down.is_collapsed {
                if let Some(tile_down) = slot_down.tile {
                    if tile_down.top == tile_center.down {
                        return true;
                    }
                }
            } else {
                for tile_down in slot_down.superposition {
                    if tile_down.top == tile_center.down {
                        return true;
                    }
                }
            }
            false
        }
        slot_center.superposition.retain(|tile_center| {
            cannected(
                slots[usize::from(position.x)][usize::from(position.y - 1)].clone(),
                tile_center,
            )
        });
    }
    if position.y < 9 {
        fn cannected(slot_top: Slot, tile_center: &Tile) -> bool {
            if slot_top.is_collapsed {
                if let Some(tile_top) = slot_top.tile {
                    if tile_top.down == tile_center.top {
                        return true;
                    }
                }
            } else {
                for tile_top in slot_top.superposition {
                    if tile_top.down == tile_center.top {
                        return true;
                    }
                }
            }
            false
        }
        slot_center.superposition.retain(|tile_center| {
            cannected(
                slots[usize::from(position.x)][usize::from(position.y + 1)].clone(),
                tile_center,
            )
        });
    }
    slot_center.entropy = slot_center.superposition.len();
    if slot_center.entropy == 0 {
        slot_center.is_collapsed = true;
    } else {
        let _ = random_collapse(&mut slot_center);
    }

    slots[usize::from(position.x)][usize::from(position.y)] = slot_center;
    Ok(())
}

#[test]
fn test() {
    use std::time::Instant;
    let start_time = Instant::now();

    let slots = wave_func_collapse();

    let time = start_time.elapsed().as_secs_f64();
    println!("{:?}", slots);
    println!("{}", time);
}

pub fn wave_func_collapse() -> [[Slot; 10]; 10] {
    use std::time::Instant;
    let start_time = Instant::now();

    let mut rng = rand::thread_rng();
    let mut slots = init();
    let _ = collapse(Position { x: 0, y: 0 }, &mut slots);
    let mut min_entropy: usize = 255;
    let mut min_slots: Vec<Slot> = Vec::new();
    let mut count_collapse: u8 = 99;
    while count_collapse > 0 {
        for i in 0..10 {
            for j in 0..10 {
                if slots[i][j].entropy != 0 && slots[i][j].entropy < min_entropy {
                    min_entropy = slots[i][j].entropy;
                    min_slots.push(slots[i][j].clone());
                }
            }
        }
        min_slots.retain(|slot| slot.entropy == min_entropy);
        if let Some(slot) = min_slots.get(rng.gen_range(0, min_slots.len())) {
            let _ = collapse(slot.position, &mut slots);
        }
        min_slots = Vec::new();
        min_entropy = 255;
        count_collapse -= 1;
    }
    println!("{:?}", slots);
    println!("{}", start_time.elapsed().as_secs_f64());
    slots
}
