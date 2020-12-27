use crate::commands::Command;
use crate::error::*;
use simctl::{DeviceQuery, Simctl};
use std::{io::Write, path::PathBuf};

pub struct LaunchAppleApp {
    pub simctl: Simctl,
    pub project_dir: PathBuf,
}

impl LaunchAppleApp {
    pub fn new(project_dir: PathBuf) -> Self {
        Self {
            simctl: Simctl::new(),
            project_dir,
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
            .by_name("iPhone 12 Pro")
            .next()
            .unwrap();
        println!("device: {:?}", device);
        // let _ = device.boot();
        // device.launch("com.apple.mobilesafari").exec()?;
        // let image = device.io().screenshot(
        //     simctl::io::ImageType::Png,
        //     simctl::io::Display::Internal,
        //     simctl::io::Mask::Ignored,
        // )?;
        // device.shutdown()?;
        // let mut file = std::fs::File::create(self.project_dir.join("scrn.png"))?;
        // file.write_all(&image)?;
        Ok(())
    }
}
