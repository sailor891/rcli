use anyhow::Result;
use clap::Parser;
use std::path::Path;
use std::str::FromStr;

#[derive(Debug, Parser)]
pub enum Base64Subcommand {
    #[command(name = "encode", about = "Base64 Encode")]
    Encode(Base64EncodeOpts),
    #[command(name = "decode", about = "Base64 Decode")]
    Decode(Base64DecodeOpts),
}
#[derive(Debug, Parser)]
pub struct Base64EncodeOpts {
    #[arg(short, long,value_parser=verify_input_file,default_value="-")]
    pub input: String,

    #[arg(short, long, default_value = "standard")]
    pub format: Base64Format,
}
#[derive(Debug, Clone, Copy)]
pub enum Base64Format {
    Standard,
    UrlSafe,
}
impl FromStr for Base64Format {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "standard" => Ok(Base64Format::Standard),
            "urlsafe" => Ok(Base64Format::UrlSafe),
            _ => Err(anyhow::anyhow!("Invalid base64 format")),
        }
    }
}
#[derive(Debug, Parser)]
pub struct Base64DecodeOpts {
    #[arg(short, long,value_parser=verify_input_file,default_value="-")]
    pub input: String,

    #[arg(short, long, default_value = "standard")]
    pub format: Base64Format,
}
fn verify_input_file(input: &str) -> Result<String, anyhow::Error> {
    if input == "-" {
        return Ok(input.to_string());
    }
    if Path::new(input).exists() {
        return Ok(input.to_string());
    }
    Err(anyhow::anyhow!("Invalid input file"))
}
