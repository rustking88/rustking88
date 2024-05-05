mod base64;
mod csv;
mod genpass;

pub use self::{
    base64::Base64Format, base64::Base64SubCommand, csv::CsvOpts, csv::OutputFormat,
    genpass::GenPassOpts,
};

use clap::Parser;
use std::path::Path;
#[derive(Parser, Debug)]
#[command(name = "rcli", author, version, about, long_about = None)]
pub struct Opts {
    #[command(subcommand)]
    pub cmd: SubCommands,
}

#[derive(Parser, Debug)]
pub enum SubCommands {
    /// does testing things
    #[command(name = "csv", about = "show csv, or to other format")]
    Csv(CsvOpts),
    #[command(name = "genpass", about = "show json, or to other format")]
    GenPass(GenPassOpts),
    #[command(subcommand)]
    Base64(Base64SubCommand),
}

fn verify_input_file(filename: &str) -> Result<String, String> {
    // if input is "-" or file exists
    if filename == "-" || Path::new(filename).exists() {
        Ok(filename.to_string())
    } else {
        Err(format!("file not found: {}", filename))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_verify_input_file() {
        assert_eq!(verify_input_file("-"), Ok("-".to_string()));
        assert_eq!(verify_input_file("*"), Err("file not found: *".to_string()));
        assert_eq!(
            verify_input_file("Cargo.toml"),
            Ok("Cargo.toml".to_string())
        );
        assert_eq!(
            verify_input_file("notfound"),
            Err("file not found: notfound".to_string())
        );
    }
}
