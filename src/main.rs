use clap::Parser;
use rcli::{process_csv, process_genpass, Opts, SubCommands};

fn main() -> anyhow::Result<()> {
    let opts = Opts::parse();
    match opts.cmd {
        SubCommands::Csv(opts) => process_csv(
            opts.input,
            opts.output.unwrap_or(format!("output.{}", opts.format)),
            opts.format,
        )?,
        SubCommands::GenPass(opts) => process_genpass(
            opts.length,
            opts.uppercase,
            opts.lowercase,
            opts.number,
            opts.symol,
        )?,
    }
    Ok(())
}
