mod commands;
mod error;
mod manifest;
mod utils;

use clap::Clap;
use colored::Colorize;
use commands::*;
use error::*;
use log::LevelFilter;
use manifest::*;
use std::path::PathBuf;
use utils::*;

#[derive(Clap)]
#[clap(author, about, version)]
pub struct Opts {
    /// The current directory where to run all commands
    #[clap(short, long)]
    pub current_dir: Option<PathBuf>,
    /// A level of verbosity, and can be used multiple times
    #[clap(short, long, parse(from_occurrences))]
    pub verbose: u32,
    /// No output printed to stdout
    #[clap(short, long)]
    pub quiet: bool,

    #[clap(subcommand)]
    pub cmd: Commands,
}

fn main() {
    handle_errors(run);
}

fn run() -> std::result::Result<(), Box<dyn std::error::Error>> {
    let opts = Opts::parse();
    let level_filter = if opts.quiet {
        LevelFilter::Error
    } else {
        // Vary the output based on how many times the user used the "verbose" flag.
        // Example: `creator -v -v -v' or 'creator -vvv' vs 'creator -v'
        match opts.verbose {
            0 => LevelFilter::Info,
            1 => LevelFilter::Debug,
            _ => LevelFilter::Trace,
        }
    };
    pretty_env_logger::formatted_builder()
        .filter_level(level_filter)
        .init();
    log::trace!("Successfully initialized logger");
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
        eprintln!("{}: {}", "caused by".red().bold(), error);
        handle_error_source(error.source());
    }
}
