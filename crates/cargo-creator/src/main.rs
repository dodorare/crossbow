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
    if let Err(error) = run() {
        eprintln!("{}: {}", "error".red().bold(), error);
        handle_error_source(error.source());
        std::process::exit(1);
    };
}

fn handle_error_source(source: Option<&(dyn std::error::Error + 'static)>) {
    if let Some(error) = source {
        if let Some(source) = error.source() {
            eprintln!("{}: {}", "caused by".red().bold(), source);
            handle_error_source(source.source());
        }
    }
}
