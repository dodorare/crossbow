pub mod android;
pub mod apple;

use android::AndroidBuildCommand;
use apple::AppleBuildCommand;

use crate::*;
use clap::Clap;
use std::path::{Path, PathBuf};

#[derive(Clap)]
pub enum BuildCommand {
    Android(AndroidBuildCommand),
    Apple(AppleBuildCommand),
}

impl BuildCommand {
    pub fn handle_command(&self, current_dir: PathBuf) -> Result<()> {
        match &self {
            Self::Android(cmd) => cmd.run(current_dir),
            Self::Apple(cmd) => cmd.run(current_dir),
        }
    }
}

#[derive(Clap)]
pub struct SharedBuildCommand {
    /// Build the specified example.
    #[clap(long)]
    pub example: Option<String>,
    /// Space or comma separated list of features to activate. These features only apply to the current
    /// directory's package. Features of direct dependencies may be enabled with `<dep-name>/<feature-name>` syntax.
    /// This flag may be specified multiple times, which enables all specified features.
    #[clap(long)]
    pub features: Vec<String>,
    /// Activate all available features of selected package.
    #[clap(long)]
    pub all_features: bool,
    /// Do not activate the `default` feature of the current directory's package.
    #[clap(long)]
    pub no_default_features: bool,
    /// Build optimized artifact with the `release` profile.
    #[clap(long)]
    pub release: bool,
    /// Directory for generated artifact and intermediate files.
    #[clap(long)]
    pub target_dir: Option<PathBuf>,
}

pub struct BuildContext {
    pub workspace_manifest_path: PathBuf,
    pub package_manifest_path: PathBuf,
    pub project_path: PathBuf,
    pub manifest: Manifest,
    pub target_dir: PathBuf,
}

impl BuildContext {
    pub fn init(current_dir: &Path, target_dir: Option<PathBuf>) -> Result<Self> {
        let workspace_manifest_path = utils::find_workspace_manifest_path(&current_dir)?;
        let package_manifest_path = utils::find_package_manifest_path(&current_dir)?;
        let project_path = package_manifest_path.parent().unwrap().to_owned();
        let target_dir =
            target_dir.unwrap_or_else(|| workspace_manifest_path.parent().unwrap().join("target"));
        log::info!("Parsing cargo manifest");
        let manifest = Manifest::from_path_with_metadata(&package_manifest_path)?;
        Ok(Self {
            workspace_manifest_path,
            package_manifest_path,
            project_path,
            manifest,
            target_dir,
        })
    }
}

// #[derive(Debug, Clap)]
// #[clap(setting = AppSettings::NoBinaryName, setting = AppSettings::DisableVersion)]
// pub struct CliCargoBuild {
//     #[clap(long, short)]
//     pub package: Option<String>,
//     #[clap(long)]
//     pub workspace: bool,
//     #[clap(long)]
//     pub all: bool,
//     #[clap(long)]
//     pub exclude: Vec<String>,
//     #[clap(long)]
//     pub lib: bool,
//     #[clap(long)]
//     pub bin: Vec<String>,
//     #[clap(long)]
//     pub bins: bool,
//     #[clap(long)]
//     pub example: Vec<String>,
//     #[clap(long)]
//     pub examples: bool,
//     #[clap(long)]
//     pub test: Vec<String>,
//     #[clap(long)]
//     pub tests: bool,
//     #[clap(long)]
//     pub bench: Vec<String>,
//     #[clap(long)]
//     pub benches: bool,
//     #[clap(long)]
//     pub all_targets: bool,
//     #[clap(long)]
//     pub features: Vec<String>,
//     #[clap(long)]
//     pub all_features: bool,
//     #[clap(long)]
//     pub no_default_features: bool,
//     #[clap(long)]
//     pub target: Vec<String>,
//     #[clap(long)]
//     pub release: bool,
//     #[clap(long)]
//     pub target_dir: Option<PathBuf>,
//     #[clap(long, short, multiple = true)]
//     pub verbose: bool,
//     #[clap(long, short)]
//     pub quiet: bool,
//     #[clap(long)]
//     pub color: Option<String>,
//     #[clap(long)]
//     pub message_format: Option<String>,
//     #[clap(long)]
//     pub build_plan: bool,
//     #[clap(long)]
//     pub manifest_path: Option<PathBuf>,
//     #[clap(long, alias = "locked")]
//     pub frozen: bool,
//     #[clap(long)]
//     pub offline: bool,
//     #[clap(long, short)]
//     pub jobs: Option<u32>,
// }
