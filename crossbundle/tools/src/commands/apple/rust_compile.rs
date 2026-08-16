use crate::{commands::CargoBuild, error::*};
use std::{borrow::Cow, path::PathBuf, process::Command};

/// Build the executable that will be placed in an iOS application bundle.
pub fn compile_ios_executable(
    build: CargoBuild<'_>,
    deployment_target: Option<&str>,
) -> Result<PathBuf> {
    let target_name = build.target.name().to_owned();
    let deployment_target = match deployment_target {
        Some(version) => Cow::Borrowed(version),
        None => Cow::Owned(rustc_deployment_target(build.target_triple)?),
    };
    let artifact = build.run(|cargo| {
        cargo.env("IPHONEOS_DEPLOYMENT_TARGET", deployment_target.as_ref());
    })?;
    let executable = artifact.executable.ok_or_else(|| {
        anyhow::anyhow!(
            "Cargo target `{target_name}` did not produce an executable. iOS application bundles require a binary or executable example target."
        )
    })?;
    if executable.is_file() {
        Ok(executable)
    } else {
        Err(Error::PathNotFound(executable))
    }
}

fn rustc_deployment_target(target: &str) -> Result<String> {
    // Rust's minimum differs by Apple target and may change with the toolchain.
    let mut rustc = Command::new(std::env::var_os("RUSTC").unwrap_or_else(|| "rustc".into()));
    rustc.args(["--print", "deployment-target", "--target", target]);
    let output = rustc.output()?;
    if !output.status.success() {
        return Err(Error::CmdFailed(
            rustc,
            String::from_utf8_lossy(&output.stdout).into_owned(),
            String::from_utf8_lossy(&output.stderr).into_owned(),
        ));
    }
    String::from_utf8_lossy(&output.stdout)
        .trim()
        .strip_prefix("IPHONEOS_DEPLOYMENT_TARGET=")
        .map(str::to_owned)
        .ok_or_else(|| anyhow::anyhow!("rustc did not report an iOS deployment target").into())
}
