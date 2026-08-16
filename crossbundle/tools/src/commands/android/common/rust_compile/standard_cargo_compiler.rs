use super::cmake_env;
use crate::{error::*, types::*};
use anyhow::Context as _;
use serde_json::Value;
use std::{
    io::{BufRead as _, Write as _},
    path::{Path, PathBuf},
    process::Stdio,
};

/// Build an Android shared library through Cargo's public command-line interface.
///
/// Cargo emits machine-readable artifact messages, which lets us discover the output instead of
/// guessing its location. Keeping this boundary at the process level isolates Crossbow from
/// Cargo's unstable compiler internals.
#[allow(clippy::too_many_arguments)]
pub fn standard_cargo_compile(
    ndk: &AndroidNdk,
    build_target: AndroidTarget,
    manifest_path: &Path,
    package_name: &str,
    library_target_name: &str,
    profile: Profile,
    features: &[String],
    all_features: bool,
    no_default_features: bool,
    min_sdk_version: u32,
    target_dir: &Path,
) -> Result<PathBuf> {
    let triple = build_target.rust_triple();
    let (clang, clang_pp) = ndk.clang(build_target, min_sdk_version)?;
    let ar = ndk.toolchain_bin("ar", build_target)?;
    let clang_target = format!(
        "--target={}{}",
        build_target.ndk_llvm_triple(),
        min_sdk_version
    );

    let mut cargo = std::process::Command::new("cargo");
    append_cargo_arguments(
        &mut cargo,
        manifest_path,
        package_name,
        triple,
        target_dir,
        profile,
        features,
        all_features,
        no_default_features,
    );

    cargo
        .stdout(Stdio::piped())
        .stderr(Stdio::inherit())
        .env(format!("CC_{triple}"), &clang)
        .env(format!("CFLAGS_{triple}"), &clang_target)
        .env(format!("CXX_{triple}"), &clang_pp)
        .env(format!("CXXFLAGS_{triple}"), &clang_target)
        .env(format!("AR_{triple}"), &ar)
        .env(cargo_env_target_cfg("LINKER", triple), &clang)
        .env(cargo_env_target_cfg("AR", triple), &ar)
        .env("CXXSTDLIB", "c++");

    let build_dir = target_dir.join(triple).join(profile);
    std::fs::create_dir_all(&build_dir)?;
    for (name, value) in cmake_env(build_target, ndk, min_sdk_version, &build_dir)? {
        cargo.env(name, value);
    }

    let mut child = cargo.spawn().context("failed to start Cargo")?;
    let stdout = child
        .stdout
        .take()
        .expect("Cargo stdout was configured as piped");
    let mut artifact = None;
    let expected_target = library_target_name;

    let mut read_error = None;
    for line in std::io::BufReader::new(stdout).lines() {
        let line = match line {
            Ok(line) => line,
            Err(error) => {
                read_error = Some(error);
                break;
            }
        };
        let Ok(message) = serde_json::from_str::<Value>(&line) else {
            eprintln!("{line}");
            continue;
        };
        if let Some(rendered) = rendered_diagnostic(&message) {
            eprint!("{rendered}");
            std::io::stderr().flush().ok();
        }
        if let Some(path) = cdylib_artifact(&message, expected_target) {
            artifact = Some(path);
        }
    }

    if read_error.is_some() {
        child.kill().ok();
    }
    let status = child.wait().context("failed to wait for Cargo")?;
    if let Some(error) = read_error {
        return Err(Error::AnyhowError(
            anyhow::Error::new(error).context("failed to read Cargo output"),
        ));
    }
    if !status.success() {
        let status_message = format!("Cargo exited with {status}");
        return Err(Error::CmdFailed(cargo, String::new(), status_message));
    }

    let artifact = artifact.ok_or_else(|| {
        Error::AnyhowError(anyhow::anyhow!(
            "Cargo did not report an Android cdylib for library target `{expected_target}`. \
             Add `[lib]\ncrate-type = [\"cdylib\", \"rlib\"]` and export the platform entry point \
             from that library."
        ))
    })?;
    if !artifact.is_file() {
        return Err(Error::PathNotFound(artifact));
    }
    Ok(artifact)
}

fn cargo_env_target_cfg(key: &str, target: &str) -> String {
    format!(
        "CARGO_TARGET_{}_{}",
        target.to_uppercase().replace('-', "_"),
        key
    )
}

#[allow(clippy::too_many_arguments)]
fn append_cargo_arguments(
    cargo: &mut std::process::Command,
    manifest_path: &Path,
    package_name: &str,
    triple: &str,
    target_dir: &Path,
    profile: Profile,
    features: &[String],
    all_features: bool,
    no_default_features: bool,
) {
    cargo
        .arg("build")
        .arg("--manifest-path")
        .arg(manifest_path)
        .arg("--package")
        .arg(package_name)
        .arg("--target")
        .arg(triple)
        .arg("--target-dir")
        .arg(target_dir)
        .arg("--message-format=json-render-diagnostics")
        .arg("--lib");
    if profile == Profile::Release {
        cargo.arg("--release");
    }
    for feature in features {
        cargo.arg("--features").arg(feature);
    }
    if all_features {
        cargo.arg("--all-features");
    }
    if no_default_features {
        cargo.arg("--no-default-features");
    }
}

fn rendered_diagnostic(message: &Value) -> Option<&str> {
    (message.get("reason").and_then(Value::as_str) == Some("compiler-message"))
        .then(|| message.pointer("/message/rendered").and_then(Value::as_str))
        .flatten()
}

fn cdylib_artifact(message: &Value, expected_target: &str) -> Option<PathBuf> {
    if message.get("reason").and_then(Value::as_str) != Some("compiler-artifact")
        || message.pointer("/target/name").and_then(Value::as_str) != Some(expected_target)
    {
        return None;
    }
    message
        .pointer("/target/crate_types")
        .and_then(Value::as_array)
        .is_some_and(|types| types.iter().any(|kind| kind.as_str() == Some("cdylib")))
        .then(|| {
            message
                .get("filenames")
                .and_then(Value::as_array)
                .into_iter()
                .flatten()
                .filter_map(Value::as_str)
                .map(PathBuf::from)
                .find(|path| path.extension().is_some_and(|ext| ext == "so"))
        })
        .flatten()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn discovers_cdylib_from_cargo_messages() {
        let message = serde_json::json!({
            "reason": "compiler-artifact",
            "target": {"name": "my_game", "crate_types": ["cdylib"]},
            "filenames": ["/tmp/target/aarch64-linux-android/debug/libmy_game.so"]
        });
        let artifact = cdylib_artifact(&message, "my_game").unwrap();
        assert_eq!(
            artifact,
            PathBuf::from("/tmp/target/aarch64-linux-android/debug/libmy_game.so")
        );
    }

    #[test]
    fn rejects_non_cdylib_artifacts() {
        let message = serde_json::json!({
            "reason": "compiler-artifact",
            "target": {"name": "my_game", "crate_types": ["rlib"]},
            "filenames": ["/tmp/libmy_game.rlib"]
        });
        assert_eq!(cdylib_artifact(&message, "my_game"), None);
    }

    #[test]
    fn extracts_human_readable_cargo_diagnostics() {
        let message = serde_json::json!({
            "reason": "compiler-message",
            "message": {"rendered": "error: useful message\n"}
        });
        assert_eq!(
            rendered_diagnostic(&message),
            Some("error: useful message\n")
        );
    }

    #[test]
    fn constructs_stable_cargo_build_command() {
        let mut command = std::process::Command::new("cargo");
        append_cargo_arguments(
            &mut command,
            Path::new("/game/Cargo.toml"),
            "my-game",
            "aarch64-linux-android",
            Path::new("/game/target"),
            Profile::Release,
            &["hot-reload".to_owned(), "bevy/png".to_owned()],
            false,
            true,
        );
        let args = command
            .get_args()
            .map(|arg| arg.to_string_lossy().into_owned())
            .collect::<Vec<_>>();
        assert_eq!(
            args,
            [
                "build",
                "--manifest-path",
                "/game/Cargo.toml",
                "--package",
                "my-game",
                "--target",
                "aarch64-linux-android",
                "--target-dir",
                "/game/target",
                "--message-format=json-render-diagnostics",
                "--lib",
                "--release",
                "--features",
                "hot-reload",
                "--features",
                "bevy/png",
                "--no-default-features",
            ]
        );
    }
}
