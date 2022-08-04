#[cfg(feature = "android")]
pub mod android;
#[cfg(feature = "apple")]
pub mod apple;
mod build_context;

pub use build_context::*;

#[cfg(feature = "android")]
use android::AndroidBuildCommand;
#[cfg(feature = "apple")]
use apple::IosBuildCommand;

use crate::error::Result;
use clap::Parser;
use crossbundle_tools::types::{Config, Profile};
use std::path::PathBuf;

#[derive(Parser, Clone, Debug)]
pub enum BuildCommand {
    /// Starts the process of building/packaging/signing of the rust crate for Android
    #[cfg(feature = "android")]
    Android(AndroidBuildCommand),
    /// Starts the process of building/packaging/signing of the rust crate for iOS
    #[cfg(feature = "apple")]
    Ios(IosBuildCommand),
}

impl BuildCommand {
    pub fn handle_command(&self, config: &Config) -> Result<()> {
        #[cfg(any(feature = "android", feature = "apple"))]
        match &self {
            #[cfg(feature = "android")]
            Self::Android(cmd) => cmd.run(config)?,
            #[cfg(feature = "apple")]
            Self::Ios(cmd) => cmd.run(config)?,
        }
        Ok(())
    }
}

#[derive(Parser, Clone, Debug, Default)]
pub struct SharedBuildCommand {
    /// Build the specified example
    #[clap(long)]
    pub example: Option<String>,
    /// Space or comma separated list of features to activate. These features only apply
    /// to the current directory's package. Features of direct dependencies may be
    /// enabled with `<dep-name>/<feature-name>` syntax. This flag may be specified
    /// multiple times, which enables all specified features
    #[clap(long)]
    pub features: Vec<String>,
    /// Activate all available features of selected package
    #[clap(long)]
    pub all_features: bool,
    /// Do not activate the `default` feature of the current directory's package
    #[clap(long)]
    pub no_default_features: bool,
    /// Build optimized artifact with the `release` profile
    #[clap(long)]
    pub release: bool,
    /// Directory for generated artifact and intermediate files
    #[clap(long)]
    pub target_dir: Option<PathBuf>,
}

impl SharedBuildCommand {
    pub fn profile(&self) -> Profile {
        match self.release {
            true => Profile::Release,
            false => Profile::Debug,
        }
    }
}
