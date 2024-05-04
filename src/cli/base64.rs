use core::fmt;
use std::str::FromStr;

use clap::Parser;

use super::verify_input_file;

#[derive(Parser, Debug)]
pub enum Base64SubCommand {
    #[command(name = "encode", about = "encode base64")]
    Encode(Base64EncodeOpts),
    #[command(name = "decode", about = "decode base64")]
    Decode(Base64DecodeOpts),
}

#[derive(Parser, Debug)]
pub struct Base64EncodeOpts {
    #[arg(short, long, default_value = "-", value_parser = verify_input_file)]
    pub input: String,
    #[arg(long, default_value = "standard", value_parser = parse_base64_format)]
    pub format: Base64Format,
}

#[derive(Parser, Debug)]
pub struct Base64DecodeOpts {
    #[arg(short, long, default_value = "-", value_parser = verify_input_file)]
    pub input: String,
    #[arg(long, default_value = "standard", value_parser = parse_base64_format)]
    pub format: Base64Format,
}

#[derive(Debug, Clone, Copy)]
pub enum Base64Format {
    Standard,
    UrlSafe,
}

fn parse_base64_format(format: &str) -> Result<Base64Format, anyhow::Error> {
    format.parse()
}

impl FromStr for Base64Format {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "standard" => Ok(Base64Format::Standard),
            "urlsafe" => Ok(Base64Format::UrlSafe),
            _ => Err(anyhow::anyhow!("invalid format")),
        }
    }
}

impl fmt::Display for Base64Format {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Base64Format::Standard => write!(f, "standard"),
            Base64Format::UrlSafe => write!(f, "urlsafe"),
        }
    }
}
