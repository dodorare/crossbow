mod commands;
mod error;
mod manifest;
mod utils;

use clap::Clap;
use colored::Colorize;
use commands::*;
use error::*;
use manifest::*;
use std::path::PathBuf;
use utils::*;

#[derive(Clap)]
#[clap(author, about, version)]
pub struct Opts {
    #[clap(short, long)]
    pub current_dir: Option<PathBuf>,

    #[clap(subcommand)]
    pub cmd: Commands,
}

fn main() {
    env_logger::init();
    handle_errors(run);
}

fn run() -> std::result::Result<(), Box<dyn std::error::Error>> {
    log::trace!("Successfully initialized env logger");
    let opts = Opts::parse();
    let current_dir = opts
        .current_dir
        .clone()
        .unwrap_or_else(|| std::env::current_dir().unwrap());
    log::trace!("Successfully parsed clap commands");
    opts.cmd.handle_command(current_dir)?;
    log::trace!("Command finished");
    Ok(())
}

fn handle_errors(run: impl FnOnce() -> std::result::Result<(), Box<dyn std::error::Error>>) {
    if let Err(err) = run() {
        eprintln!("{}: {}", "error".red().bold(), err);
        if let Some(source) = err.source() {
            eprintln!("{}: {}", "caused by".red().bold(), source);
        }
        std::process::exit(1);
    };
}
