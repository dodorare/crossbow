use crate::commands::build::{android::AndroidBuildCommand, BuildContext};
use crate::error::Result;
use android_tools::error::CommandExt;
use clap::Parser;
use crossbundle_tools::{
    commands::android,
    commands::android::gradle_init,
    tools::{BuildApks, InstallApks},
    types::AndroidStrategy,
    utils::Config,
};

#[derive(Parser, Clone, Debug)]
pub struct AndroidRunCommand {
    #[clap(flatten)]
    pub build_command: AndroidBuildCommand,
}

impl AndroidRunCommand {
    /// Deployes and runs application in AAB or APK format on your device or emulator
    pub fn run(&self, config: &Config) -> Result<()> {
        let context = BuildContext::new(config, self.build_command.shared.target_dir.clone())?;
        if self.build_command.lib.is_some() {
            config.status("Can not run dynamic library")?;
            return Ok(());
        }
        match self.build_command.strategy {
            AndroidStrategy::NativeApk => {
                self.run_native_apk(config, &context)?;
            }
            AndroidStrategy::NativeAab => {
                self.run_native_aab(config, &context)?;
            }
            AndroidStrategy::GradleApk => {
                self.run_gradle_apk(config, &context)?;
            }
        }
        Ok(())
    }

    pub fn run_native_aab(&self, config: &Config, context: &BuildContext) -> Result<()> {
        let (android_manifest, sdk, aab_path, package_name, key) =
            self.build_command.execute_aab(config, context)?;
        config.status("Generating apks")?;
        let apks = aab_path
            .parent()
            .unwrap()
            .join(format!("{}.apks", package_name));
        let apks_path = BuildApks::new(&aab_path, &apks)
            .overwrite(true)
            .ks(&key.key_path)
            .ks_pass_pass(key.key_pass)
            .ks_key_alias(key.key_alias)
            .run()?;
        config.status("Starting run process")?;
        config.status("Installing APKs file")?;
        InstallApks::new(&apks_path).run()?;
        config.status("Starting APK file")?;
        android::start_apk(
            &sdk,
            &android_manifest.package,
            "android.app.NativeActivity",
        )?;
        config.status("Run finished successfully")?;
        Ok(())
    }

    pub fn run_native_apk(&self, config: &Config, context: &BuildContext) -> Result<()> {
        let (android_manifest, sdk, apk_path) = self.build_command.execute_apk(config, context)?;
        config.status("Starting run process")?;
        config.status("Installing APK file")?;
        android::install_apk(&sdk, &apk_path)?;
        config.status("Starting APK file")?;
        android::start_apk(
            &sdk,
            &android_manifest.package,
            "android.app.NativeActivity",
        )?;
        config.status("Run finished successfully")?;
        Ok(())
    }

    pub fn run_gradle_apk(&self, config: &Config, context: &BuildContext) -> Result<()> {
        let (android_manifest, sdk, gradle_project_path) =
            self.build_command
                .build_gradle(config, context, &self.build_command.export_path)?;
        config.status("Installing APK file on device")?;
        let mut gradle = gradle_init()?;
        gradle
            .arg("installDebug")
            .arg("-p")
            .arg(dunce::simplified(&gradle_project_path));
        gradle.output_err(true)?;
        config.status("Starting APK file")?;
        android::start_apk(
            &sdk,
            &android_manifest.package,
            "com.crossbow.game.CrossbowApp",
        )?;
        config.status("Run finished successfully")?;
        Ok(())
    }
}
