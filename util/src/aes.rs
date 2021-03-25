pub struct AESUtil {
    vi: [u8; 16],
    cipher: libaes::Cipher,
}

impl AESUtil {
    pub fn init() -> AESUtil {
        AESUtil {
            vi: *b"i7tyo9a2b93g83y2",
            cipher: libaes::Cipher::new_256(b"74t1vn02yu4-9[u1a0x,=35y-2vbn43y"),
        }
    }
    pub fn config(key: &[u8; 32], vi: &[u8; 16]) -> AESUtil {
        AESUtil {
            vi: *vi,
            cipher: libaes::Cipher::new_256(key),
        }
    }
    pub fn encrypt(&self, data: &[u8]) -> String {
        base64::encode(self.cipher.cbc_encrypt(&self.vi, data))
    }
    pub fn decrypt(&self, data: &[u8]) -> Option<String> {
        match base64::decode(data) {
            Ok(data) => match String::from_utf8(self.cipher.cbc_decrypt(&self.vi, &data[..])) {
                Ok(s) => Some(s),
                Err(_) => None,
            },
            Err(_) => None,
        }
    }
}

#[test]
fn aes() {
    let aes_util = AESUtil::init();

    let plaintext = b"f0:18:98:e9:3b:82,f0:18:98:e9:3b:83";
    // 加密
    let encrypted = aes_util.encrypt(plaintext);
    println!("{}", &encrypted);

    let aes_util = AESUtil::config(b"dsvbj2hbvkjhb2k3vb2ui3v23v23v23v", b"1234567890123456");

    // 解密
    let decrypted = aes_util.decrypt(&encrypted.as_bytes());
    println!("{:?}", &decrypted);
}
