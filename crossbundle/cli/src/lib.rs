#[macro_use]
extern crate log;

pub mod cargo_manifest;
pub mod commands;
pub mod error;

use clap::Parser;
use colored::Colorize;
pub use commands::*;
use crossbundle_tools::utils::{Config, Shell, Verbosity};
use std::path::PathBuf;

#[derive(Parser, Clone, Debug)]
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

impl Opts {
    pub fn get_verbosity(&self) -> Verbosity {
        if self.quiet {
            Verbosity::Quiet
        } else {
            // Vary the output based on how many times the user used the "verbose" flag.
            // Example: `crossbundle -v -v -v' or 'crossbundle -vvv' vs 'crossbundle -v'
            match self.verbose {
                0 => Verbosity::Normal,
                1 => Verbosity::Verbose,
                _ => {
                    pretty_env_logger::formatted_builder()
                        .filter_level(log::LevelFilter::Trace)
                        .init();
                    Verbosity::Verbose
                }
            }
        }
    }

    pub fn get_current_dir(&self) -> PathBuf {
        self.current_dir
            .clone()
            .unwrap_or_else(|| std::env::current_dir().unwrap())
    }
}

pub fn run() -> std::result::Result<(), Box<dyn std::error::Error>> {
    let opts = Opts::parse();
    let mut shell = Shell::new();
    shell.set_verbosity(opts.get_verbosity());
    let config = Config::new(shell, opts.get_current_dir());
    opts.cmd.handle_command(&config)?;
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
