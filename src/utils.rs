use anyhow::Result;
use std::io::Read;
use std::{fs::File, io::stdin};
pub fn get_reader(input: &str) -> Result<Box<dyn Read>> {
    if input == "-" {
        Ok(Box::new(stdin()))
    } else {
        Ok(Box::new(File::open(input)?))
    }
}
