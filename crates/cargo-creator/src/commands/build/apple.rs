use super::{BuildContext, SharedBuildCommand};
use crate::*;
use clap::Clap;
use creator_tools::types::*;
use creator_tools::*;
use std::path::{Path, PathBuf};

#[derive(Clap, Clone, Debug)]
pub struct AppleBuildCommand {
    #[clap(flatten)]
    pub shared: SharedBuildCommand,
    /// Specify custom cargo binary.
    #[clap(long, conflicts_with = "example")]
    pub bin: Option<String>,
    /// Build for the given apple architecture.
    /// Supported targets are: 'aarch64-apple-ios`, `armv7-apple-ios`, `armv7s-apple-ios`, `i386-apple-ios`, `x86_64-apple-ios`.
    #[clap(long, default_value = "aarch64-apple-ios")]
    pub target: Vec<AppleTarget>,
    /// Provisioning profile name to find in this directory: `~/Library/MobileDevice/Provisioning\ Profiles/`.
    #[clap(long, conflicts_with = "profile-path")]
    pub profile_name: Option<String>,
    /// Absolute path to provisioning profile.
    #[clap(long)]
    pub profile_path: Option<PathBuf>,
    /// The team identifier of your signing identity.
    #[clap(long)]
    pub team_identifier: Option<String>,
    /// The id of the identity used for signing. It won't start the signing process until you provide this flag.
    #[clap(long)]
    pub identity: Option<String>,
}

impl AppleBuildCommand {
    pub fn run(&self, current_dir: PathBuf) -> Result<()> {
        let build_context = BuildContext::init(&current_dir, self.shared.target_dir.clone())?;
        self.execute(&build_context)?;
        Ok(())
    }

    pub fn execute(&self, build_context: &BuildContext) -> Result<(AppleMetadata, Vec<PathBuf>)> {
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
            .clone()
            .unwrap();
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
        let build_targets = if !self.target.is_empty() {
            &self.target
        } else {
            metadata
                .build_targets
                .as_ref()
                .ok_or(Error::BuildTargetsNotProvided)?
        };
        let mut app_paths = vec![];
        for build_target in build_targets {
            let app_path = self.build_app(
                build_context,
                &metadata,
                target.clone(),
                project_path,
                *build_target,
                properties,
                profile,
                &name,
            )?;
            app_paths.push(app_path);
        }
        Ok((metadata, app_paths))
    }

    fn build_app(
        &self,
        build_context: &BuildContext,
        metadata: &AppleMetadata,
        target: Target,
        project_path: &Path,
        build_target: AppleTarget,
        properties: &InfoPlist,
        profile: Profile,
        name: &str,
    ) -> Result<PathBuf> {
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
        let app_path = gen_apple_app(
            &build_context
                .target_dir
                .join("apple")
                .join(build_target.rust_triple())
                .join(&profile),
            &name,
            metadata.resources.as_ref().map(|r| project_path.join(r)),
            metadata.assets.as_ref().map(|r| project_path.join(r)),
        )?;
        log::info!("Coping binary to app folder");
        std::fs::copy(&bin_path, &app_path.join(&name)).unwrap();
        log::info!("Generating Info.plist");
        gen_apple_plist(&app_path, properties, false).unwrap();
        if self.identity.is_some() {
            log::info!("Starting code signing process");
            copy_profile(
                &app_path,
                self.profile_name.clone(),
                self.profile_path.clone(),
            )?;
            log::info!("Generating xcent file");
            let xcent_path = gen_xcent(
                &app_path,
                &name,
                self.team_identifier
                    .as_ref()
                    .ok_or(Error::TeamIdentifierNotProvided)?,
                &properties.identification.bundle_identifier,
                false,
            )?;
            log::info!("Signing the binary");
            codesign(&app_path.join(&name), true, self.identity.clone(), None)?;
            log::info!("Signing the bundle itself");
            codesign(&app_path, true, self.identity.clone(), Some(xcent_path))?;
            log::info!("Code signing process finished");
        }
        log::info!("Build finished successfully");
        Ok(app_path)
    }
}
