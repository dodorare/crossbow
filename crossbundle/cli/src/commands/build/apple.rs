use super::{BuildContext, SharedBuildCommand};
use crate::{error::*, types::CrossbowMetadata};
use apple_bundle::prelude::InfoPlist;
use clap::{ArgAction, Parser};
use crossbundle_tools::{
    commands::{CargoBuild, apple, combine_folders},
    types::*,
};
use std::path::{Path, PathBuf};

#[derive(Parser, Clone, Debug)]
pub struct IosBuildCommand {
    #[clap(flatten)]
    pub shared: SharedBuildCommand,
    /// Build the specified Cargo binary target.
    #[clap(long, conflicts_with = "example")]
    pub bin: Option<String>,
    /// Build for the given iOS Rust target.
    /// Supported targets are: `aarch64-apple-ios`, `aarch64-apple-ios-sim`,
    /// `x86_64-apple-ios`
    #[clap(long, short, action = ArgAction::Append)]
    pub target: Vec<IosTarget>,
    /// Absolute path to provisioning profile.
    #[clap(long, requires = "signing_identity")]
    pub profile_path: Option<PathBuf>,
    /// Apple Developer Team ID.
    #[clap(long, requires = "signing_identity")]
    pub team_id: Option<String>,
    /// Certificate name or SHA-1 hash used to sign the application.
    #[clap(long, requires_all = ["profile_path", "team_id"])]
    pub signing_identity: Option<String>,
}

impl IosBuildCommand {
    pub fn run(&self, config: &Config) -> Result<()> {
        let context = BuildContext::new(config, &self.shared)?;
        self.execute(config, &context)?;
        Ok(())
    }

    pub fn execute(
        &self,
        config: &Config,
        context: &BuildContext,
    ) -> Result<(InfoPlist, Vec<(IosTarget, PathBuf)>)> {
        let profile = self.shared.profile();
        let target = context
            .project
            .executable_target(self.bin.as_deref(), self.shared.example.as_deref())?;
        let package_name = target.name().to_owned();
        let properties = Self::gen_info_plist(context, &package_name)?;
        config.status_message("Starting build process", &package_name)?;
        config.status("Compiling app")?;
        let build_targets = Self::ios_build_targets(context, profile, &self.target);
        let mut app_paths = vec![];
        for build_target in build_targets {
            let app_path = self.build_app(
                config,
                context,
                &target,
                build_target,
                &properties,
                profile,
                &package_name,
            )?;
            app_paths.push((build_target, app_path));
        }
        Ok((properties, app_paths))
    }

    fn build_app(
        &self,
        config: &Config,
        context: &BuildContext,
        target: &CargoTargetSelection,
        build_target: IosTarget,
        properties: &InfoPlist,
        profile: Profile,
        name: &str,
    ) -> Result<PathBuf> {
        let rust_triple = build_target.rust_triple();
        config.status_message("Compiling for target", rust_triple)?;
        let bin_path = apple::compile_ios_executable(
            CargoBuild {
                package: &context.project.package,
                target,
                target_triple: rust_triple,
                target_dir: &context.target_dir,
                profile,
                features: &self.shared.features,
                all_features: self.shared.all_features,
                no_default_features: self.shared.no_default_features,
            },
            properties
                .operating_system_version
                .minimum_os_version
                .as_deref(),
        )?;

        config.status("Generating app folder")?;
        let apple_target_dir = &context
            .target_dir
            .join("apple")
            .join(rust_triple)
            .join(profile);

        config.status("Preparing resources and assets")?;
        let (assets, resources) =
            Self::prepare_assets_and_resources(&context.config, apple_target_dir)?;

        let app_path = apple::gen_apple_app_folder(apple_target_dir, name, assets, resources)?;
        config.status("Copying binary to app folder")?;
        std::fs::copy(bin_path, app_path.join(name))?;
        config.status_message("Generating", "Info.plist")?;
        apple::save_info_plist(&app_path, properties, false)?;

        if build_target.is_simulator() && self.signing_identity.is_none() {
            config.status("Ad-hoc signing simulator application")?;
            apple::codesign(&app_path, true, None, None)?;
        } else if self.signing_identity.is_some() {
            config.status("Starting code signing process")?;
            apple::copy_profile(
                &app_path,
                self.profile_path.as_deref().ok_or_else(|| {
                    anyhow::anyhow!("--profile-path is required with --signing-identity")
                })?,
            )?;
            config.status_message("Generating", "xcent file")?;
            let xcent_path = apple::gen_xcent(
                &app_path,
                name,
                self.team_id.as_ref().ok_or_else(|| {
                    anyhow::anyhow!("--team-id is required with --signing-identity")
                })?,
                &properties.identification.bundle_identifier,
                false,
            )?;
            config.status("Signing the binary")?;
            apple::codesign(
                &app_path.join(name),
                true,
                self.signing_identity.as_deref(),
                None,
            )?;
            config.status("Signing the bundle itself")?;
            apple::codesign(
                &app_path,
                true,
                self.signing_identity.as_deref(),
                Some(&xcent_path),
            )?;
            config.status("Code signing process finished")?;
        }

        config.status("Generating ipa file")?;
        apple::gen_apple_ipa(apple_target_dir, &app_path, name)?;
        config.status("Build finished successfully")?;
        Ok(app_path)
    }

    /// Get iOS build targets from Cargo metadata.
    pub fn ios_build_targets(
        context: &BuildContext,
        profile: Profile,
        build_targets: &[IosTarget],
    ) -> Vec<IosTarget> {
        if !build_targets.is_empty() {
            return build_targets.into();
        }
        if profile == Profile::Debug && !context.config.apple.debug_build_targets.is_empty() {
            return context.config.apple.debug_build_targets.clone();
        }
        if profile == Profile::Release && !context.config.apple.release_build_targets.is_empty() {
            return context.config.apple.release_build_targets.clone();
        }
        vec![IosTarget::host_simulator()]
    }

    /// Get info plist from the path in cargo manifest or generate it with the given
    /// configuration
    pub fn gen_info_plist(context: &BuildContext, package_name: &str) -> Result<InfoPlist> {
        Ok(apple::resolve_info_plist(
            &context.config,
            package_name,
            context.config.apple.info_plist_path.as_deref(),
        )?)
    }

    /// Prepare assets and resources for the application.
    pub fn prepare_assets_and_resources(
        config: &CrossbowMetadata,
        out_dir: &Path,
    ) -> Result<(Option<PathBuf>, Option<PathBuf>)> {
        let res = config.get_apple_resources();
        let gen_resources = if res.is_empty() && config.icon.is_none() {
            None
        } else {
            let path = out_dir.join("gen_resources");
            std::fs::remove_dir_all(&path).ok();
            combine_folders(res, &path)?;

            // TODO: Generate icons
            Some(path)
        };

        let assets = config.get_apple_assets();
        let gen_assets = if !assets.is_empty() {
            let path = out_dir.join("gen_assets");
            std::fs::remove_dir_all(&path).ok();
            combine_folders(assets, &path)?;
            Some(path)
        } else {
            None
        };
        Ok((gen_assets, gen_resources))
    }
}
