use crate::error::*;
use simctl::{list::DeviceState, DeviceQuery, Simctl};
use std::path::Path;

pub fn launch_apple_app(
    app_path: &Path,
    device_name: &str,
    bundle_id: &str,
    open: bool,
) -> Result<()> {
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
    let path = "/dev/null";
    let result = device.launch(bundle_id).stdout(&path).stderr(&path).exec();
    if open {
        simctl.open()?;
    }
    result?;
    Ok(())
}
