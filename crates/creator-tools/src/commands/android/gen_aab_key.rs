use crate::error::*;
use std::path::PathBuf;
use std::process::Command;

/// Generates debug key for signing APK.
/// Runs `keytool ...` command.
pub fn gen_aab_key(key: AabKey) -> Result<AabKey> {
    // for entry in std::fs::read_dir(android_dir()?)? {
    //     let path = entry?.path();
    //     if path.ends_with("aab.keystore") {
    //         return AabKey;
    //     }
    // }
    let key_path = android_dir()?.join("aab.keystore");
    let key_pass = "android".to_string();
    let key_alias = "androidaabkey".to_string();
    let mut keytool = keytool()?;
    keytool.arg("-genkey").arg("-v");
    if key.key_path.exists() {
        keytool
            .arg("-keystore")
            .arg(key.key_path.join("aab.keystore"));
    } else {
        keytool.arg("-keystore").arg(&key_path);
    }
    if !key.key_alias.is_empty() {
        keytool.arg("-alias").arg(key.key_alias);
    } else {
        keytool.arg("-alias").arg(&key_alias);
    }
    if !key.key_pass.is_empty() {
        keytool.arg("-keypass").arg(&key.key_pass);
        keytool.arg("-storepass").arg(key.key_pass);
    } else {
        keytool.arg("-keypass").arg(&key_pass);
        keytool.arg("-storepass").arg(&key_pass);
    }
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

    Ok(AabKey {
        key_path,
        key_pass,
        key_alias,
    })
}

#[derive(Debug, Clone, Default)]
pub struct AabKey {
    pub key_path: PathBuf,
    pub key_pass: String,
    pub key_alias: String,
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
        gen_aab_key(AabKey {
            key_path: dunce::simplified(std::path::Path::new(
                "C:\\Users\\den99\\Desktop\\Work\\creator\\target\\android\\debug\\",
            ))
            .to_owned(),
            key_pass: "dodorare".to_string(),
            key_alias: "danyaaa".to_string(),
        })
        .unwrap();
    }
}

#[test]
fn remove_aab() {
    let android_build_dir = std::path::Path::new("target/android/debug");
    let package_label = "twod".to_string();
    let signed_aab = format!("{}_signed.aab", package_label);
    for entry in std::fs::read_dir(&android_build_dir).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        if path.ends_with(format!("{}_unsigned.aab", package_label)) {
            std::fs::rename(&path, &signed_aab).unwrap();
        }
    }
}
