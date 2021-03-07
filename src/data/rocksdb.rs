use super::super::util::aes::AESUtil;
use rocksdb::{ColumnFamilyDescriptor, Error, Options, DB};

pub struct RocksDB {
    db: DB,
    aes: AESUtil,
    opts: Options,
    path: String,
}

impl RocksDB {
    pub fn open() -> RocksDB {
        let path = "rocks_game_db";
        let mut cf_opts = Options::default();
        cf_opts.set_keep_log_file_num(3);
        let cf = ColumnFamilyDescriptor::new("cf1", cf_opts);

        let mut db_opts = Options::default();
        db_opts.set_keep_log_file_num(3);
        db_opts.create_missing_column_families(true);
        db_opts.create_if_missing(true);
        let db = DB::open_cf_descriptors(&db_opts, path, vec![cf]).unwrap();
        let aes = AESUtil::config(b"09bn39189y30v47620c334yct285hbp2", b"7v3g41itb236gt9c");
        RocksDB {
            db,
            aes,
            opts: db_opts,
            path: String::from(path),
        }
    }

    // 加密存储
    pub fn put<K, V>(&self, key: K, value: V) -> Result<(), Error>
    where
        K: AsRef<[u8]>,
        V: AsRef<[u8]>,
    {
        self.db
            .put(key, self.aes.encrypt(value.as_ref()).as_bytes())
    }

    // 无加密存储
    pub fn put_value<K, V>(&self, key: K, value: V) -> Result<(), Error>
    where
        K: AsRef<[u8]>,
        V: AsRef<[u8]>,
    {
        self.db.put(key, value)
    }

    // 读取加密值
    pub fn get<K: AsRef<[u8]>>(&self, key: K) -> Option<String> {
        match self.db.get(key) {
            Ok(Some(ciphertext)) => self.aes.decrypt(ciphertext.as_slice()),
            Ok(None) => None,
            Err(e) => {
                println!("RocksDB: 读取数据失败 -> {}", e);
                None
            }
        }
    }

    // 读取原值
    pub fn get_value<K: AsRef<[u8]>>(&self, key: K) -> Option<String> {
        match self.db.get(key) {
            Ok(Some(ciphertext)) => match String::from_utf8(ciphertext) {
                Ok(data) => Some(data),
                Err(_) => None,
            },
            Ok(None) => None,
            Err(e) => {
                println!("RocksDB: 读取数据失败 -> {}", e);
                None
            }
        }
    }

    pub fn delete<K: AsRef<[u8]>>(&self, key: K) {
        match self.db.delete(key) {
            Ok(_) => {}
            Err(e) => println!("RocksDB: 清空库文件失败 -> {}", e),
        }
    }

    pub fn destroy(&self) {
        match DB::destroy(&self.opts, self.path.clone()) {
            Ok(_) => {}
            Err(e) => println!("RocksDB: 清空库文件失败 -> {}", e),
        }
    }
}

#[test]
fn test() {
    let rocks_db = RocksDB::open();
    for i in 0..100000 {
        rocks_db
            .put(format!("key-{}", i), format!("value-{}", i))
            .unwrap();
        match rocks_db.get_value(format!("key-{}", i)) {
            Some(value) => println!("读取 -> {}", value),
            None => {}
        }
        rocks_db.delete(format!("key-{}", i));
    }
    // let _ = rocks_db.destroy();
}
