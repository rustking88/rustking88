use clap::Parser;
use rcli::{Opts, SubCommands};
use rcli::process_csv;





fn main() -> anyhow::Result<()> {
    let opts = Opts::parse();
    match opts.cmd {
        SubCommands::Csv(opts) => process_csv(&opts.input, opts.output.unwrap_or(format!("output.{}", opts.format)), opts.format)?,
    } 
    Ok(())
}
