use anyhow::Result;
use clap::Parser;
use std::fmt;
use std::path::{Path, PathBuf};
use std::str::FromStr;

#[derive(Debug, Parser)]
pub enum TextSubcommand {
    #[command(about = "Sign a text file")]
    Sign(TextSignOpts),
    #[command(about = "Verify a text file")]
    Verify(TextVerifyOpts),
    #[command(about = "Generate a key for text sign")]
    Generate(TextKeyGenerateOpts),
}
#[derive(Debug, Parser)]
pub struct TextSignOpts {
    #[arg(short, long,value_parser=verify_input_file,default_value="-")]
    pub input: String,

    #[arg(short, long,value_parser=verify_input_file)]
    pub key: String,

    #[arg(short, long, default_value = "blake3",value_parser=parse_format)]
    pub format: TextSignFormat,
}
#[derive(Debug, Parser)]
pub struct TextVerifyOpts {
    #[arg(short, long,value_parser=verify_input_file,default_value="-")]
    pub input: String,

    #[arg(short, long,value_parser=verify_input_file)]
    pub key: String,

    #[arg(short, long,value_parser=verify_input_file,default_value="-")]
    pub sign: String,

    #[arg(short, long, default_value = "blake3",value_parser=parse_format)]
    pub format: TextSignFormat,
}
#[derive(Debug, Clone, Copy)]
pub enum TextSignFormat {
    Blake3,
    Ed25519,
}
#[derive(Debug, Parser)]
pub struct TextKeyGenerateOpts {
    #[arg(short, long, default_value = "blake3",value_parser=parse_format)]
    pub format: TextSignFormat,
    #[arg(short, long, value_parser=verify_path)]
    pub output: PathBuf,
}
fn parse_format(format: &str) -> Result<TextSignFormat, anyhow::Error> {
    format.parse::<TextSignFormat>()
}
impl FromStr for TextSignFormat {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "blake3" => Ok(TextSignFormat::Blake3),
            "ed25519" => Ok(TextSignFormat::Ed25519),
            _ => Err(anyhow::anyhow!("Invalid text sign format")),
        }
    }
}
impl From<TextSignFormat> for &'static str {
    fn from(format: TextSignFormat) -> Self {
        match format {
            TextSignFormat::Blake3 => "blake3",
            TextSignFormat::Ed25519 => "ed25519",
        }
    }
}
impl fmt::Display for TextSignFormat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", Into::<&str>::into(*self))
    }
}
fn verify_input_file(input: &str) -> Result<String, anyhow::Error> {
    if input == "-" {
        return Ok(input.to_string());
    }
    if Path::new(input).exists() {
        return Ok(input.to_string());
    }
    Err(anyhow::anyhow!(format!(
        "{} is not a valid file path",
        input
    )))
}
fn verify_path(path: &str) -> Result<PathBuf, &'static str> {
    let p = Path::new(path);
    if p.exists() && p.is_dir() {
        Ok(path.into())
    } else {
        Err("Path does not exist or is a directory")
    }
}
