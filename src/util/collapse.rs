use std::collections::HashMap;

use bevy::math::Vec3;
use rand::Rng;

use crate::{
    data::rocksdb::RocksDB,
    engine::plugin::tile_map::{get_tiles, Slot, Tile},
};

pub fn init(position: Vec3, size: Vec3, step: f32) -> HashMap<String, Slot> {
    let mut slots: HashMap<String, Slot> = HashMap::default();

    let rocks_db = RocksDB::open();

    for x in (-(size.x as i32) / (2 as i32)) + 1..=((size.x as i32) / (2 as i32)) + 1 {
        let position_x = x as f32 * step;
        for y in (-(size.y as i32) / (2 as i32))..=((size.y as i32) / (2 as i32)) {
            let position_y = y as f32 * step;
            for z in (-(size.z as i32) / (2 as i32))..=((size.z as i32) / (2 as i32)) {
                let position_z = z as f32 * step;
                let position = Vec3::new(
                    position_x + position.x,
                    position_y + position.y,
                    position_z + position.z,
                );

                let position_key = format!(
                    "{:?},{:?},{:?}",
                    position.x as i32, position.y as i32, position.z as i32
                );

                match rocks_db.get_value(&position_key) {
                    Some(value) => {
                        let _ = slots.insert(position_key, serde_json::from_str(&value).unwrap());
                    }
                    None => {
                        let _ = slots.insert(position_key, Slot::new(position));
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
        slot.superposition = get_tiles();
        slot.entropy = 0;
        slot.is_collapsed = true;
        return Ok(());
    };
    Err(())
}

pub fn collapse(
    position: Vec3,
    size: Vec3,
    step: f32,
    slots: &mut HashMap<String, Slot>,
) -> Result<(), String> {
    // 矩阵大小
    let x = size.x as usize * 2 + 1;
    let y = size.y as usize * 2 + 1;
    let z = size.z as usize * 2 + 1;

    // 矩阵左下角与右上角坐标
    let min_position: Vec3 = Vec3::new((-size.x / 2f32), (-size.y / 2f32), (-size.z / 2f32));
    let max_position: Vec3 = Vec3::new((size.x / 2f32), (size.y / 2f32), (size.z / 2f32));

    // 传入位置是否在矩阵内
    if position.x > max_position.x
        || position.y > max_position.y
        || position.x < min_position.x
        || position.y < min_position.y
    {
        return Err("位置不合法".to_string());
    }

    // 传入位置所在矩阵坐标
    let i: i32 = (position.x - min_position.x) as i32;
    let j: i32 = (position.y - min_position.y) as i32;
    let k: i32 = (position.z - min_position.z) as i32;
    // 当前位置
    let slots_clone = slots.clone();
    let mut slot_current = slots
        .get_mut(&format!(
            "{:?},{:?},{:?}",
            position.x as i32, position.y as i32, position.z as i32
        ))
        .unwrap();

    // 是否已坍缩
    if slot_current.is_collapsed {
        return Ok(());
    }
    // 计算熵
    // 四周
    let mut slot_top = *slots_clone
        .get(&format!(
            "{:?},{:?},{:?}",
            position.x as i32,
            position.y as i32 + size.y as i32,
            position.z as i32
        ))
        .unwrap_or(&Slot::new(Vec3::new(
            position.x,
            position.y + size.y,
            position.z,
        )));
    let mut slot_down = *slots_clone
        .get(&format!(
            "{:?},{:?},{:?}",
            position.x as i32,
            position.y as i32 - size.y as i32,
            position.z as i32
        ))
        .unwrap_or(&Slot::new(Vec3::new(
            position.x,
            position.y - size.y,
            position.z,
        )));
    let mut slot_left = *slots_clone
        .get(&format!(
            "{:?},{:?},{:?}",
            position.x as i32 - size.x as i32,
            position.y as i32,
            position.z as i32
        ))
        .unwrap_or(&Slot::new(Vec3::new(
            position.x - size.x,
            position.y,
            position.z,
        )));
    let mut slot_right = *slots_clone
        .get(&format!(
            "{:?},{:?},{:?}",
            position.x as i32 + size.x as i32,
            position.y as i32,
            position.z as i32
        ))
        .unwrap_or(&Slot::new(Vec3::new(
            position.x + size.x,
            position.y,
            position.z,
        )));

    let mut entropy = slot_current.superposition.len();

    for i in 0..slot_current.superposition.len() {
        if let Some(tile_current) = slot_current.superposition[i] {
            // 左连接
            let mut flag_left = false;
            for left in 0..slot_left.superposition.len() {
                if let Some(tile_left) = slot_left.superposition[left] {
                    if tile_left.right == tile_current.left {
                        flag_left = true;
                        break;
                    }
                }
            }
            if flag_left {
                break;
            }

            // 右连接
            let mut flag_right = false;
            for right in 0..slot_right.superposition.len() {
                if let Some(tile) = slot_right.superposition[right] {
                    if tile.left == tile_current.right {
                        flag_right = true;
                        break;
                    }
                }
            }
            if flag_right {
                break;
            }

            // 上连接
            let mut flag_top = false;
            for top in 0..slot_top.superposition.len() {
                if let Some(tile) = slot_top.superposition[top] {
                    if tile.down == tile_current.top {
                        flag_top = true;
                        break;
                    }
                }
            }
            if flag_top {
                break;
            }

            // 下连接
            let mut flag_down = false;
            for down in 0..slot_down.superposition.len() {
                if let Some(tile) = slot_down.superposition[down] {
                    if tile.top == tile_current.down {
                        flag_down = true;
                        break;
                    }
                }
            }
            if flag_down {
                break;
            }

            slot_current.superposition[i] = None;
        } else {
            entropy -= 1;
        }
    }

    slot_current.entropy = slot_current.superposition.len() as usize;
    if slot_current.entropy == 0 {
        slot_current.is_collapsed = true;
    } else {
        let _ = random_collapse(&mut slot_current);
    }

    Ok(())
}

pub fn wave_func_collapse(position: Vec3, add_x: usize, add_y: usize) -> HashMap<String, Slot> {
    // use std::time::Instant;
    // let start_time = Instant::now();

    let mut rng = rand::thread_rng();
    let size = Vec3::new((add_x * 2) as f32, (add_y * 2) as f32, 0f32);
    let mut slots = init(position, size, 50f32);
    let _ = collapse(position, size, 50f32, &mut slots);
    let mut min_entropy: usize = 999999999;
    let mut min_slots: Vec<Slot> = Vec::new();
    let mut count_collapse = (size.x as usize + 1) * (size.y as usize + 1);
    while count_collapse > 0 {
        for slot in slots.values() {
            if slot.entropy != 0 && slot.entropy < min_entropy {
                min_entropy = slot.entropy;
                min_slots.push(slot.clone());
            }
        }
        min_slots.retain(|slot| slot.entropy == min_entropy);
        if min_slots.len() > 0 {
            if let Some(slot) = min_slots.get(rng.gen_range(0, min_slots.len())) {
                let _ = collapse(slot.position, size, 50f32, &mut slots);
            }
        }
        min_slots = Vec::new();
        min_entropy = 999999999;
        count_collapse -= 1;
    }

    // let elapsed = start_time.elapsed().as_secs_f64();
    // println!("{:?}", slots);
    // println!("{}", elapsed);
    slots
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
