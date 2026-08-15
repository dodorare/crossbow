use crate::error::{Error, Result};
use clap::{ArgAction, Parser};
use crossbundle_tools::toolchain::{
    CheckStatus, DoctorPlatform, DoctorRequest, ReportStatus, diagnose_current, resolve_platforms,
};
use crossbundle_tools::types::Config;
use std::path::PathBuf;

#[derive(Parser, Clone, Debug, Default)]
pub struct DoctorCommand {
    /// Diagnose one or more platforms (repeat or use comma-delimited values)
    #[clap(long, value_delimiter = ',', action = ArgAction::Append)]
    pub platform: Vec<DoctorPlatform>,
    /// Also validate the project at this directory or Cargo.toml path
    #[clap(long, value_name = "PATH")]
    pub project: Option<PathBuf>,
    /// Emit a stable, versioned JSON report to stdout
    #[clap(long)]
    pub json: bool,
    /// Treat unsupported and unknown versions as failures
    #[clap(long)]
    pub strict: bool,
}

impl DoctorCommand {
    pub fn run(&self, config: &Config) -> Result<()> {
        let platforms = resolve_platforms(&self.platform).map_err(Error::DoctorPlatformDisabled)?;
        let project = self.project.as_ref().map(|path| {
            if path.is_absolute() {
                path.clone()
            } else {
                config.current_dir().join(path)
            }
        });
        let request = DoctorRequest {
            project,
            strict: self.strict,
            platforms,
            targets: Vec::new(),
        };
        let report = diagnose_current(&request);
        if self.json {
            println!(
                "{}",
                serde_json::to_string_pretty(&report).map_err(Error::DoctorReport)?
            );
        } else {
            eprintln!("Crossbundle doctor ({:?})", report.scope);
            for check in &report.checks {
                let marker = match check.status {
                    CheckStatus::Pass => "[pass]",
                    CheckStatus::Warn => "[warn]",
                    CheckStatus::Fail => "[fail]",
                    CheckStatus::Skip => "[skip]",
                };
                eprintln!("{marker} {}: {}", check.id, check.summary);
                if let Some(remediation) = &check.remediation {
                    eprintln!("       {remediation}");
                }
            }
            eprintln!(
                "Result: {:?} ({} passed, {} warnings, {} failed, {} skipped)",
                report.status,
                report.summary.pass,
                report.summary.warn,
                report.summary.fail,
                report.summary.skip
            );
        }
        if report.status == ReportStatus::Fail {
            Err(Error::DoctorFailed)
        } else {
            Ok(())
        }
    }
}
