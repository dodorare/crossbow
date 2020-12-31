mod commands;
mod error;
mod manifest;

use clap::Clap;
use commands::Commands;
use error::*;
use manifest::*;

#[derive(Clap)]
#[clap(author, about, version)]
pub struct Opts {
    #[clap(subcommand)]
    pub cmd: Commands,
}

fn main() -> Result<()> {
    env_logger::init();
    log::trace!("Successfully initialized env logger");
    let opts = Opts::parse();
    log::trace!("Successfully parsed clap commands");
    opts.cmd.handle_command()?;
    log::trace!("Command finished");
    Ok(())
}
