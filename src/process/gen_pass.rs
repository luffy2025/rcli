use anyhow::Result;
use rand::prelude::SliceRandom;

const UPPER: &[u8] = b"ABCDEFGHJKLMNPQRSTUVWXYZ";
const LOWER: &[u8] = b"abcdefghijkmnopqrstuvwxyz";
const NUMBER: &[u8] = b"123456789";
const SYMBOL: &[u8] = b"@#$%^&*_+=.";

pub fn process_genpass(
    length: u8,
    no_upper: bool,
    no_lower: bool,
    no_number: bool,
    no_symbol: bool,
) -> Result<String> {
    let mut pass: Vec<u8> = Vec::with_capacity(length as usize);
    let mut rng = rand::thread_rng();
    let mut chars: Vec<u8> = Vec::new();
    if !no_upper {
        chars.extend(UPPER);
        pass.push(*UPPER.choose(&mut rng).expect("UPPER won't be empty"));
    }
    if !no_lower {
        chars.extend(LOWER);
        pass.push(*LOWER.choose(&mut rng).expect("LOWER won't be empty"));
    }
    if !no_number {
        chars.extend(NUMBER);
        pass.push(*NUMBER.choose(&mut rng).expect("NUMBER won't be empty"));
    }
    if !no_symbol {
        chars.extend(SYMBOL);
        pass.push(*SYMBOL.choose(&mut rng).expect("SYMBOL won't be empty"));
    }

    // 方案1
    // for _ in 0..length {
    //     let c = chars
    //         .choose(&mut rng)
    //         .expect("chars won't be empty in this context");
    //     pass.push(*c as _); // 对 一个引用：&u8 解引用（拿其中的值）时，由于其为u8，所以获取值时会拷贝。

    // 方案2
    if length < 4 {
        return Err(anyhow::anyhow!("Password length must be greater than 4"));
    }

    chars.shuffle(&mut rng);
    let pass_chars = chars.split_at(length as usize - pass.len());
    pass.extend_from_slice(pass_chars.0);
    pass.shuffle(&mut rng);
    let password = String::from_utf8_lossy(&pass);

    Ok(password.to_string())
}
