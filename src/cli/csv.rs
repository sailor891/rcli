use clap::Parser;
use std::fmt;
use std::path::Path;
use std::str::FromStr;

use crate::{process_csv, CmdExcutor};

#[derive(Debug, Clone, Copy)]
pub enum OutputFormat {
    Json,
    Yaml,
}
#[derive(Debug, Parser)]
pub struct CsvOpts {
    #[arg(short,long,value_parser=verify_input_file)]
    pub input: String,

    #[arg(short, long)]
    pub output: Option<String>,

    #[arg(long, value_parser=parse_format, default_value = "json")]
    pub format: OutputFormat,

    #[arg(short, long, default_value_t = ',')]
    pub delimiter: char,

    #[arg(long, default_value_t = true)]
    pub header: bool,
}
fn verify_input_file(filename: &str) -> Result<String, &'static str> {
    if filename.is_empty() {
        return Err("Input file is empty");
    }
    if !filename.ends_with(".csv") {
        return Err("Input file must be a CSV file");
    }
    if !Path::new(filename).exists() {
        return Err("Input file does not exist");
    }
    Ok(filename.into())
}
// 从&str转换为OutputFormat，实现将&str转为OutputFormat的trait（FromStr trait的from_str方法）
fn parse_format(format: &str) -> Result<OutputFormat, anyhow::Error> {
    format.parse::<OutputFormat>()
}
// 从OutputFormat转换为&str
impl From<OutputFormat> for &'static str {
    fn from(format: OutputFormat) -> Self {
        match format {
            OutputFormat::Json => "json",
            OutputFormat::Yaml => "yaml",
        }
    }
}
// 从&str转换为OutputFormat
impl FromStr for OutputFormat {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "json" => Ok(OutputFormat::Json),
            "yaml" => Ok(OutputFormat::Yaml),
            _ => Err(anyhow::anyhow!("Invalid output format")),
        }
    }
}
// 实现Display trait，用于将OutputFormat转换为字符串
impl fmt::Display for OutputFormat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", Into::<&str>::into(*self))
    }
}

impl CmdExcutor for CsvOpts {
    async fn excutor(self) -> anyhow::Result<()> {
        let output = if let Some(output) = self.output {
            output.clone()
        } else {
            format!("output.{}", self.format)
        };
        process_csv(&self.input, output, self.format)
    }
}
