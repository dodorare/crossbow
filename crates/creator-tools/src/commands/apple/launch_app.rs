use crate::commands::Command;
use crate::error::*;
use simctl::{list::DeviceState, DeviceQuery, Simctl};
use std::path::PathBuf;

pub struct LaunchAppleApp {
    pub app_path: PathBuf,
    pub device_name: String,
    pub bundle_id: String,
    pub open: bool,
}

impl LaunchAppleApp {
    pub fn new(app_path: PathBuf, device_name: String, bundle_id: String, open: bool) -> Self {
        Self {
            app_path,
            device_name,
            bundle_id,
            open,
        }
    }
}

impl Command for LaunchAppleApp {
    type Deps = ();
    type Output = ();

    fn run(&self) -> Result<Self::Output> {
        let simctl = Simctl::new();
        let device_list = simctl.list()?;
        let device = device_list
            .devices()
            .iter()
            .available()
            .by_name(&self.device_name)
            .next()
            .unwrap();
        if device.state != DeviceState::Booted {
            device.boot()?;
        }
        device.install(&self.app_path)?;
        let path = "/dev/null";
        let result = device
            .launch(&self.bundle_id)
            .stdout(&path)
            .stderr(&path)
            .exec();
        if self.open {
            simctl.open()?;
        }
        result?;
        Ok(())
    }
}
