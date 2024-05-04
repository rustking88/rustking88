use clap::Parser;
use std::{fmt, str::FromStr};

use super::verify_input_file;

#[derive(Debug, Clone, Copy)]
pub enum OutputFormat {
    Json,
    Yaml,
}
#[derive(Parser, Debug)]
pub struct CsvOpts {
    #[arg(short, long, value_parser = verify_input_file)]
    pub input: String,
    #[arg(short, long)]
    pub output: Option<String>,
    #[arg(short, long, default_value_t = ',')]
    pub delimiter: char,
    #[arg(long, value_parser = parse_format, default_value = "json")]
    pub format: OutputFormat,
    #[arg(long, default_value_t = true)]
    pub header: bool,
}

fn parse_format(format: &str) -> Result<OutputFormat, &'static str> {
    format.parse()
}

impl FromStr for OutputFormat {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "json" => Ok(OutputFormat::Json),
            "yaml" => Ok(OutputFormat::Yaml),
            _ => Err("invalid format"),
        }
    }
}

impl fmt::Display for OutputFormat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            OutputFormat::Json => write!(f, "json"),
            OutputFormat::Yaml => write!(f, "yaml"),
        }
    }
}
