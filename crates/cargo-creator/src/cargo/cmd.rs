use crate::error::Error;
// pub use artifact::{Artifact, CrateType};
// pub use profile::Profile;
use clap::{AppSettings, Clap, FromArgMatches, IntoApp};
use std::io::BufRead;
use std::path::{Path, PathBuf};
use std::process::Command;

#[derive(Debug, Clap)]
#[clap(setting = AppSettings::NoBinaryName, setting = AppSettings::DisableVersion)]
struct CliCargoBuild {
    #[clap(long, short)]
    pub package: Option<String>,
    #[clap(long)]
    pub workspace: bool,
    #[clap(long)]
    pub all: bool,
    #[clap(long)]
    pub exclude: Vec<String>,
    #[clap(long)]
    pub lib: bool,
    #[clap(long)]
    pub bin: Vec<String>,
    #[clap(long)]
    pub bins: bool,
    #[clap(long)]
    pub example: Vec<String>,
    #[clap(long)]
    pub examples: bool,
    #[clap(long)]
    pub test: Vec<String>,
    #[clap(long)]
    pub tests: bool,
    #[clap(long)]
    pub bench: Vec<String>,
    #[clap(long)]
    pub benches: bool,
    #[clap(long)]
    pub all_targets: bool,
    #[clap(long)]
    pub features: Vec<String>,
    #[clap(long)]
    pub all_features: bool,
    #[clap(long)]
    pub no_default_features: bool,
    #[clap(long)]
    pub target: Vec<String>,
    #[clap(long)]
    pub release: bool,
    #[clap(long)]
    pub target_dir: Option<PathBuf>,
    #[clap(long, short, multiple = true)]
    pub verbose: bool,
    #[clap(long, short)]
    pub quiet: bool,
    #[clap(long)]
    pub color: Option<String>,
    #[clap(long)]
    pub message_format: Option<String>,
    #[clap(long)]
    pub build_plan: bool,
    #[clap(long)]
    pub manifest_path: Option<PathBuf>,
    #[clap(long, alias = "locked")]
    pub frozen: bool,
    #[clap(long)]
    pub offline: bool,
    #[clap(long, short)]
    pub jobs: Option<u32>,
}

#[derive(Debug)]
pub struct CargoBuildArgs {
    args: Vec<String>,
    // package: String,
    // manifest: PathBuf,
    // target_dir: PathBuf,
    // target: Option<String>,
    // host_triple: String,
    // profile: Profile,
    // artifacts: Vec<Artifact>,
    // quiet: bool,
}

impl CargoBuildArgs {
    pub fn from_cargo_args(args: Vec<String>) -> Result<Self, Error> {
        let cli = CliCargoBuild::parse_from(args.clone());
        println!("{:?}", cli);
        Ok(CargoBuildArgs { args })
    }
}
