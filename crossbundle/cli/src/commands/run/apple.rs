use crate::commands::build::{BuildContext, apple::IosBuildCommand};
use crate::error::*;
use clap::Parser;
use crossbundle_tools::{commands::apple, types::CliContext, types::*};
use std::path::{Path, PathBuf};

#[derive(Parser, Clone, Debug)]
pub struct IosRunCommand {
    #[clap(flatten)]
    pub build_command: IosBuildCommand,
    /// Simulator name or UDID. Defaults to a booted or the newest available iOS Simulator.
    #[clap(short, long, value_name = "NAME_OR_UDID", conflicts_with = "device")]
    pub simulator: Option<String>,
    /// Do not open Simulator.app
    #[clap(long, conflicts_with = "device")]
    pub no_open: bool,
    /// Return after launching instead of attaching to the application console
    #[clap(long, conflicts_with = "device")]
    pub detach: bool,
    /// Start the debugger when running on a connected device
    #[clap(long, requires = "device")]
    pub debug: bool,
    /// Install and launch on the connected device
    #[clap(short, long, conflicts_with = "target", requires = "signing_identity")]
    pub device: bool,
    /// Connected device id
    #[clap(short = 'D', long, requires = "device")]
    pub device_id: Option<String>,
}

impl IosRunCommand {
    pub fn run(&self, config: &CliContext) -> Result<()> {
        let mut build_command = self.build_command.clone();
        if build_command.target.is_empty() {
            build_command.target.push(if self.device {
                IosTarget::Aarch64Device
            } else {
                IosTarget::host_simulator()
            });
        }
        let context = BuildContext::new(config, &build_command.shared)?;
        let (info_plist, app_paths) = build_command.execute(config, &context)?;
        config.status("Starting run process")?;
        let bundle_id = &info_plist.identification.bundle_identifier;
        let app_path = self.get_app_path(&app_paths)?;
        if self.device {
            config.shell().status("Launching app on connected device")?;
            apple::launch_ios_device_app(app_path, self.debug, self.device_id.as_deref())?;
        } else {
            config.status("Installing and launching application on simulator")?;
            let device = apple::launch_ios_simulator_app(
                app_path,
                bundle_id,
                apple::IosSimulatorLaunchOptions {
                    simulator: self.simulator.as_deref(),
                    open: !self.no_open,
                    detach: self.detach,
                },
            )?;
            config.status_message("Simulator", format!("{} ({})", device.name, device.udid))?;
        }
        config.status("Run finished successfully")?;
        Ok(())
    }

    fn get_app_path<'a>(&self, app_paths: &'a [(IosTarget, PathBuf)]) -> Result<&'a Path> {
        let preferred = if self.device {
            IosTarget::Aarch64Device
        } else {
            IosTarget::host_simulator()
        };
        app_paths
            .iter()
            .find(|(target, _)| *target == preferred)
            .or_else(|| {
                if self.device {
                    None
                } else {
                    app_paths.iter().find(|(target, _)| target.is_simulator())
                }
            })
            .map(|(_, path)| path.as_path())
            .ok_or_else(|| {
                anyhow::anyhow!(
                    "the build did not produce a runnable iOS {} artifact",
                    if self.device { "device" } else { "Simulator" }
                )
                .into()
            })
    }
}
