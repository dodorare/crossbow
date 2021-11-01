use crate::error::*;
use simctl::{list::DeviceState, Device, DeviceQuery, Simctl};
use std::path::Path;

pub fn launch_apple_app(
    app_path: &Path,
    device_name: &str,
    bundle_id: &str,
    open: bool,
) -> Result<Device> {
    let simctl = Simctl::new();
    let device_list = simctl.list()?;
    let device = device_list
        .devices()
        .iter()
        .available()
        .by_name(device_name)
        .next()
        .unwrap();
    if device.state != DeviceState::Booted {
        device.boot()?;
    }
    device.install(app_path)?;
    if open {
        simctl.open()?;
    }
    device.launch(bundle_id).use_pty(true).exec()?;
    Ok(device.clone())
}
