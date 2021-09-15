use crate::error::*;
use std::path::{Path, PathBuf};
use std::process::Command;

pub fn jarsigner(key: String, keystore_path: &Path, aab_path: &Path, alias: String) -> Result<()> {
    let mut jarsigner = jarsigner_tool()?;
    jarsigner
        .arg("-verbose")
        .arg("-sigalg")
        .arg("SHA256withRSA")
        .arg("-digestalg")
        .arg("SHA-256")
        .arg("-keystore")
        .arg(keystore_path)
        .arg("-storepass")
        .arg(key)
        .arg(aab_path)
        .arg(alias);
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
            "android".to_string(),
            Path::new("C:\\Users\\den99\\.android\\debug.keystore"),
            Path::new("C:\\Users\\den99\\Desktop\\Work\\DodoRare\\creator\\target\\android\\debug\\threed_unsigned.aab"),
            "androiddebugkey".to_string(),
        ).unwrap();
    }
}
