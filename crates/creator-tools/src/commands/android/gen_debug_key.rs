use super::Key;
use crate::error::*;
use std::path::PathBuf;
use std::process::Command;

/// Generates debug key for signing APK.
/// Runs `keytool ...` command.
pub fn gen_debug_key() -> Result<Key> {
    let path = android_dir()?.join("debug.keystore");
    let password = "android".to_string();
    if !path.exists() {
        let mut keytool = keytool()?;
        keytool
            .arg("-genkey")
            .arg("-v")
            .arg("-keystore")
            .arg(&path)
            .arg("-storepass")
            .arg("android")
            .arg("-alias")
            .arg("androiddebugkey")
            .arg("-keypass")
            .arg(&password)
            .arg("-dname")
            .arg("CN=Android Debug,O=Android,C=US")
            .arg("-keyalg")
            .arg("RSA")
            .arg("-keysize")
            .arg("2048")
            .arg("-validity")
            .arg("10000");
        keytool.output_err(true)?;
    }
    Ok(Key { path, password })
}

fn android_dir() -> Result<PathBuf> {
    let android_dir = dirs::home_dir()
        .ok_or_else(|| Error::PathNotFound(PathBuf::from("$HOME")))?
        .join(".android");
    std::fs::create_dir_all(&android_dir)?;
    Ok(android_dir)
}
fn keytool() -> Result<Command> {
    if let Ok(keytool) = which::which(bin!("keytool")) {
        return Ok(Command::new(keytool));
    }
    if let Ok(java) = std::env::var("JAVA_HOME") {
        let keytool = PathBuf::from(java).join("bin").join(bin!("keytool"));
        if keytool.exists() {
            return Ok(Command::new(keytool));
        }
    }
    Err(Error::CmdNotFound("keytool".to_string()))
}
