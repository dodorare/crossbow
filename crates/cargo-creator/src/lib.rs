#[macro_use]
extern crate log;

pub mod commands;
pub mod error;
pub mod manifest;
pub mod utils;

use clap::Clap;
use colored::Colorize;
use commands::*;
use creator_tools::{Config, Shell, Verbosity};
use error::*;
use manifest::*;
use std::path::PathBuf;

#[derive(Clap, Clone, Debug)]
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

pub fn run() -> std::result::Result<(), Box<dyn std::error::Error>> {
    let opts = Opts::parse();
    let verbosity = if opts.quiet {
        Verbosity::Quiet
    } else {
        // Vary the output based on how many times the user used the "verbose" flag.
        // Example: `creator -v -v -v' or 'creator -vvv' vs 'creator -v'
        match opts.verbose {
            0 => Verbosity::Normal,
            1 => Verbosity::Verbose,
            _ => {
                pretty_env_logger::formatted_builder()
                    .filter_level(log::LevelFilter::Trace)
                    .init();
                Verbosity::Verbose
            }
        }
    };
    let mut shell = Shell::new();
    shell.set_verbosity(verbosity);
    let config = Config::new(shell);
    trace!("Successfully initialized logger");
    let current_dir = opts
        .current_dir
        .clone()
        .unwrap_or_else(|| std::env::current_dir().unwrap());
    trace!("Successfully parsed clap commands");
    opts.cmd.handle_command(&config, current_dir)?;
    trace!("Command finished");
    Ok(())
}

pub fn handle_errors(run: impl FnOnce() -> std::result::Result<(), Box<dyn std::error::Error>>) {
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
