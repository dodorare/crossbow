use super::{android_dir, keytool, Key};
use crate::error::*;

/// Generates debug key for signing APK.
/// Runs `keytool ...` command
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
