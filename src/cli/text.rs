use anyhow::Result;
use clap::Parser;
use std::fmt;
use std::path::{Path, PathBuf};
use std::str::FromStr;
use tokio::fs;

use crate::{process_text_generate, process_text_sign, process_text_verify, CmdExcutor};

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
impl CmdExcutor for TextSignOpts {
    async fn excutor(self) -> anyhow::Result<()> {
        let signed = process_text_sign(&self.input, &self.key, self.format)?;
        println!("{}", signed);
        Ok(())
    }
}
impl CmdExcutor for TextVerifyOpts {
    async fn excutor(self) -> anyhow::Result<()> {
        match process_text_verify(&self.input, &self.key, &self.sign, self.format)? {
            true => println!("✓ Signature verified"),
            false => println!("✗ Signature verification failed"),
        }
        Ok(())
    }
}
impl CmdExcutor for TextKeyGenerateOpts {
    async fn excutor(self) -> anyhow::Result<()> {
        let keys = process_text_generate(self.format)?;
        for (k, v) in keys {
            fs::write(self.output.join(k), v).await?;
        }
        Ok(())
    }
}
impl CmdExcutor for TextSubcommand {
    async fn excutor(self) -> anyhow::Result<()> {
        match self {
            TextSubcommand::Sign(opts) => opts.excutor().await,
            TextSubcommand::Verify(opts) => opts.excutor().await,
            TextSubcommand::Generate(opts) => opts.excutor().await,
        }
    }
}
