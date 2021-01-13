pub mod android;
pub mod apple;

use android::AndroidBuildCommand;
use apple::AppleBuildCommand;

use crate::*;
use clap::Clap;
use std::path::{Path, PathBuf};

#[derive(Clap, Clone, Debug)]
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

#[derive(Clap, Clone, Debug)]
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
        info!("Parsing cargo manifest");
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
