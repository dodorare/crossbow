use super::{BuildContext, SharedBuildCommand};
use crate::*;
use clap::Clap;
use creator_tools::types::*;
use creator_tools::*;
use std::path::PathBuf;

#[derive(Clap)]
pub struct AppleBuildCommand {
    #[clap(flatten)]
    pub shared: SharedBuildCommand,
    /// Specify custom cargo binary.
    #[clap(long, conflicts_with = "example")]
    pub bin: Option<String>,
    /// Provisioning profile name to find in this directory: "~/Library/MobileDevice/Provisioning\ Profiles/".
    #[clap(long, conflicts_with = "profile-path")]
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
        let build_context = BuildContext::init(&current_dir, self.shared.target_dir.clone())?;
        self.execute(&build_context)?;
        Ok(())
    }

    pub fn execute(&self, build_context: &BuildContext) -> Result<(AppleMetadata, PathBuf)> {
        log::info!("Starting build process");
        let package = build_context
            .manifest
            .package
            .as_ref()
            .ok_or(Error::InvalidManifest)?;
        let metadata = package
            .metadata
            .as_ref()
            .ok_or(Error::InvalidManifestMetadata)?
            .apple
            .clone();
        let properties = &metadata.info_plist;
        let project_path = &build_context.project_path;
        let profile = match self.shared.release {
            true => Profile::Release,
            false => Profile::Debug,
        };
        let name;
        let target = if let Some(example) = self.shared.example.clone() {
            name = example;
            Target::Example(name.clone())
        } else if let Some(bin) = self.bin.clone() {
            name = bin;
            Target::Bin(name.clone())
        } else {
            name = package.name.clone();
            Target::Bin(name.clone())
        };
        log::info!("Compiling app");
        let build_target = metadata.build_targets.as_ref().unwrap()[0];
        apple_rust_compile(
            target,
            build_target,
            &project_path,
            profile,
            self.shared.features.clone(),
            self.shared.all_features,
            self.shared.no_default_features,
        )?;
        let out_dir = build_context
            .target_dir
            .join(build_target.rust_triple())
            .join(&profile);
        let bin_path = out_dir.join(&name);
        log::info!("Generating app folder");
        let app_dir = gen_apple_app(
            &build_context.target_dir,
            &name,
            metadata.resources.as_ref().map(|r| project_path.join(r)),
            metadata.assets.as_ref().map(|r| project_path.join(r)),
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
