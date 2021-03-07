use bevy::math::Vec3;
use rand::Rng;

use crate::engine::plugin::tile_map::{Slot, Tile};

pub fn init(position: Vec3, x: usize, y: usize) -> Vec<Vec<Slot>> {
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

    let mut slots: Vec<Vec<Slot>> = vec![];
    for i in (-(x as i32) / (2 as i32))..=((x as i32) / (2 as i32)) {
        let mut slots_y: Vec<Slot> = vec![];
        for j in (-(y as i32) / (2 as i32))..=((y as i32) / (2 as i32)) {
            let position = Vec3::new(i as f32 + position.x, j as f32 + position.y, position.z);
            slots_y.push(Slot {
                position,
                is_collapsed: false,
                superposition: tiles.clone(),
                entropy: tiles.len() as u64,
                tile: None,
            });
        }
        slots.push(slots_y);
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

pub fn collapse(position: Vec3, slots: &mut Vec<Vec<Slot>>) -> Result<(), String> {
    // 矩阵大小
    let x = slots.len();
    let y = slots[0].len();

    // 矩阵左下角与右上角坐标
    let min_position: Vec3 = slots[0][0].position;
    let max_position: Vec3 = slots[x - 1][y - 1].position;

    // 传入位置是否在矩阵内
    if position.x > max_position.x
        || position.y > max_position.y
        || position.x < min_position.x
        || position.y < min_position.y
    {
        return Err("位置不合法".to_string());
    }

    // 传入位置所在矩阵坐标
    let i: usize = (position.x - min_position.x) as usize;
    let j: usize = (position.y - min_position.y) as usize;
    // 当前位置
    let mut slot_current = slots[i][j].clone();

    // 是否已坍缩
    if slot_current.is_collapsed {
        return Ok(());
    }
    // 计算熵
    if i > 0 {
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
        slot_current
            .superposition
            .retain(|tile_center| cannected(slots[i - 1][j].clone(), tile_center));
    }
    if i < (x - 1) {
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
        slot_current
            .superposition
            .retain(|tile_center| cannected(slots[i + 1][j].clone(), tile_center));
    }
    if j > 0 {
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
        slot_current
            .superposition
            .retain(|tile_center| cannected(slots[i][j - 1].clone(), tile_center));
    }
    if j < (y - 1) {
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
        slot_current
            .superposition
            .retain(|tile_center| cannected(slots[i][j + 1].clone(), tile_center));
    }
    slot_current.entropy = slot_current.superposition.len() as u64;
    if slot_current.entropy == 0 {
        slot_current.is_collapsed = true;
    } else {
        let _ = random_collapse(&mut slot_current);
    }

    slots[i][j] = slot_current;
    Ok(())
}

#[test]
fn test() {
    use std::time::Instant;
    let start_time = Instant::now();

    let slots = wave_func_collapse(Vec3::new(-9.0, -9.0, 0.0), 10, 10);

    let time = start_time.elapsed().as_secs_f64();
    println!("{:?}", slots);
    println!("{}", time);
}

pub fn wave_func_collapse(position: Vec3, add_x: usize, add_y: usize) -> Vec<Vec<Slot>> {
    use std::time::Instant;
    let start_time = Instant::now();

    let mut rng = rand::thread_rng();
    let size_x = add_x * 2;
    let size_y = add_y * 2;
    let mut slots = init(position, size_x, size_y);
    let _ = collapse(position, &mut slots);
    let mut min_entropy: u64 = 18446744073709551615;
    let mut min_slots: Vec<Slot> = Vec::new();
    let mut count_collapse = (size_x + 1) * (size_y + 1);
    while count_collapse > 0 {
        for i in 0..=size_x {
            for j in 0..=size_y {
                if slots[i][j].entropy != 0 && slots[i][j].entropy < min_entropy {
                    min_entropy = slots[i][j].entropy as u64;
                    min_slots.push(slots[i][j].clone());
                }
            }
        }
        min_slots.retain(|slot| slot.entropy == min_entropy);
        if min_slots.len() > 0 {
            if let Some(slot) = min_slots.get(rng.gen_range(0, min_slots.len())) {
                let _ = collapse(slot.position, &mut slots);
            }
        }
        min_slots = Vec::new();
        min_entropy = 18446744073709551615;
        count_collapse -= 1;
    }

    let elapsed = start_time.elapsed().as_secs_f64();
    println!("{:?}", slots);
    println!("{}", elapsed);
    slots
}
