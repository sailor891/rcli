use anyhow::Result;
use std::io::Read;
use std::path::{Path, PathBuf};
use std::{fs::File, io::stdin};
pub fn get_reader(input: &str) -> Result<Box<dyn Read>> {
    if input == "-" {
        Ok(Box::new(stdin()))
    } else {
        Ok(Box::new(File::open(input)?))
    }
}
pub fn verify_path(path: &str) -> Result<PathBuf, &'static str> {
    let p = Path::new(path);
    if p.exists() && p.is_dir() {
        Ok(path.into())
    } else {
        Err("Path does not exist or is a directory")
    }
}
pub fn get_reader_content(reader: &mut dyn Read) -> Result<Vec<u8>> {
    let mut buf = String::new();
    reader.read_to_string(&mut buf)?;
    let buf = buf.trim();
    Ok(buf.as_bytes().to_vec())
}
pub fn verify_input_file(input: &str) -> Result<String, anyhow::Error> {
    if input == "-" {
        return Ok(input.to_string());
    }
    if Path::new(input).exists() {
        return Ok(input.to_string());
    }
    Err(anyhow::anyhow!("Invalid input file"))
}
