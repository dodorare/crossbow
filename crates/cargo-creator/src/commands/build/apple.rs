use super::BuildContext;
use crate::*;
use clap::Clap;
use creator_tools::types::*;
use creator_tools::*;
use std::path::PathBuf;

#[derive(Clap)]
pub struct AppleBuildCommand {
    /// Build in release mode.
    #[clap(long)]
    pub release: bool,
    /// Target directory path.
    #[clap(short, long)]
    pub target_dir: Option<PathBuf>,
    /// Specify custom cargo binary.
    #[clap(long)]
    pub bin: Option<String>,
    /// Provisioning profile name to find in this directory: "~/Library/MobileDevice/Provisioning\ Profiles/".
    #[clap(long)]
    pub profile_name: Option<String>,
    /// Absolute path to provisioning profile.
    #[clap(long)]
    pub profile_path: Option<PathBuf>,
    /// The team identifier of your signing identity.
    #[clap(long)]
    pub team_identifier: Option<String>,
    /// The id of the identity used for signing.
    #[clap(long)]
    pub identity: Option<String>,
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
        // TODO: Support apple silicon simulators without signing
        if build_target != AppleTarget::X86_64AppleIos {
            log::info!("Starting code signing process");
            copy_profile(
                &app_dir,
                self.profile_name.clone(),
                self.profile_path.clone(),
            )?;
            log::info!("Generating xcent file");
            let xcent_path = gen_xcent(
                &app_dir,
                &name,
                self.team_identifier
                    .as_ref()
                    .ok_or(Error::TeamIdentifierNotProvided)?,
                &properties.identification.bundle_identifier,
                false,
            )?;
            log::info!("Signing the binary");
            codesign(&app_dir.join(&name), true, self.identity.clone(), None)?;
            log::info!("Signing the bundle itself");
            codesign(&app_dir, true, self.identity.clone(), Some(xcent_path))?;
            log::info!("Code signing process finished");
        }
        log::info!("Build finished successfully");
        Ok((metadata, app_dir))
    }
}
