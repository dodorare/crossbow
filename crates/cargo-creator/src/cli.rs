use clap::AppSettings;
pub use clap::Clap;
use std::path::PathBuf;

#[derive(Debug, Clap)]
#[clap(setting = AppSettings::TrailingVarArg)]
pub struct CliCargoBuild {
    #[clap(long)]
    pub bin: Vec<String>,
    #[clap(long)]
    pub bins: bool,
    #[clap(long)]
    pub example: Vec<String>,
    #[clap(long)]
    pub examples: bool,
    #[clap(long)]
    pub target_dir: Option<PathBuf>,
    #[clap(allow_hyphen_values = true)]
    pub cargo_args: Vec<String>,
}

#[derive(Debug, Clap)]
pub struct CliBuildAndroid {
    #[clap(long)]
    pub debug: bool,
    #[clap(flatten)]
    pub cargo: CliCargoBuild,
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
