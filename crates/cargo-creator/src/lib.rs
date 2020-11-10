mod apk;
mod error;
mod manifest;
mod ndk;
mod subcommand;

pub use apk::ApkBuilder;
pub use error::Error;
pub use ndk::*;
pub use subcommand::*;

static VERSION: &str = env!("CARGO_PKG_VERSION");
static DESCRIPTION: &str = env!("CARGO_PKG_DESCRIPTION");

pub fn cli_run(name: &str) -> anyhow::Result<()> {
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
                    "gdb" => print_gdb_help(name),
                    _ => print_help(name),
                }
            } else {
                print_help(name);
            }
        }
        _ => print_help(name),
    }

    Ok(())
}

fn run_cargo(cmd: &Subcommand) -> Result<(), Error> {
    std::process::Command::new("cargo")
        .arg(cmd.cmd())
        .args(cmd.args())
        .status()?;
    Ok(())
}

fn print_help(name: &str) {
    println!(
        r#"{} {}
{}
USAGE:
    {} [SUBCOMMAND]
SUBCOMMAND:
    build   Compiles the current package
    run     Run a binary or example of the local package
    gdb     Start a gdb session attached to an adb device with symbols loaded
"#,
        name,
        VERSION,
        DESCRIPTION,
        name.replace("-", " "),
    );
}

fn print_gdb_help(name: &str) {
    println!(
        r#"{} gdb
Start a gdb session attached to an adb device with symbols loaded
USAGE:
    {} gdb
"#,
        name,
        name.replace("-", " "),
    );
}
