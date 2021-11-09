use crate::error::*;
use std::path::{Path, PathBuf};
use std::process::Command;

/// ## Generate a set of APKs from your app bundle
///
/// When `bundletool` generates APKs from your app bundle,it includes them in a container
/// called an APK set archive, which uses the `.apks` file extension. To generate an APK
/// set for all device configurations your app supports from your app bundle, use the
/// `bundletool build-apks` command, as shown below.
///
/// ```xml
/// bundletool build-apks --bundle=/MyApp/my_app.aab --output=/MyApp/my_app.apks
/// ```
///
/// If you want to deploy the APKs to a device, you need to also include your app’s
/// signing information, as shown in the command below. If you do not specify signing
/// information, bundletool attempts to sign your APKs with a debug key for you.
///
/// ```xml
/// bundletool build-apks --bundle=/MyApp/my_app.aab --output=/MyApp/my_app.apks
/// --ks=/MyApp/keystore.jks
/// --ks-pass=file:/MyApp/keystore.pwd
/// --ks-key-alias=MyKeyAlias
/// --key-pass=file:/MyApp/key.pwd
/// ```
///
/// The table below describes the various flags and options you can set when using the
/// `bundletool build-apks` command in greater detail. Only `--bundle` and `--output` are
/// required—all other flags are optional.
#[derive(Debug, PartialEq)]
pub struct BuildApks {
    bundle: PathBuf,
    output: PathBuf,
    overwrite: bool,
    aapt2: Option<PathBuf>,
    ks: Option<PathBuf>,
    ks_pass_pass: Option<String>,
    ks_pass_file: Option<PathBuf>,
    ks_key_alias: Option<String>,
    key_pass_pass: Option<String>,
    key_pass_file: Option<PathBuf>,
    connected_device: bool,
    device_id: Option<String>,
    device_spec: Option<PathBuf>,
    mode_universal: bool,
    local_testing: bool,
}

#[derive(Debug, PartialEq)]
pub enum KsPass {
    KsPassPass,
    KsPassFile,
}

#[derive(Debug, PartialEq)]
pub enum KeyPass {
    KeyPassPass,
    KeyPassFile,
}

/// (`Required`) Specifies the path to the app bundle you built using Android Studio.
/// To learn more, read [`Build your project`].
///
/// [`Build your project`]::https://developer.android.com/studio/run#reference
///
/// (Required) Specifies the name of the output `.apks` file, which contains all the
/// APK artifacts for your app. To test the artifacts in this file on a device, go to
/// the section about how to [`deploy APKs to a connected device`].
///
/// [`deploy APKs to a connected device`]::https://developer.android.com/studio/command-line/bundletool#deploy_with_bundletool
impl BuildApks {
    pub fn new(bundle: &Path, output: &Path) -> Self {
        Self {
            bundle: bundle.to_owned(),
            output: output.to_owned(),
            overwrite: false,
            aapt2: None,
            ks: None,
            ks_pass_pass: None,
            ks_pass_file: None,
            ks_key_alias: None,
            key_pass_pass: None,
            key_pass_file: None,
            connected_device: false,
            device_id: None,
            device_spec: None,
            mode_universal: false,
            local_testing: false,
        }
    }

    /// Include this flag if you want to overwrite any existing output file with the same
    /// path you specify using the --output option. If you don't include this flag and the
    /// output file already exists, you get a build error.
    pub fn overwrite(&mut self, overwrite: bool) -> &mut Self {
        self.overwrite = overwrite;
        self
    }

    /// Specifies a custom path to AAPT2. By default, bundletool includes its own version
    /// of AAPT2.
    pub fn aapt2(&mut self, aapt2: &Path) -> &mut Self {
        self.aapt2 = Some(aapt2.to_owned());
        self
    }

    /// Specifies the path to the deployment keystore used to sign the APKs. This flag is
    /// optional. If you don't include it, bundletool attempts to sign your APKs with a
    /// debug signing key.
    pub fn ks(&mut self, ks: &Path) -> &mut Self {
        self.ks = Some(ks.to_owned());
        self
    }

    /// Specifies your keystore’s password. If you’re specifying a password in plain text,
    /// qualify it with pass:. If you’re passing the path to a file that contains the
    /// password, qualify it with file:. If you specify a keystore using the --ks flag
    /// without specifying --ks-pass, bundletool prompts you for a password from the
    /// command line.
    pub fn ks_pass_pass(&mut self, ks_pass_pass: String) -> &mut Self {
        self.ks_pass_pass = Some(ks_pass_pass);
        self
    }

    /// Specifies your keystore’s password. If you’re specifying a password in plain text,
    /// qualify it with pass:. If you’re passing the path to a file that contains the
    /// password, qualify it with file:. If you specify a keystore using the --ks flag
    /// without specifying --ks-pass, bundletool prompts you for a password from the
    /// command line.
    pub fn ks_pass_file(&mut self, ks_pass_file: &Path) -> &mut Self {
        self.ks_pass_file = Some(ks_pass_file.to_owned());
        self
    }

    /// Specifies the alias of the signing key you want to use.
    pub fn ks_key_alias(&mut self, ks_key_alias: String) -> &mut Self {
        self.ks_key_alias = Some(ks_key_alias);
        self
    }

    ///Specifies the password for the signing key. If you’re specifying a password in
    /// plain text, qualify it with pass:. If you’re passing the path to a file that
    /// contains the password, qualify it with file:.
    ///
    /// If this password is identical to the one for the keystore itself, you can omit
    /// this flag.
    pub fn key_pass_pass(&mut self, key_pass_pass: String) -> &mut Self {
        self.key_pass_pass = Some(key_pass_pass);
        self
    }

    ///Specifies the password for the signing key. If you’re specifying a password in
    /// plain text, qualify it with pass:. If you’re passing the path to a file that
    /// contains the password, qualify it with file:.
    ///
    /// If this password is identical to the one for the keystore itself, you can omit
    /// this flag.
    pub fn key_pass_file(&mut self, key_pass_file: &Path) -> &mut Self {
        self.key_pass_file = Some(key_pass_file.to_owned());
        self
    }

    /// Instructs bundletool to build APKs that target the configuration of a connected
    /// device. If you don’t include this flag, bundletool generates APKs for all device
    /// configurations your app supports.
    pub fn connected_device(&mut self, connected_device: bool) -> &mut Self {
        self.connected_device = connected_device;
        self
    }

    /// If you have more than one connected device, use this flag to specify the serial ID
    /// of the device to which you want to deploy your app.
    pub fn device_id(&mut self, device_id: String) -> &mut Self {
        self.device_id = Some(device_id);
        self
    }

    /// Use this flag to provide a path to a .json file that specifies the device
    /// configuration you want to target. To learn more, go to the section about how to
    /// [`Create and use device specification JSON files`].
    ///
    /// [`Create and use device specification JSON files`]::https://developer.android.com/studio/command-line/bundletool#create_use_json
    pub fn device_spec(&mut self, device_spec: &Path) -> &mut Self {
        self.device_spec = Some(device_spec.to_owned());
        self
    }

    /// Set the mode to universal if you want bundletool to build only a single APK that
    /// includes all of your app's code and resources such that the APK is compatible with
    /// all device configurations your app supports.
    ///
    /// ## Note
    /// `bundletool` includes only feature modules that specify `<dist:fusing
    /// dist:include="true"/>` in their manifest in a universal APK. To learn more, read
    /// about the [`feature module manifest`].
    ///
    /// Keep in mind, these APKs are larger than those optimized for a particular device
    /// configuration. However, they’re easier to share with internal testers who, for
    /// example, want to test your app on multiple device configurations.
    ///
    /// [`feature module manifest`]::https://developer.android.com/guide/playcore/feature-delivery#dynamic_feature_manifest
    pub fn mode_universal(&mut self, mode_universal: bool) -> &mut Self {
        self.mode_universal = mode_universal;
        self
    }

    /// Use this flag to enable your app bundle for local testing. Local testing allows
    /// for quick, iterative testing cycles without the need to upload to Google Play
    /// servers.
    ///
    /// For an example of how to test module installation using the --local-testing flag,
    /// see [`Locally test module installs`].
    ///
    /// [`Locally test module installs`]::https://developer.android.com/guide/app-bundle/test/testing-fakesplitinstallmanager
    pub fn local_testing(&mut self, local_testing: bool) -> &mut Self {
        self.local_testing = local_testing;
        self
    }

    pub fn run(&self) -> Result<()> {
        let mut build_apks = Command::new("java");
        build_apks.arg("-jar");
        if let Ok(bundletool_path) = std::env::var("BUNDLETOOL_PATH") {
            build_apks.arg(bundletool_path);
        } else {
            return Err(AndroidError::BundletoolNotFound.into());
        }
        build_apks.arg("build-apks");
        build_apks.arg("--bundle").arg(&self.bundle);
        build_apks.arg("--output").arg(&self.output);
        if self.overwrite {
            build_apks.arg("--overwrite");
        }
        if let Some(aapt2) = &self.aapt2 {
            build_apks.arg("--aapt2").arg(aapt2);
        }
        if let Some(ks) = &self.ks {
            build_apks.arg("--ks").arg(ks);
        }
        if let Some(ks_pass_pass) = &self.ks_pass_pass {
            build_apks.arg("--ks-pass=pass:").arg(ks_pass_pass);
        }
        if let Some(ks_pass_file) = &self.ks_pass_file {
            build_apks.arg("--ks-pass=file:").arg(ks_pass_file);
        }
        if let Some(ks_key_alias) = &self.ks_key_alias {
            build_apks.arg("--ks-key-alias").arg(ks_key_alias);
        }
        if let Some(key_pass_pass) = &self.key_pass_pass {
            build_apks.arg("--key-pass=pass").arg(key_pass_pass);
        }
        if let Some(key_pass_file) = &self.key_pass_file {
            build_apks.arg("--key-pass=file").arg(key_pass_file);
        }
        if self.connected_device {
            build_apks.arg("--connected-device");
        }
        if let Some(device_id) = &self.device_id {
            build_apks.arg("--device-id").arg(device_id);
        }
        if let Some(device_spec) = &self.device_spec {
            build_apks.arg("--device-spec").arg(device_spec);
        }
        if self.mode_universal {
            build_apks.arg("--mode=universal");
        }
        if self.local_testing {
            build_apks.arg("--local-testing");
        }
        build_apks.output_err(true)?;
        Ok(())
    }
}
