use anyhow::{bail, Error, Result};
use base64::{Engine as _, engine::general_purpose};
use hash::merhash::mersenne_hash;


/// 密码子 （长度100）， 可随意交换次序和增减字符，以实现个性化定制
const CRYPTO: &str = "!QWSA$FTIU*)FDGHQ$%sdgfjhg7s67$%&*(sdjhguidsghgbnzbhvzghdjkfjanbyu8<w12>09?:{23567};,.[]4_+-=ZXCGIQ";

pub fn generate_password(seed: &str, length: usize)
    -> Result<String, Error>
{
    if length < 6 {
        bail!("length must >= 6");
    }

    let p = match length {
        6..=10 => 1,
        11..=15 => 2,
        _ => 3,
    };

    let mut mer_hash = mersenne_hash(seed).pow(p);
    let mut passwd = String::new();
    let crypto_len = CRYPTO.len();
    while mer_hash > 9 {
        let loc = mer_hash % crypto_len;
        let nthc = CRYPTO.chars()
            .nth(loc)
            .expect("Error while getting char!");
        passwd.push(nthc);
        mer_hash /= crypto_len;
    }

    let interval = passwd.clone();
    for c in seed.chars() {
        passwd.push(c);
        passwd += &interval;
    }

    passwd = general_purpose::STANDARD.encode(passwd);
    passwd = passwd.replace(['+', '/'], "*");

    let interval = passwd.clone();
    while passwd.len() < length {
        passwd += &interval;
    }

    Ok(format!("{seed}: {passwd}"))
}

