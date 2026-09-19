use crate::error::*;
use std::{path::Path, process::Command};

/// Installs and launches an iOS application on a physical device.
///
/// Xcode 15 and newer ship `devicectl`, which speaks CoreDevice and therefore reaches iOS 17+
/// phones; it is used whenever the selected Xcode provides it. Older toolchains fall back to
/// `ios-deploy`. With `debug`, `devicectl` keeps the application attached to the console,
/// while `ios-deploy` starts its debugger.
pub fn launch_ios_device_app(app_path: &Path, debug: bool, device_id: Option<&str>) -> Result<()> {
    if devicectl_available() {
        launch_with_devicectl(app_path, debug, device_id)
    } else {
        launch_with_ios_deploy(app_path, debug, device_id)
    }
}

fn devicectl_available() -> bool {
    Command::new("xcrun")
        .args(["--find", "devicectl"])
        .output()
        .map(|output| output.status.success())
        .unwrap_or(false)
}

fn launch_with_devicectl(app_path: &Path, console: bool, device_id: Option<&str>) -> Result<()> {
    let device = match device_id {
        Some(device_id) => device_id.to_owned(),
        None => first_paired_physical_device()?,
    };
    let mut install = Command::new("xcrun");
    install
        .args(["devicectl", "device", "install", "app", "--device", &device])
        .arg(app_path);
    install.output_err(true)?;

    let bundle_id = super::read_info_plist(&app_path.join("Info.plist"))?
        .identification
        .bundle_identifier;
    let mut launch = Command::new("xcrun");
    launch.args([
        "devicectl",
        "device",
        "process",
        "launch",
        "--terminate-existing",
        "--device",
        &device,
    ]);
    if console {
        launch.arg("--console");
    }
    launch.arg(&bundle_id);
    launch.output_err(true)?;
    Ok(())
}

fn launch_with_ios_deploy(app_path: &Path, debug: bool, device_id: Option<&str>) -> Result<()> {
    let mut cmd = Command::new("ios-deploy");
    if debug {
        cmd.arg("--debug");
    }
    if let Some(device_id) = device_id {
        cmd.args(["--id", device_id]);
    }
    cmd.arg("--bundle").arg(app_path).arg("--no-wifi");
    cmd.output_err(true)?;
    Ok(())
}

/// Asks `devicectl` for its device listing and picks the first paired physical device.
fn first_paired_physical_device() -> Result<String> {
    let listing_path = std::env::temp_dir().join(format!(
        "crossbundle-devicectl-devices-{}.json",
        std::process::id()
    ));
    let mut list = Command::new("xcrun");
    list.args(["devicectl", "list", "devices", "--json-output"])
        .arg(&listing_path);
    list.output_err(false)?;
    let listing = std::fs::read_to_string(&listing_path)?;
    std::fs::remove_file(&listing_path).ok();
    let listing: serde_json::Value = serde_json::from_str(&listing)
        .map_err(|error| AppleError::DevicectlListing(error.to_string()))?;
    paired_physical_device(&listing).ok_or_else(|| AppleError::IosDeviceNotFound.into())
}

/// Extracts the identifier of the first paired physical device from a `devicectl list devices`
/// JSON listing.
fn paired_physical_device(listing: &serde_json::Value) -> Option<String> {
    listing["result"]["devices"]
        .as_array()?
        .iter()
        .find(|device| {
            device["hardwareProperties"]["reality"].as_str() == Some("physical")
                && device["connectionProperties"]["pairingState"].as_str() == Some("paired")
        })
        .and_then(|device| device["identifier"].as_str().map(str::to_owned))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn picks_the_first_paired_physical_device() {
        let listing = serde_json::json!({
            "result": {
                "devices": [
                    {
                        "identifier": "SIMULATOR",
                        "hardwareProperties": { "reality": "simulated" },
                        "connectionProperties": { "pairingState": "paired" }
                    },
                    {
                        "identifier": "UNPAIRED",
                        "hardwareProperties": { "reality": "physical" },
                        "connectionProperties": { "pairingState": "unpaired" }
                    },
                    {
                        "identifier": "PHONE",
                        "hardwareProperties": { "reality": "physical" },
                        "connectionProperties": { "pairingState": "paired" }
                    }
                ]
            }
        });
        assert_eq!(paired_physical_device(&listing).as_deref(), Some("PHONE"));
    }

    #[test]
    fn no_devices_yields_none() {
        assert_eq!(paired_physical_device(&serde_json::json!({})), None);
        assert_eq!(
            paired_physical_device(&serde_json::json!({ "result": { "devices": [] } })),
            None
        );
    }
}
