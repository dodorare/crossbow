use crate::error::*;
use simctl::{DeviceQuery, Simctl, list::DeviceState};
use std::{
    collections::HashMap,
    path::{Path, PathBuf},
    process::Command,
};

/// Options for selecting and launching an iOS Simulator application.
#[derive(Clone, Copy, Debug)]
pub struct IosSimulatorLaunchOptions<'a> {
    /// Simulator name or UDID. When omitted, Crossbundle chooses automatically.
    pub simulator: Option<&'a str>,
    /// Whether to open the Simulator UI (`Simulator.app`, or `DeviceHub.app` on Xcode 27+).
    pub open: bool,
    /// Whether to return after launching instead of attaching to the application console.
    pub detach: bool,
}

/// The selected iOS Simulator.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IosSimulator {
    /// Human-readable device name.
    pub name: String,
    /// CoreSimulator device identifier.
    pub udid: String,
    /// The Simulator UI application that was opened, if one was requested and found.
    pub ui: Option<PathBuf>,
}

/// Selects an iOS Simulator, installs the application, and launches it.
pub fn launch_ios_simulator_app(
    app_path: &Path,
    bundle_id: &str,
    options: IosSimulatorLaunchOptions<'_>,
) -> Result<IosSimulator> {
    let developer_dir = developer_dir()?;
    let simctl = Simctl::with_developer_dir(&developer_dir);
    let device_list = simctl.list()?;
    let runtime_versions: HashMap<_, _> = device_list
        .runtimes()
        .iter()
        .filter(|runtime| {
            runtime.is_available
                && runtime
                    .identifier
                    .starts_with("com.apple.CoreSimulator.SimRuntime.iOS-")
        })
        .filter_map(|runtime| {
            version_key(&runtime.version).map(|version| (runtime.identifier.as_str(), version))
        })
        .collect();
    let device = device_list
        .devices()
        .iter()
        .available()
        .filter_map(|device| {
            runtime_versions
                .get(device.runtime_identifier.as_str())
                .map(|version| (device, version))
        })
        .filter(|(device, _)| {
            options
                .simulator
                .is_none_or(|selector| device.name == selector || device.udid == selector)
        })
        .max_by(|(left, left_version), (right, right_version)| {
            (left.state == DeviceState::Booted)
                .cmp(&(right.state == DeviceState::Booted))
                .then_with(|| left_version.cmp(right_version))
                .then_with(|| left.name.cmp(&right.name))
                .then_with(|| left.udid.cmp(&right.udid))
        })
        .map(|(device, _)| device.clone())
        .ok_or_else(|| match options.simulator {
            Some(selector) => AppleError::IosSimulatorUnavailable(selector.to_owned()),
            None => AppleError::IosSimulatorNotFound,
        })?;

    let mut boot = device.simctl().command("bootstatus");
    boot.arg(&device.udid).arg("-b");
    boot.output_err(false)?;
    device.install(app_path)?;
    let ui = if options.open {
        open_simulator_ui(&developer_dir)?
    } else {
        None
    };
    if options.detach {
        let mut launch = device.simctl().command("launch");
        launch.arg(&device.udid).arg(bundle_id);
        launch.output_err(false)?;
    } else {
        device.launch(bundle_id).use_pty(true).exec()?;
    }
    Ok(IosSimulator {
        name: device.name.clone(),
        udid: device.udid.clone(),
        ui,
    })
}

/// Opens the Simulator UI that ships with the selected Xcode and returns its path.
///
/// Xcode 26 and older ship `Simulator.app` inside the developer directory; Xcode 27 replaced
/// it with `DeviceHub.app` next to Xcode's other bundled applications. `simctl` keeps working
/// without either, so a missing UI yields `Ok(None)` rather than an error and callers decide
/// whether to warn.
pub fn open_simulator_ui(developer_dir: &Path) -> Result<Option<PathBuf>> {
    let Some(app) = simulator_ui_candidates(developer_dir)
        .into_iter()
        .find(|path| path.exists())
    else {
        return Ok(None);
    };
    let mut open = Command::new("open");
    open.arg(&app);
    open.output_err(false)?;
    Ok(Some(app))
}

/// Simulator UI applications in preference order, relative to an Xcode developer directory.
fn simulator_ui_candidates(developer_dir: &Path) -> [PathBuf; 2] {
    [
        developer_dir.join("Applications").join("Simulator.app"),
        developer_dir
            .join("..")
            .join("Applications")
            .join("DeviceHub.app"),
    ]
}

/// Resolves the active Xcode developer directory the same way `xcrun` does.
fn developer_dir() -> Result<PathBuf> {
    if let Some(developer_dir) = std::env::var_os("DEVELOPER_DIR") {
        return Ok(PathBuf::from(developer_dir));
    }
    let mut command = Command::new("xcode-select");
    command.arg("--print-path");
    let output = command.output_err(false)?;
    let developer_dir =
        String::from_utf8(output.stdout).map_err(|error| Error::OtherError(Box::new(error)))?;
    Ok(PathBuf::from(developer_dir.trim()))
}

fn version_key(version: &str) -> Option<Vec<u32>> {
    version
        .split('.')
        .map(str::parse)
        .collect::<std::result::Result<_, _>>()
        .ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simulator_ui_prefers_simulator_app_then_device_hub() {
        let developer_dir = Path::new("/Applications/Xcode.app/Contents/Developer");
        let [simulator, device_hub] = simulator_ui_candidates(developer_dir);
        assert_eq!(
            simulator,
            Path::new("/Applications/Xcode.app/Contents/Developer/Applications/Simulator.app")
        );
        assert_eq!(
            device_hub,
            Path::new("/Applications/Xcode.app/Contents/Developer/../Applications/DeviceHub.app")
        );
    }

    #[test]
    fn missing_simulator_ui_is_not_an_error() {
        let developer_dir = std::env::temp_dir().join("crossbundle-no-xcode-here");
        assert_eq!(open_simulator_ui(&developer_dir).unwrap(), None);
    }

    #[test]
    fn simulator_runtime_versions_sort_numerically() {
        assert!(version_key("18.10") > version_key("18.9"));
        assert!(version_key("19.0") > version_key("18.10"));
        assert_eq!(version_key("invalid"), None);
    }
}
