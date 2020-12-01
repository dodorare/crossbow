use clap::AppSettings;
use clap::Clap;
use std::path::PathBuf;

// Todo: maybe we need to think about better naming

#[derive(Debug, Clap)]
#[clap(setting = AppSettings::NoBinaryName, setting = AppSettings::DisableVersion)]
pub struct CliCargoBuild {
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

#[derive(Debug, Clap)]
#[clap(setting = AppSettings::TrailingVarArg)]
pub struct CliBuildAndroid {
    #[clap(allow_hyphen_values = true)]
    pub cargo_args: Vec<String>,
}

#[derive(Debug, Clap)]
pub enum CliBuildCmd {
    Android(CliBuildAndroid),
}

#[derive(Debug, Clap)]
pub struct CliCreatorBuild {
    #[clap(subcommand)]
    pub cmd: CliBuildCmd,
}

#[derive(Debug, Clap)]
pub enum CliCreatorCmd {
    Build(CliCreatorBuild),
}

#[derive(Debug, Clap)]
#[clap(author, about, version)]
pub struct CliCreator {
    #[clap(subcommand)]
    pub cmd: CliCreatorCmd,
}
