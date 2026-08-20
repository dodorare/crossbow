#[macro_use]
extern crate log;

pub mod commands;
pub mod error;
pub mod types;

use clap::{ArgAction, Parser};
use colored::Colorize;
use commands::*;
use crossbundle_tools::types::{CliContext, Shell, Verbosity};
use std::path::PathBuf;

#[derive(Parser, Clone, Debug)]
#[clap(author, about, version)]
pub struct Opts {
    /// The current directory where to run all commands
    #[clap(short, long)]
    pub current_dir: Option<PathBuf>,
    /// A level of verbosity, and can be used multiple times
    #[clap(short, long, action = ArgAction::Count)]
    pub verbose: u8,
    /// No output printed to stdout
    #[clap(short, long)]
    pub quiet: bool,

    #[clap(subcommand)]
    pub cmd: Commands,
}

impl Opts {
    pub fn get_verbosity(&self) -> Verbosity {
        if self.quiet {
            Verbosity::Quiet
        } else {
            // Vary the output based on how many times the user used the "verbose" flag.
            // Example: `crossbundle -v -v -v' or 'crossbundle -vvv' vs 'crossbundle -v'
            match self.verbose {
                0 => Verbosity::Normal,
                1 => Verbosity::Verbose,
                _ => {
                    pretty_env_logger::formatted_builder()
                        .filter_level(log::LevelFilter::Trace)
                        .init();
                    Verbosity::Verbose
                }
            }
        }
    }

    pub fn get_current_dir(&self) -> std::io::Result<PathBuf> {
        self.current_dir
            .clone()
            .map_or_else(std::env::current_dir, Ok)
    }
}

pub fn run() -> std::result::Result<(), Box<dyn std::error::Error>> {
    let opts = Opts::parse();
    let mut shell = Shell::new();
    shell.set_verbosity(opts.get_verbosity());
    let context = CliContext::new(shell, opts.get_current_dir()?);
    opts.cmd.handle_command(&context)?;
    Ok(())
}

pub fn handle_errors(run: impl FnOnce() -> std::result::Result<(), Box<dyn std::error::Error>>) {
    if let Err(error) = run() {
        eprintln!("{}: {}", "error".red().bold(), error);
        handle_error_source(error.source());
        let exit_code = error
            .downcast_ref::<crate::error::Error>()
            .map_or(1, crate::error::Error::exit_code);
        std::process::exit(exit_code);
    };
}

fn handle_error_source(source: Option<&(dyn std::error::Error + 'static)>) {
    if let Some(error) = source {
        eprintln!("{}: {}", "caused by".red().bold(), error);
        handle_error_source(error.source());
    }
}

#[cfg(test)]
mod tests {
    use super::Opts;
    #[cfg(any(feature = "android", feature = "apple"))]
    use super::commands;
    use clap::{CommandFactory, Parser};
    #[cfg(any(feature = "android", feature = "apple"))]
    use crossbundle_tools::toolchain::{DoctorPlatform, resolve_platforms};

    #[test]
    fn parses_repeated_verbose_flags() {
        let opts = Opts::try_parse_from(["crossbundle", "-vv", "update"]).unwrap();

        assert_eq!(opts.verbose, 2);
    }

    #[test]
    fn command_line_definition_is_consistent() {
        Opts::command().debug_assert();
    }

    #[cfg(feature = "apple")]
    #[test]
    fn parses_ios_simulator_automation_options() {
        let opts = Opts::try_parse_from([
            "crossbundle",
            "run",
            "ios",
            "--simulator",
            "iPhone 17",
            "--no-open",
            "--detach",
        ])
        .unwrap();
        let commands::Commands::Run(commands::run::RunCommand::Ios(command)) = opts.cmd else {
            panic!("expected iOS run command")
        };
        assert_eq!(command.simulator.as_deref(), Some("iPhone 17"));
        assert!(command.no_open && command.detach);
    }

    #[cfg(feature = "apple")]
    #[test]
    fn rejects_simulator_options_for_physical_devices() {
        for option in ["--simulator", "--no-open", "--detach"] {
            let mut args = vec![
                "crossbundle",
                "run",
                "ios",
                "--device",
                "--profile-path=profile.mobileprovision",
                "--team-id=TEAM",
                "--signing-identity=IDENTITY",
                option,
            ];
            if option == "--simulator" {
                args.push("iPhone 17");
            }
            assert!(Opts::try_parse_from(args).is_err());
        }
        assert!(Opts::try_parse_from(["crossbundle", "run", "ios", "--device"]).is_err());
        assert!(Opts::try_parse_from(["crossbundle", "run", "ios", "--debug"]).is_err());
    }

    #[cfg(any(feature = "android", feature = "apple"))]
    #[test]
    fn parses_doctor_project_json_and_strict() {
        let opts = Opts::try_parse_from([
            "crossbundle",
            "doctor",
            "--project",
            ".",
            "--json",
            "--strict",
        ])
        .unwrap();
        let commands::Commands::Doctor(command) = opts.cmd else {
            panic!("expected doctor")
        };
        assert_eq!(command.project.unwrap(), std::path::PathBuf::from("."));
        assert!(command.json && command.strict);
    }

    #[cfg(any(feature = "android", feature = "apple"))]
    #[test]
    fn parses_repeated_and_delimited_doctor_platforms() {
        let opts = Opts::try_parse_from([
            "crossbundle",
            "doctor",
            "--platform",
            "apple,android",
            "--platform",
            "apple",
        ])
        .unwrap();
        let commands::Commands::Doctor(command) = opts.cmd else {
            panic!("expected doctor")
        };
        assert_eq!(command.platform.len(), 3);
        #[cfg(all(feature = "android", feature = "apple"))]
        assert_eq!(
            resolve_platforms(&command.platform).unwrap(),
            vec![DoctorPlatform::Android, DoctorPlatform::Apple]
        );
    }

    #[cfg(any(feature = "android", feature = "apple"))]
    #[test]
    fn parses_omitted_single_and_duplicate_doctor_platforms() {
        let omitted = Opts::try_parse_from(["crossbundle", "doctor"]).unwrap();
        let commands::Commands::Doctor(omitted) = omitted.cmd else {
            panic!("expected doctor")
        };
        assert!(omitted.platform.is_empty());

        let single = Opts::try_parse_from([
            "crossbundle",
            "doctor",
            "--platform",
            "android",
            "--platform",
            "android",
        ])
        .unwrap();
        let commands::Commands::Doctor(single) = single.cmd else {
            panic!("expected doctor")
        };
        if cfg!(feature = "android") {
            assert_eq!(
                resolve_platforms(&single.platform).unwrap(),
                vec![DoctorPlatform::Android]
            );
        } else {
            assert_eq!(
                resolve_platforms(&single.platform),
                Err(DoctorPlatform::Android)
            );
        }
    }

    #[cfg(any(feature = "android", feature = "apple"))]
    #[test]
    fn rejects_an_unknown_doctor_platform() {
        let error = Opts::try_parse_from(["crossbundle", "doctor", "--platform", "windows-phone"])
            .unwrap_err();
        assert_eq!(error.exit_code(), 2);
    }

    #[cfg(all(feature = "android", not(feature = "apple")))]
    #[test]
    fn rejects_apple_when_it_is_not_compiled() {
        assert_eq!(
            resolve_platforms(&[DoctorPlatform::Apple]),
            Err(DoctorPlatform::Apple)
        );
        assert_eq!(
            crate::error::Error::DoctorPlatformDisabled(DoctorPlatform::Apple).exit_code(),
            2
        );
    }

    #[cfg(all(feature = "apple", not(feature = "android")))]
    #[test]
    fn rejects_android_when_it_is_not_compiled() {
        assert_eq!(
            resolve_platforms(&[DoctorPlatform::Android]),
            Err(DoctorPlatform::Android)
        );
        assert_eq!(
            crate::error::Error::DoctorPlatformDisabled(DoctorPlatform::Android).exit_code(),
            2
        );
    }

    #[cfg(feature = "android")]
    #[test]
    fn parses_android_build_dry_run_json() {
        let opts = Opts::try_parse_from(["crossbundle", "build", "android", "--dry-run", "--json"])
            .unwrap();
        let commands::Commands::Build(commands::build::BuildCommand::Android(command)) = opts.cmd
        else {
            panic!("expected Android build")
        };
        assert!(command.dry_run && command.json);
    }
}
