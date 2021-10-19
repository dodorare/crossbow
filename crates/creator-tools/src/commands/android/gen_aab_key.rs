use crate::error::*;
use std::path::PathBuf;
use std::process::Command;

/// Generates debug key for signing APK.
/// Runs `keytool ...` command.
pub fn gen_aab_key(
    key_path: Option<PathBuf>,
    key_pass: Option<String>,
    key_alias: Option<String>,
) -> Result<AabKey> {
    let mut keytool = keytool()?;
    keytool
        .arg("-genkey")
        .arg("-v")
        .arg("-dname")
        .arg("CN=Android Debug,O=Android,C=US")
        .arg("-keyalg")
        .arg("RSA")
        .arg("-keysize")
        .arg("2048")
        .arg("-validity")
        .arg("10000");
    if let Some(key_path) = &key_path {
        keytool.arg("-keystore").arg(key_path);
    } else {
        log::debug!("Using default keystore for generating aab key");
        let path = android_dir()?.join("aab.keystore");
        keytool.arg("-keystore").arg(&path);
    }
    if let Some(key_pass) = &key_pass {
        keytool.arg("-storepass").arg(&key_pass);
        keytool.arg("-keypass").arg(key_pass);
    } else {
        log::debug!("Using default key password for generating aab key");
        let password = "android".to_string();
        keytool.arg("-storepass").arg(&password);
        keytool.arg("-keypass").arg(&password);
    }
    if let Some(key_alias) = &key_alias {
        keytool.arg("-alias").arg(key_alias);
    } else {
        log::debug!("Using default key alias for generating aab key");
        let alias = "androiddebugkey".to_string();
        keytool.arg("-alias").arg(alias);
    }
    keytool.output_err(true)?;
    Ok(AabKey {
        key_path,
        key_pass,
        key_alias,
    })
}

#[derive(Debug, Clone)]
pub struct AabKey {
    pub key_path: Option<PathBuf>,
    pub key_pass: Option<String>,
    pub key_alias: Option<String>,
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

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn aab_key_test() {
        gen_aab_key(
            // TODO: Fix this test with absolute paths
            //             Some(std::path::Path::new("target").("android").join("debug").join("test_aab_keystore").to_absolute()),
            //     Some("dodorare".to_string()),
            // Some("devtools".to_string())).unwrap();
            None, None, None,
        )
        .unwrap();
    }
}
