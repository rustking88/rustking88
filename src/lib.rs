mod cli;
mod process;

pub use self::cli::{
    Base64Format, Base64SubCommand, CsvOpts, GenPassOpts, Opts, OutputFormat, SubCommands,
};
pub use self::process::{process_csv, process_decode, process_encode, process_genpass};
