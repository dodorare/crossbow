use crate::*;
use clap::Clap;
use creator_tools::types::*;
use creator_tools::*;
use std::path::PathBuf;

#[derive(Clap)]
pub struct AppleBuildCommand {
    /// Build profile. Can be one of: debug, release
    #[clap(short, long, default_value = "debug")]
    pub profile: Profile,
    /// Target directory path
    #[clap(short, long)]
    pub target_dir: Option<PathBuf>,
}

impl AppleBuildCommand {
    pub fn run(&self, current_dir: PathBuf) -> Result<()> {
        let workspace_manifest_path = find_workspace_manifest_path(&current_dir)?;
        let package_manifest_path = find_package_manifest_path(&current_dir)?;
        let manifest = Manifest::from_path_with_metadata(&package_manifest_path)?;
        let target_dir = self
            .target_dir
            .clone()
            .unwrap_or_else(|| workspace_manifest_path.parent().unwrap().join("target"));

        let metadata = manifest.package.unwrap().metadata.unwrap().apple;
        let properties = &metadata.info_plist;
        let name = properties.launch.bundle_executable.clone().unwrap();
        // Compile app
        let build_target = metadata.build_targets.unwrap()[0];
        apple_rust_compile(
            &name,
            build_target,
            &package_manifest_path,
            self.profile,
            vec![],
        )?;
        let out_dir = target_dir
            .join(build_target.rust_triple())
            .join(&self.profile);
        let bin_path = out_dir.join(&name);
        // Generate app folder
        let app_dir = gen_apple_app(
            &target_dir,
            &name,
            Some(package_manifest_path.join(metadata.resources.unwrap())),
            Some(package_manifest_path.join(metadata.assets.unwrap())),
        )?;
        // Copy binary to app folder
        std::fs::copy(&bin_path, &app_dir.join(&name)).unwrap();
        // Generate Info.plist
        gen_apple_plist(&app_dir, properties, false).unwrap();
        Ok(())
    }
}
