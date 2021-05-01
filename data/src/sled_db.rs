pub static mut SLED_DB: Option<SledDB> = None;
pub struct SledDB {
    pub db: sled::Db,
    pub path: String,
}

impl SledDB {
    pub fn open(path: &str) -> Result<&'static SledDB, Box<dyn std::error::Error>> {
        unsafe {
            match &SLED_DB {
                Some(sled_db) => Ok(sled_db),
                None => {
                    let db: sled::Db = sled::open(path)?;
                    SLED_DB = Some(SledDB {
                        db,
                        path: path.to_string(),
                    });
                    if let Some(sled_db) = &SLED_DB {
                        Ok(sled_db)
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

    pub fn show_all(path: &str) {
        let db = &SledDB::open(path).unwrap().db;
        for iter in db.iter() {
            match iter {
                Ok((k, v)) => {
                    println!(
                        "{:?}==={:?}",
                        String::from_utf8(k.to_vec()).unwrap(),
                        String::from_utf8(v.to_vec()).unwrap()
                    );
                }
                Err(e) => {
                    println!("{}", e);
                }
            }
        }
    }
}

#[test]
fn test_sled() {
    let db: sled::Db = sled::open("db_sled").unwrap();

    // insert and get
    let _ = db.insert(b"yo!", b"v1");
    assert_eq!(&db.get(b"yo!").unwrap().unwrap(), b"v1");

    // Atomic compare-and-swap.
    let _ = db
        .compare_and_swap(
            b"yo!",      // key
            Some(b"v1"), // old value, None for not present
            Some(b"v2"), // new value, None for delete
        )
        .unwrap();

    // Iterates over key-value pairs, starting at the given key.
    let scan_key: &[u8] = b"a non-present key before yo!";
    let mut iter = db.range(scan_key..);
    assert_eq!(&iter.next().unwrap().unwrap().0, b"yo!");
    assert_eq!(iter.next(), None);

    let _ = db.remove(b"yo!");
    assert_eq!(db.get(b"yo!"), Ok(None));

    let other_tree: sled::Tree = db.open_tree(b"cool db facts").unwrap();
    other_tree
        .insert(
            b"k1",
            &b"a Db acts like a Tree due to implementing Deref<Target = Tree>"[..],
        )
        .unwrap();
}

#[test]
fn test_iter() {
    SledDB::show_all(&format!("../{}", config::DB_PATH_SERVER));
}
