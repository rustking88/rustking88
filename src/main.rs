use clap::Parser;
use rcli::{
    process_csv, process_decode, process_encode, process_genpass, Base64SubCommand, Opts,
    SubCommands,
};

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
        SubCommands::Base64(Base64SubCommand::Encode(opts)) => {
            process_encode(&opts.input, opts.format)?;
        }
        SubCommands::Base64(Base64SubCommand::Decode(opts)) => {
            process_decode(&opts.input, opts.format)?;
        }
    }
    Ok(())
}
