use anyhow::Result;
use rand::prelude::SliceRandom;
use rand::Rng;
use zxcvbn::zxcvbn;
const UPPER: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ";
const LOWER: &[u8] = b"abcdefghijklmnopqrstuvwxyz";
const NUMBERS: &[u8] = b"0123456789";
const SYMBOLS: &[u8] = b"!#$%&.@";
pub fn process_genpass(
    upper: bool,
    lower: bool,
    numbers: bool,
    symbols: bool,
    length: u8,
) -> Result<String, anyhow::Error> {
    let mut chars = Vec::new();
    let mut rng = rand::thread_rng();
    let mut password = Vec::new();
    if upper {
        chars.extend(UPPER.iter().collect::<Vec<_>>());
        password.push(UPPER[rng.gen_range(0..UPPER.len())]);
    }
    if lower {
        chars.extend(LOWER.iter().collect::<Vec<_>>());
        password.push(LOWER[rng.gen_range(0..LOWER.len())]);
    }
    if numbers {
        chars.extend(NUMBERS.iter().collect::<Vec<_>>());
        password.push(NUMBERS[rng.gen_range(0..NUMBERS.len())]);
    }
    if symbols {
        chars.extend(SYMBOLS.iter().collect::<Vec<_>>());
        password.push(SYMBOLS[rng.gen_range(0..SYMBOLS.len())]);
    }

    for _ in 0..(length - password.len() as u8) {
        password.push(chars[rng.gen_range(0..chars.len())]);
    }
    password.shuffle(&mut rng);
    let password = String::from_utf8(password)?;
    let estimate = zxcvbn(&password, &[]);
    eprintln!("Password strength:{}", estimate.score());
    Ok(password)
}
