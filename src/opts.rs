use clap::{Parser, Subcommand};
use std::path::Path;

#[derive(Parser, Debug)]
#[command(name = "rcli", author, version, about, long_about = None)]
pub struct Opts {
    #[command(subcommand)]
    pub cmd: SubCommands,
}

#[derive(Subcommand, Debug)]
pub enum SubCommands {
    /// does testing things
    #[command(name = "csv2", about = "show csv, or to other format")]
    Csv(CsvOpts),
}
#[derive(Parser, Debug)]
pub struct CsvOpts {
    #[arg(short, long, value_parser = verify_input_file)]
    pub input: String,
    #[arg(short, long, default_value = "output.json")]
    pub output: String,
    #[arg(short, long, default_value_t = ',')]
    pub delimiter: char,
    #[arg(long, default_value_t = true)]
    pub header: bool,

}

fn verify_input_file(filename: &str) -> Result<String, String> {
    if Path::new(filename).exists() {
        Ok(filename.into())
    } else {
        Err("file is not exist".into())
    }
}
