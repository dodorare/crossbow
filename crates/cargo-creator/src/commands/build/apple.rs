use super::{BuildContext, SharedBuildCommand};
use crate::error::*;
use apple_bundle::prelude::InfoPlist;
use clap::Clap;
use creator_tools::{commands::apple, types::*, utils::Config};
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
    pub fn run(&self, config: &Config) -> Result<()> {
        let context = BuildContext::new(config, self.shared.target_dir.clone())?;
        self.execute(config, &context)?;
        Ok(())
    }

    pub fn execute(
        &self,
        config: &Config,
        context: &BuildContext,
    ) -> Result<(InfoPlist, Vec<PathBuf>)> {
        let project_path = &context.project_path;
        let profile = self.shared.profile();
        let (target, package_name) = if let Some(example) = &self.shared.example {
            (Target::Example(example.clone()), example.clone())
        } else if let Some(bin) = &self.bin {
            (Target::Bin(bin.clone()), bin.clone())
        } else {
            (Target::Bin(context.package_name()), context.package_name())
        };
        let properties = context.gen_info_plist(&package_name)?;
        config.status_message("Starting build process", &package_name)?;
        config.status("Compiling app")?;
        let build_targets = context.apple_build_targets(&self.target);
        let mut app_paths = vec![];
        for build_target in build_targets {
            let app_path = self.build_app(
                config,
                context,
                target.clone(),
                project_path,
                build_target,
                &properties,
                profile,
                &package_name,
            )?;
            app_paths.push(app_path);
        }
        Ok((properties, app_paths))
    }

    fn build_app(
        &self,
        config: &Config,
        context: &BuildContext,
        target: Target,
        project_path: &Path,
        build_target: AppleTarget,
        properties: &InfoPlist,
        profile: Profile,
        name: &str,
    ) -> Result<PathBuf> {
        let rust_triple = build_target.rust_triple();
        config.status_message("Compiling for architecture", rust_triple)?;
        apple::compile_rust_for_ios(
            target,
            build_target,
            &project_path,
            profile,
            self.shared.features.clone(),
            self.shared.all_features,
            self.shared.no_default_features,
        )?;
        let out_dir = context.target_dir.join(rust_triple).join(&profile);
        let bin_path = out_dir.join(&name);
        config.status("Generating app folder")?;
        let apple_target_dir = &context
            .target_dir
            .join("apple")
            .join(rust_triple)
            .join(&profile);
        let app_path = apple::gen_apple_app_folder(
            &apple_target_dir,
            &name,
            context.apple_res().as_ref().map(|r| project_path.join(r)),
            context
                .apple_assets()
                .as_ref()
                .map(|r| project_path.join(r)),
        )?;
        config.status("Copying binary to app folder")?;
        std::fs::copy(&bin_path, &app_path.join(&name)).unwrap();
        config.status_message("Generating", "Info.plist")?;
        apple::save_apple_plist(&app_path, properties, false).unwrap();
        if self.identity.is_some() {
            config.status("Starting code signing process")?;
            apple::copy_profile(
                &app_path,
                self.profile_name.clone(),
                self.profile_path.clone(),
            )?;
            config.status_message("Generating", "xcent file")?;
            let xcent_path = apple::gen_xcent(
                &app_path,
                &name,
                self.team_identifier
                    .as_ref()
                    .ok_or(Error::TeamIdentifierNotProvided)?,
                &properties.identification.bundle_identifier,
                false,
            )?;
            config.status("Signing the binary")?;
            apple::codesign(&app_path.join(&name), true, self.identity.clone(), None)?;
            config.status("Signing the bundle itself")?;
            apple::codesign(&app_path, true, self.identity.clone(), Some(xcent_path))?;
            config.status("Code signing process finished")?;
        }
        config.status("Generating ipa file")?;
        apple::gen_apple_ipa(&apple_target_dir, &app_path, &name)?;
        config.status("Build finished successfully")?;
        Ok(app_path)
    }
}
