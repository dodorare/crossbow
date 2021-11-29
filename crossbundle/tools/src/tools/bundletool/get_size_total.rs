use crate::error::*;
use std::path::{Path, PathBuf};
use std::process::Command;

/// ## Measure the estimated download sizes of APKs in an APK set
///
/// To measure the estimated download sizes of APKs in an APK set as they would be served
/// compressed over-the-wire, use the `get-size total` command:
///
/// ```xml
/// bundletool get-size total --apks=/MyApp/my_app.apks
/// ```
///
/// You can modify the behavior of the `get-size total` command using the following flags:
pub struct GetSizeTotal {
    apks: PathBuf,
    device_spec: Option<PathBuf>,
    dimensions: Option<String>,
    instant: bool,
    modules: Option<String>,
}

impl GetSizeTotal {
    /// (Required) Specifies the path to the existing APK set file whose download size is
    /// measured.
    pub fn new(apks: &Path) -> Self {
        Self {
            apks: apks.to_owned(),
            device_spec: None,
            dimensions: None,
            instant: false,
            modules: None,
        }
    }

    /// Specifies the path to the device spec file (from get-device-spec or constructed
    /// manually) to use for matching. You can specify a partial path to evaluate a set of
    /// configurations.
    pub fn device_spec(&mut self, device_spec: &Path) -> &mut Self {
        self.device_spec = Some(device_spec.to_owned());
        self
    }

    /// Specifies the dimensions used when computing the size estimates.
    /// Accepts a comma-separated list of: `SDK`, `ABI`, `SCREEN_DENSITY`, and `LANGUAGE`.
    /// To measure across all dimensions, specify `ALL`.
    pub fn dimensions(&mut self, dimensions: String) -> &mut Self {
        self.dimensions = Some(dimensions);
        self
    }

    /// Measures the download size of the instant-enabled APKs instead of the installable
    /// APKs. By default, `bundletool` measures the installable APK download sizes
    pub fn instant(&mut self, instant: bool) -> &mut Self {
        self.instant = instant;
        self
    }

    /// Specifies a comma-separated list of modules in the APK set to consider in the
    /// measurement. The `bundletool` command automatically includes any dependent modules
    /// for the specified set. By default, the command measures the download size of
    /// all modules installed during the first download.
    pub fn modules(&mut self, modules: String) -> &mut Self {
        self.modules = Some(modules);
        self
    }

    pub fn run(&self) -> Result<()> {
        let mut get_size_total = Command::new("java");
        get_size_total.arg("-jar");
        if let Ok(bundletool_path) = std::env::var("BUNDLETOOL_PATH") {
            get_size_total.arg(bundletool_path);
        } else {
            return Err(AndroidError::BundletoolNotFound.into());
        }
        get_size_total.arg("get-size");
        get_size_total.arg("total");
        get_size_total.arg("--apks");
        get_size_total.arg(&self.apks);
        if let Some(device_spec) = &self.device_spec {
            get_size_total.arg("--device-spec").arg(device_spec);
        }
        if let Some(dimensions) = &self.dimensions {
            get_size_total.arg("--dimensions").arg(dimensions);
        }
        if self.instant {
            get_size_total.arg("--instant");
        }
        if let Some(modules) = &self.modules {
            get_size_total.arg("--modules").arg(modules);
        }
        get_size_total.output_err(true)?;
        Ok(())
    }
}
