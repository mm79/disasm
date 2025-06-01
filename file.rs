use std::fs;
use std::io;

pub fn read(path: &str) -> Result<Vec<u8>, io::Error> {
    let data = fs::read(path)?;

    Ok(data)
}

pub fn is_text(data: &Vec<u8>) -> bool {
    data.iter()
        .all(|&b| (b == 9 || b == 10 || b == 13) || (b >= 32 && b <= 126))
}
