use cargo_creator::{ApkBuilder, Error, Subcommand};
use std::process::Command;

fn main() -> anyhow::Result<()> {
    env_logger::init();

    let cmd = Subcommand::new("creator", |_, _| Ok(false))?;
    let builder = ApkBuilder::from_subcommand(&cmd)?;

    match cmd.cmd() {
        "build" => {
            for artifact in cmd.artifacts() {
                builder.build(artifact)?;
            }
        }
        "run" => {
            anyhow::ensure!(cmd.artifacts().len() == 1, Error::InvalidArgs);
            builder.run(&cmd.artifacts()[0])?;
        }
        "--" => {
            builder.default()?;
        }
        "gdb" => {
            anyhow::ensure!(cmd.artifacts().len() == 1, Error::InvalidArgs);
            builder.gdb(&cmd.artifacts()[0])?;
        }
        "help" => {
            if let Some(arg) = cmd.args().get(0) {
                match &**arg {
                    "build" | "run" | "test" | "doc" => run_cargo(&cmd)?,
                    "gdb" => print_gdb_help(),
                    _ => print_help(),
                }
            } else {
                print_help();
            }
        }
        _ => print_help(),
    }

    Ok(())
}

fn run_cargo(cmd: &Subcommand) -> Result<(), Error> {
    Command::new("cargo")
        .arg(cmd.cmd())
        .args(cmd.args())
        .status()?;
    Ok(())
}

fn print_help() {
    println!(
        r#"cargo-creator
Helps cargo build apk's for android
USAGE:
    cargo creator [SUBCOMMAND]
SUBCOMMAND:
    build   Compiles the current package
    run     Run a binary or example of the local package
    gdb     Start a gdb session attached to an adb device with symbols loaded
"#
    );
}

fn print_gdb_help() {
    println!(
        r#"cargo-creator gdb
Start a gdb session attached to an adb device with symbols loaded
USAGE:
    cargo creator gdb
"#
    );
}
