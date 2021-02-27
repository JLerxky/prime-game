use rand::rngs::OsRng;
use rsa::{PaddingScheme, PublicKey, RSAPrivateKey, RSAPublicKey};

pub struct RSAUtil {
    private_key: RSAPrivateKey,
    public_key: RSAPublicKey,
}

impl RSAUtil {
    pub fn init() -> RSAUtil {
        let private_key_content = r#"
-----BEGIN RSA PRIVATE KEY-----
MIIEowIBAAKCAQEA1Bak146r1XMR3vUfv6EqHBnvLiuV+v8MdBa64HTZQckqPj/c
Mi10z2xs54KwaClmU+vQWuiJhcYqVaE6Gee45mzGxIBrQu5RXQKMZh82SD4CPUaD
I1hxsfXU2lSBd38dLeZ+57+I7iClZUZ2+g9EvZPRyvgle2SiD5Zqqkx29UxRUqEC
4b38HR/BtcqQl54CP+cPwezecUitWUi+yz4xtp5XaYvPIOMYrYc0OdkyT6BaNxfK
wRdPitcFDyNVoYWsByCAPZejqwlHVJCwQogtLJyxTuCQzmDl2qsmbX4QizZOpi6D
FeKxesIAXrJhOHU8yJRp3oZdgZzio4OPeH+z3QIDAQABAoIBAGlbRk/+g8Xo/7uu
MTNUsEJ9b5+T71FcWadhkzvlT/eqIt4BgzU594kC56ap+VAFINwXjM1cLtTReD5J
dT9hKSzuZF9BHAeHs8tsdKpWXvTMOPiPip2IVhJ9eFttF1NxReGDWw9symzgssj4
lCnsY815HQKi+puthgQQSLSIf5+iff3ka7asby/WREDWK2YOP+9vmUiiGI1D/EAN
0VjYPLpWYlOyWOmCIvSARTBZrnZnKmyaDP1E3kIbShJ/0CPQnVKErUVuTyyQrEgB
V6dG9tDFjYSeIp7aOKO+VPBELxjEVoBI6xiU+dHcz0etFoLWQXMKc0JpW99dU14G
bVWBWdECgYEA9cjkdqzerkRrpvdYlyVpMV7mmbLUlIXNL3VvPkKLIDNzuzZ5BfOf
A8WaHlxP3AIyqX7D3Xb7SMNgMqV0TTYkIbeu3aPQLR2j7qlnVZx+ZcYfsfjINQZu
0QG0JHOLy/Ainc0krgpCfrnLrG8K6UzHExebsMNk2KQPvkgF6EtQm8cCgYEA3Oc6
kbfP4JMuDIi0HocsPrOGEejCCLC5GOR+buYqtIP9wJMolA2RfC2yMPEkEPJjpm/C
ybiPPo3kX3/H3EL9Ibv5Y92ounAVGhIgpOfJcKdXrPFKF60sDeF/MiyT4Y0dsuMr
xoCJsKvWFD0vJEFCXpX8Ok9m2yL9wWjoSRFzyzsCgYA0mDCLVKKyF+IRcIx8i3zf
G7es3B+xSJCHv1F2uTacRWZWgWmUZHV6kSZRqN9N7Qp6Sq4PDg7nmydSryUcAKHq
tSuuMgeyH82cnJOINly12pedtebhsea0wvCRfEhF87T0n5lhxMBF7ewvkfN7yPow
k7S1Npq14SWdSjw5YSR2sQKBgCggtOn9ivyPn+DVAGs/QNFPCT69mmm7uJQGBdWd
aAqMo47U7J0gbdox7tmnJFcTwPhd+vNr3FGPsvmKG8MuRA9Zty7l+B2N+LdYradn
F5eoPmwDFOOCKKuI1/NEV8S0Pr3dOHmBpgF65ZOdna+nTyghdMUe4V9TVudIaFHF
otQ7AoGBALZLbRFriOVrXW1yJa7hXa0GVbspzb5f1dm0k3lCvSo0nJwnfsP1gBNT
zaUpW/u98iDVpxalPcKGYMNFdoa6/KONVKdXEbJITPuN7D0ztLNFJ2/qtxoMYNNj
oWL75n60/RC6YRVMYI5OZNLzrRYmPz22oYAOcB6oOkP077RhSqD1
-----END RSA PRIVATE KEY-----
"#;

        let private_encoded = private_key_content
            .lines()
            .filter(|line| !line.starts_with("-"))
            .fold(String::new(), |mut data, line| {
                data.push_str(&line);
                data
            });
        let private_bytes = base64::decode(&private_encoded).expect("base64解码失败");
        let private_key = RSAPrivateKey::from_pkcs1(&private_bytes).expect("解析key失败");

        let public_key = RSAPublicKey::from(&private_key);
        RSAUtil {
            private_key,
            public_key,
        }
    }
    pub fn encrypt(&mut self, data: &[u8]) -> String {
        let mut rng = OsRng;
        let padding = PaddingScheme::new_oaep::<sha2::Sha256>();
        base64::encode(self.public_key.encrypt(&mut rng, padding, data).unwrap())
    }
    pub fn decrypt(&mut self, data: &[u8]) -> String {
        let padding = PaddingScheme::new_oaep::<sha2::Sha256>();
        String::from_utf8(
            self.private_key
                .decrypt(padding, &base64::decode(data).unwrap()[..])
                .unwrap(),
        )
        .unwrap()
    }
}

#[test]
fn rsa() {
    let mut rsa_util = RSAUtil::init();

    let plaintext = b"127.0.0.1,192.168.0.1";
    // 加密
    let encrypted = rsa_util.encrypt(plaintext);
    println!("{}", &encrypted);

    // 解密
    let decrypted = rsa_util.decrypt(&encrypted.as_bytes());
    println!("{}", &decrypted);
}