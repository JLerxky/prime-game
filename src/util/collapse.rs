use std::collections::HashMap;

use bevy::math::Vec3;
use rand::Rng;

use crate::{
    data::rocksdb::RocksDB,
    engine::plugin::tile_map::{get_none_tiles, get_tiles, Slot},
};

pub fn vec3_to_key(pos: Vec3) -> String {
    format!("{},{},{}", pos.x as i32, pos.y as i32, pos.z as i32)
}

pub fn i32_to_key(x: i32, y: i32, z: i32) -> String {
    format!("{},{},{}", x, y, z)
}

pub fn init(position: Vec3, size: Vec3, step: Vec3) -> HashMap<String, Slot> {
    let mut slots: HashMap<String, Slot> = HashMap::default();

    let rocks_db = RocksDB::open();

    for x in (-(size.x as i32) / (2 as i32)) + 1..=((size.x as i32) / (2 as i32)) + 1 {
        let position_x = x as f32 * step.x;
        for y in (-(size.y as i32) / (2 as i32))..=((size.y as i32) / (2 as i32)) {
            let position_y = y as f32 * step.y;
            for z in (-(size.z as i32) / (2 as i32))..=((size.z as i32) / (2 as i32)) {
                let position_z = z as f32 * step.z;
                let tile_position = Vec3::new(position_x, position_y, position_z) + position;

                let position_key = vec3_to_key(tile_position);

                match rocks_db.get_value(&position_key) {
                    Some(value) => {
                        let _ = slots.insert(position_key, serde_json::from_str(&value).unwrap());
                    }
                    None => {
                        let _ = slots.insert(position_key, Slot::new(tile_position));
                    }
                }
            }
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
        slot.tile = *tile;
        slot.superposition = get_none_tiles();
        slot.entropy = 0;
        slot.is_collapsed = true;
        return Ok(());
    };
    Err(())
}

pub fn collapse(
    position: Vec3,
    size: Vec3,
    step: Vec3,
    slots: &mut HashMap<String, Slot>,
) -> Result<(), String> {
    // // 矩阵大小
    // let x = size.x as usize * 2 + 1;
    // let y = size.y as usize * 2 + 1;
    // let z = size.z as usize * 2 + 1;

    // // 矩阵左下角与右上角坐标
    // let min_position: Vec3 = Vec3::new((-size.x / 2f32), (-size.y / 2f32), (-size.z / 2f32));
    // let max_position: Vec3 = Vec3::new((size.x / 2f32), (size.y / 2f32), (size.z / 2f32));

    // // 传入位置是否在矩阵内
    // if position.x > max_position.x
    //     || position.y > max_position.y
    //     || position.x < min_position.x
    //     || position.y < min_position.y
    // {
    //     return Err("位置不合法".to_string());
    // }

    // // 传入位置所在矩阵坐标
    // let i: i32 = (position.x - min_position.x) as i32;
    // let j: i32 = (position.y - min_position.y) as i32;
    // let k: i32 = (position.z - min_position.z) as i32;
    // 当前位置
    let slots_clone = slots.clone();
    let mut slot_current = slots.get_mut(&vec3_to_key(position)).unwrap();

    // 是否已坍缩
    if slot_current.is_collapsed {
        return Ok(());
    }
    // 计算熵
    build_entropy(slots_clone, position, step, &mut slot_current);
    if slot_current.entropy <= 0 {
        slot_current.entropy = 0;
        slot_current.is_collapsed = true;
    } else {
        let _ = random_collapse(&mut slot_current);
        let slots_clone1 = slots.clone();
        let slots_clone2 = slots.clone();
        let slots_clone3 = slots.clone();
        let slots_clone4 = slots.clone();

        let pos_top = Vec3::new(position.x, position.y + step.y, position.z);
        if let Some(slot_top) = slots.get_mut(&vec3_to_key(pos_top)) {
            build_entropy(slots_clone1, pos_top, step, slot_top);
        }

        let pos_down = Vec3::new(position.x, position.y - step.y, position.z);
        if let Some(slot_down) = slots.get_mut(&vec3_to_key(pos_down)) {
            build_entropy(slots_clone2, pos_down, step, slot_down);
        }

        let pos_left = Vec3::new(position.x - step.x, position.y, position.z);
        if let Some(slot_left) = slots.get_mut(&vec3_to_key(pos_left)) {
            build_entropy(slots_clone3, pos_left, step, slot_left);
        }

        let pos_right = Vec3::new(position.x + step.x, position.y, position.z);
        if let Some(slot_right) = slots.get_mut(&vec3_to_key(pos_right)) {
            build_entropy(slots_clone4, pos_right, step, slot_right);
        }
    }

    Ok(())
}

fn build_entropy(
    slots_clone: HashMap<String, Slot>,
    position: Vec3,
    step: Vec3,
    slot_current: &mut Slot,
) {
    if slot_current.is_collapsed || slot_current.entropy <= 0 {
        return;
    }
    let slot_top = *slots_clone
        .get(&i32_to_key(
            position.x as i32,
            position.y as i32 + step.y as i32,
            position.z as i32,
        ))
        .unwrap_or(&Slot::new(Vec3::new(
            position.x,
            position.y + step.y,
            position.z,
        )));
    let slot_down = *slots_clone
        .get(&i32_to_key(
            position.x as i32,
            position.y as i32 - step.y as i32,
            position.z as i32,
        ))
        .unwrap_or(&Slot::new(Vec3::new(
            position.x,
            position.y - step.y,
            position.z,
        )));
    let slot_left = *slots_clone
        .get(&i32_to_key(
            position.x as i32 - step.x as i32,
            position.y as i32,
            position.z as i32,
        ))
        .unwrap_or(&Slot::new(Vec3::new(
            position.x - step.x,
            position.y,
            position.z,
        )));
    let slot_right = *slots_clone
        .get(&i32_to_key(
            position.x as i32 + step.x as i32,
            position.y as i32,
            position.z as i32,
        ))
        .unwrap_or(&Slot::new(Vec3::new(
            position.x + step.x,
            position.y,
            position.z,
        )));
    let mut entropy = slot_current.superposition.len();
    for i in 0..slot_current.superposition.len() {
        if let Some(tile_current) = slot_current.superposition[i] {
            // 左连接
            let mut flag_left = false;
            if slot_left.is_collapsed {
                if let Some(tile_left) = slot_left.tile {
                    if tile_left.right == tile_current.left {
                        flag_left = true;
                    }
                }
            } else {
                for left in 0..slot_left.superposition.len() {
                    if let Some(tile_left) = slot_left.superposition[left] {
                        if tile_left.right == tile_current.left {
                            flag_left = true;
                            break;
                        }
                    }
                }
            }
            if flag_left {
                continue;
            }

            // 右连接
            let mut flag_right = false;
            if slot_right.is_collapsed {
                if let Some(tile_right) = slot_right.tile {
                    if tile_right.left == tile_current.right {
                        flag_right = true;
                    }
                }
            } else {
                for right in 0..slot_right.superposition.len() {
                    if let Some(tile_right) = slot_right.superposition[right] {
                        if tile_right.left == tile_current.right {
                            flag_right = true;
                            break;
                        }
                    }
                }
            }
            if flag_right {
                continue;
            }

            // 上连接
            let mut flag_top = false;
            if slot_top.is_collapsed {
                if let Some(tile_top) = slot_top.tile {
                    if tile_top.down == tile_current.top {
                        flag_top = true;
                    }
                }
            } else {
                for top in 0..slot_top.superposition.len() {
                    if let Some(tile_top) = slot_top.superposition[top] {
                        if tile_top.down == tile_current.top {
                            flag_top = true;
                            break;
                        }
                    }
                }
            }
            if flag_top {
                continue;
            }

            // 下连接
            let mut flag_down = false;
            if slot_down.is_collapsed {
                if let Some(tile_down) = slot_down.tile {
                    if tile_down.top == tile_current.down {
                        flag_down = true;
                    }
                }
            } else {
                for down in 0..slot_down.superposition.len() {
                    if let Some(tile) = slot_down.superposition[down] {
                        if tile.top == tile_current.down {
                            flag_down = true;
                            break;
                        }
                    }
                }
            }
            if flag_down {
                continue;
            }

            slot_current.superposition[i] = None;
        }
        entropy -= 1;
    }
    slot_current.entropy = entropy;
}

#[test]
fn test_1() {
    let mut map = HashMap::new();
    map.insert("a", 1);
    map.insert("b", 2);
    map.insert("c", 3);

    for val in map.values() {
        println!("{}", val);
    }
}

pub fn wave_func_collapse(position: Vec3, mut size: Vec3, step: Vec3) -> HashMap<String, Slot> {
    // use std::time::Instant;
    // let start_time = Instant::now();

    let mut rng = rand::thread_rng();
    size *= 2f32;
    let mut slots = init(position, size, step);
    let _ = collapse(position, size, step, &mut slots);
    let mut min_entropy: usize = 999999999;
    let mut min_slots: Vec<Slot> = Vec::new();
    let mut count_collapse = (size.x as usize + 1) * (size.y as usize + 1);
    while count_collapse > 0 {
        println!("{:?}", slots.get(&vec3_to_key(position)));
        for slot in slots.values() {
            if slot.entropy != 0 && slot.entropy < min_entropy {
                min_entropy = slot.entropy;
                min_slots.push(slot.clone());
            }
        }
        min_slots.retain(|slot| slot.entropy == min_entropy);
        if min_slots.len() > 0 {
            if let Some(slot) = min_slots.get(rng.gen_range(0, min_slots.len())) {
                let _ = collapse(slot.position, size, step, &mut slots);
            }
        }
        min_slots = Vec::new();
        min_entropy = 999999999;
        count_collapse -= 1;
    }

    // let elapsed = start_time.elapsed().as_secs_f64();
    println!("{:?}", slots);
    // println!("{}", elapsed);
    slots
}

#[test]
fn test() {
    use std::time::Instant;
    let start_time = Instant::now();

    let slots = wave_func_collapse(
        Vec3::new(-9.0, -9.0, 0.0),
        Vec3::new(10f32, 10f32, 0f32),
        Vec3::new(50f32, 50f32, 50f32),
    );

    let time = start_time.elapsed().as_secs_f64();
    println!("{:?}", slots);
    println!("{}", time);
}
