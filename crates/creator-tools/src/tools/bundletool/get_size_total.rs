use crate::error::*;
use std::path::{Path, PathBuf};
use std::process::Command;
/// To measure the estimated download sizes of APKs in an APK set as they would be served compressed over-the-wire, use the get-size total command:
/// ```
/// bundletool get-size total --apks=/MyApp/my_app.apks
/// ```
/// You can modify the behavior of the get-size total command using the following flags:
#[derive()]
pub struct GetSizeTotal {
    /// (Required) Specifies the path to the existing APK set file whose download size is measured.
    apks: PathBuf,
    /// Specifies the path to the device spec file (from get-device-spec or constructed manually) to use for matching. You can specify a partial path to evaluate a set of configurations.
    device_spec: Option<PathBuf>,
    /// Specifies the dimensions used when computing the size estimates.
    /// Accepts a comma-separated list of: SDK, ABI, SCREEN_DENSITY, and LANGUAGE. To measure across all dimensions, specify ALL.
    dimensions: Option<String>,
    /// Measures the download size of the instant-enabled APKs instead of the installable APKs.
    /// By default, bundletool measures the installable APK download sizes.
    instant: bool,
    /// Specifies a comma-separated list of modules in the APK set to consider in the measurement. The bundletool command automatically includes any dependent modules for the specified set.
    /// By default, the command measures the download size of all modules installed during the first download.
    modules: Option<String>,
}

impl GetSizeTotal {
    fn new(&self, apks: &Path) -> Self {
        Self {
            apks: apks.to_owned(),
            device_spec: None,
            dimensions: None,
            instant: false,
            modules: None,
        }
    }

    fn device_spec(&mut self, device_spec: &Path) -> &mut Self {
        self.device_spec = Some(device_spec);
        self
    }

    fn dimensions(&mut self, dimensions: &String) -> &mut Self {
        self.dimensions = Some(dimensions);
        self
    }

    fn instant(&mut self, instant: bool) -> &mut Self {
        self.instant = instant;
        self
    }

    fn modules(&mut self, modules: &String) -> &mut Self {
        self.modules = Some(modules);
        self
    }

    fn run(&self) -> Result<()> {
        let mut get_size_total = Command::new("get-size total").arg("--apks=").arg(self.apks);
        if let Some(device_spec) = &self.device_spec {
            get_size_total.arg("--device-spec=").arg(&self.device_spec)
        }
        if let Some(dimensoins) = &self.dimensions {
            get_size_total.arg("--dimensions=").arg(&self.dimensions)
        }
        if self.instant {
            get_size_total.arg("--instant")
        }
        if let Some(modules) = &self.modules {
            get_size_total.arg("--modules=").arg(&self.modules)
        }
        get_size_total.output_err(true)?;
        Ok(())
    }
}
