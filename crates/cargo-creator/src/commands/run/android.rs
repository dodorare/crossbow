use crate::commands::build::{android::AndroidBuildCommand, BuildContext};
use crate::error::Result;
use clap::Parser;
use creator_tools::tools::InstallApks;
use creator_tools::{commands::android, utils::Config};

#[derive(Parser, Clone, Debug)]
pub struct AndroidRunCommand {
    #[clap(flatten)]
    pub build_command: AndroidBuildCommand,
}

impl AndroidRunCommand {
    pub fn run(&self, config: &Config) -> Result<()> {
        let context = BuildContext::new(config, self.build_command.shared.target_dir.clone())?;
        if self.build_command.aab {
            let (android_manifest, sdk, aab_path, package_name, key) =
                self.build_command.execute_aab(config, &context)?;
            config.status("Generating apks")?;
            let apks = aab_path
                .parent()
                .unwrap()
                .join(format!("{}.apks", package_name));
            let apks_path = android::build_apks(&aab_path, &apks, key)?;
            config.status("Starting run process")?;
            config.status("Installing APKs file")?;
            InstallApks::new(&apks_path).run()?;
            config.status("Starting APK file")?;
            android::start_apk(&sdk, &android_manifest.package)?;
            config.status("Run finished successfully")?;
        } else {
            let (android_manifest, sdk, apk_path) = self.build_command.execute(config, &context)?;
            config.status("Starting run process")?;
            config.status("Installing APK file")?;
            android::install_apk(&sdk, &apk_path)?;
            config.status("Starting APK file")?;
            android::start_apk(&sdk, &android_manifest.package)?;
            config.status("Run finished successfully")?;
        }
        Ok(())
    }
}
