use crate::error::*;
use std::path::PathBuf;
use std::process::Command;

/// Generates debug key for signing APK.
/// Runs `keytool ...` command.
pub fn gen_aab_key(key: AabKey) -> Result<AabKey> {
    let mut keytool = keytool()?;
    keytool.arg("-genkey").arg("-v");
    if key.key_path.is_dir() {
        keytool
            .arg("-keystore")
            .arg(key.key_path.join("aab.keystore"));
    } else {
        keytool.arg("-keystore").arg(&key.key_path);
    }
    keytool.arg("-alias").arg(&key.key_alias);
    keytool.arg("-keypass").arg(&key.key_pass);
    keytool.arg("-storepass").arg(&key.key_pass);
    keytool
        .arg("-dname")
        .arg("CN=Android Debug,O=Android,C=US")
        .arg("-keyalg")
        .arg("RSA")
        .arg("-keysize")
        .arg("2048")
        .arg("-validity")
        .arg("10000");
    keytool.output_err(true)?;
    Ok(key)
}

#[derive(Debug, Clone)]
pub struct AabKey {
    pub key_path: PathBuf,
    pub key_pass: String,
    pub key_alias: String,
}

impl Default for AabKey {
    fn default() -> Self {
        let key_path = android_dir().unwrap().join("aab.keystore");
        let key_pass = "android".to_string();
        let key_alias = "androidaabkey".to_string();
        Self {
            key_path,
            key_pass,
            key_alias,
        }
    }
}

pub fn android_dir() -> Result<PathBuf> {
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
