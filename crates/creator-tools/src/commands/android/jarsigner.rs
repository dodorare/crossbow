use crate::error::*;
use std::path::{Path, PathBuf};
use std::process::Command;

use super::android_dir;

pub fn jarsigner(
    key: Option<String>,
    keystore_path: Option<PathBuf>,
    aab_path: &Path,
    alias: Option<String>,
) -> Result<()> {
    let mut jarsigner = jarsigner_tool()?;
    let path = android_dir()?.join("aab.keystore");
    let password = "android".to_string();
    let aab_alias = "androiddebugkey".to_string();
    jarsigner
        .arg("-verbose")
        .arg("-sigalg")
        .arg("SHA256withRSA")
        .arg("-digestalg")
        .arg("SHA-256")
        .arg(aab_path);
    if let Some(keystore_path) = keystore_path {
        jarsigner.arg("-keystore").arg(keystore_path);
    } else {
        jarsigner.arg("-keystore").arg(&path);
    }
    if let Some(key) = key {
        jarsigner.arg("-storepass").arg(key);
    } else {
        jarsigner.arg("-storepass").arg(&password);
    }
    if let Some(alias) = alias {
        jarsigner.arg(alias);
    } else {
        jarsigner.arg(&aab_alias);
    }
    jarsigner.output_err(true)?;
    Ok(())
}

pub fn verify_aab(aab_path: &Path) -> Result<()> {
    let mut verify = jarsigner_tool()?;
    verify.arg("-verify").arg("-verbose").arg(aab_path);
    verify.output_err(true)?;
    Ok(())
}

fn jarsigner_tool() -> Result<Command> {
    if let Ok(jarsigner) = which::which(bin!("jarsigner")) {
        return Ok(Command::new(jarsigner));
    }
    if let Ok(java) = std::env::var("JAVA_HOME") {
        let keytool = PathBuf::from(java).join("bin").join(bin!("jarsigner"));
        if keytool.exists() {
            return Ok(Command::new(keytool));
        }
    }
    Err(Error::CmdNotFound("jarsigner".to_string()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_command_run() {
        jarsigner(
            // Some("android".to_string()),
            None,
            // Path::new("C:\\Users\\den99\\Desktop\\Work\\DodoRare\\creator\\target\\android\\debug\\threed_unsigned.aab"),
            // Some("androiddebugkey".to_string()),
            None,
            Path::new("C:\\Users\\den99\\Desktop\\Work\\DodoRare\\creator\\target\\android\\debug\\threed_unsigned.aab"),
            None,
        )
        .unwrap();
    }
}
