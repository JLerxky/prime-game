use std::error::Error;

use rocksdb::{ColumnFamilyDescriptor, Error as RocksdbError, IteratorMode, Options, DB};
use util::aes::AESUtil;
pub struct RocksDB {
    db: DB,
    aes: AESUtil,
    opts: Options,
    path: String,
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub enum ColumnFamily {
    GameServer,
    GameClient,
}

pub static mut ROCKS_DB: [Option<RocksDB>; 2] = [None, None];

impl RocksDB {
    pub fn open(column_family: ColumnFamily) -> Result<&'static RocksDB, Box<dyn Error>> {
        unsafe {
            let db = &ROCKS_DB[column_family as usize];
            match &db {
                Some(db) => Ok(db),
                None => {
                    let path = "rocks_game_db";
                    let mut cf_opts = Options::default();
                    cf_opts.set_keep_log_file_num(3);
                    let cf =
                        ColumnFamilyDescriptor::new((column_family as u8).to_string(), cf_opts);

                    let mut db_opts = Options::default();
                    db_opts.set_keep_log_file_num(3);
                    db_opts.create_missing_column_families(true);
                    db_opts.create_if_missing(true);
                    // let db = DB::open_for_read_only(&db_opts, path, false)?;
                    let db = DB::open_cf_descriptors(&db_opts, path, vec![cf])?;
                    let aes =
                        AESUtil::config(b"09bn39189y30v47620c334yct285hbp2", b"7v3g41itb236gt9c");
                    ROCKS_DB[column_family as usize] = Some(RocksDB {
                        db,
                        aes,
                        opts: db_opts,
                        path: String::from(path),
                    });
                    if let Some(db) = &ROCKS_DB[column_family as usize] {
                        Ok(db)
                    } else {
                        Err(Box::new(std::io::Error::new(
                            std::io::ErrorKind::Other,
                            "打开数据库失败!",
                        )))
                    }
                }
            }
        }
    }

    // 加密存储
    pub fn put<K, V>(&self, key: K, value: V) -> Result<(), RocksdbError>
    where
        K: AsRef<[u8]>,
        V: AsRef<[u8]>,
    {
        self.db
            .put(key, self.aes.encrypt(value.as_ref()).as_bytes())
    }

    // 无加密存储
    pub fn put_value<K, V>(&self, key: K, value: V) -> Result<(), Box<dyn std::error::Error>>
    where
        K: AsRef<[u8]>,
        V: AsRef<[u8]>,
    {
        match self.db.put(key, value) {
            Ok(_) => Ok(()),
            Err(e) => Err(Box::new(e)),
        }
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

    // 打印所有原值
    pub fn get_all_value(&self) {
        for (key, data) in self.db.full_iterator(IteratorMode::Start).next() {
            match String::from_utf8(key.to_vec()) {
                Ok(key) => match String::from_utf8(data.to_vec()) {
                    Ok(data) => println!("{}: {}", &key, data),
                    Err(_) => {}
                },
                Err(_) => {}
            };
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
    // let rocks_db = RocksDB::open()?;
    // for i in 0..100000 {
    //     rocks_db
    //         .put(format!("key-{}", i), format!("value-{}", i))
    //         .unwrap();
    //     match rocks_db.get_value(format!("key-{}", i)) {
    //         Some(value) => println!("读取 -> {}", value),
    //         None => {}
    //     }
    //     rocks_db.delete(format!("key-{}", i));
    // }
    // let _ = rocks_db.destroy();
}

// 打印所有数据
#[test]
fn test1() {
    // let rocks_db = RocksDB::open();
    // rocks_db.get_all_value();
}
