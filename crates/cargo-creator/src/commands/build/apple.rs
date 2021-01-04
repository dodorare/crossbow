use super::BuildContext;
use crate::*;
use clap::Clap;
use creator_tools::types::*;
use creator_tools::*;
use std::path::PathBuf;

#[derive(Clap)]
pub struct AppleBuildCommand {
    /// Build in release mode
    #[clap(long)]
    pub release: bool,
    /// Target directory path
    #[clap(short, long)]
    pub target_dir: Option<PathBuf>,
    /// Provisioning profile name to find in this directory: "~/Library/MobileDevice/Provisioning\ Profiles/"
    #[clap(short, long)]
    provisioning_profile_name: Option<String>,
    /// Absolute path to provisioning profile
    #[clap(short, long)]
    provisioning_profile_path: Option<PathBuf>,
}

impl AppleBuildCommand {
    pub fn run(&self, current_dir: PathBuf) -> Result<()> {
        let build_context = BuildContext::init(&current_dir, self.target_dir.clone())?;
        self.execute(&build_context)?;
        Ok(())
    }

    pub fn execute(&self, build_context: &BuildContext) -> Result<(AppleMetadata, PathBuf)> {
        log::info!("Starting build process");
        let metadata = build_context
            .manifest
            .package
            .as_ref()
            .ok_or(Error::InvalidManifest)?
            .metadata
            .as_ref()
            .ok_or(Error::InvalidManifestMetadata)?
            .apple
            .clone();
        let properties = &metadata.info_plist;
        let project_path = &build_context.project_path;
        let name = properties.launch.bundle_executable.clone().unwrap();
        let profile = match self.release {
            true => Profile::Release,
            false => Profile::Debug,
        };
        log::info!("Compiling app");
        let build_target = metadata.build_targets.as_ref().unwrap()[0];
        apple_rust_compile(&name, build_target, &project_path, profile, vec![])?;
        let out_dir = build_context
            .target_dir
            .join(build_target.rust_triple())
            .join(&profile);
        let bin_path = out_dir.join(&name);
        log::info!("Generating app folder");
        let app_dir = gen_apple_app(
            &build_context.target_dir,
            &name,
            Some(project_path.join(metadata.resources.as_ref().unwrap())),
            Some(project_path.join(metadata.assets.as_ref().unwrap())),
        )?;
        log::info!("Coping binary to app folder");
        std::fs::copy(&bin_path, &app_dir.join(&name)).unwrap();
        log::info!("Generating Info.plist");
        gen_apple_plist(&app_dir, properties, false).unwrap();
        log::info!("Build finished successfully");
        // TODO: Support apple silicon simulators
        if build_target != AppleTarget::X86_64AppleIos {
            log::info!("Starting code signing process");
            let profile_path = if let Some(path) = self.provisioning_profile_path.clone() {
                path
            } else if let Some(name) = self.provisioning_profile_name.clone() {
                let profiles_path = "~/Library/MobileDevice/Provisioning Profiles";
                format!("{}/{}", profiles_path, name).into()
            } else {
                return Err(Error::CodeSigningFlagsNotProvided);
            };
            log::info!("Profile path: {:?}", profile_path);
            // profile_path
            // codesign(&app_dir, true, None, None)?;
        }
        Ok((metadata, app_dir))
    }
}
