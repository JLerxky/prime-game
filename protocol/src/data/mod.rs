pub mod update_data;

// 包数据[2..]
pub trait Data {
    fn data(&self) -> Vec<u8>;
}
