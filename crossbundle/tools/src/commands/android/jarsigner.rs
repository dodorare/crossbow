use super::AabKey;
use crate::error::*;
use std::path::{Path, PathBuf};
use std::process::Command;

/// Sign aab with key
pub fn jarsigner(aab_path: &Path, key: &AabKey) -> Result<()> {
    let mut jarsigner = jarsigner_tool()?;
    jarsigner
        .arg("-verbose")
        .arg("-sigalg")
        .arg("SHA256withRSA")
        .arg("-digestalg")
        .arg("SHA-256")
        .arg(aab_path)
        .arg("-keystore")
        .arg(&key.key_path)
        .arg("-storepass")
        .arg(&key.key_pass)
        .arg(&key.key_alias);
    jarsigner.output_err(true)?;
    Ok(())
}

/// The `-verify` option can take zero or more keystore alias names after the JAR file name.
/// When the `-verify` option is specified, the `jarsigner` command checks that the certificate
/// used to verify each signed entry in the JAR file matches one of the keystore aliases.
/// The aliases are defined in the keystore specified by `-keystore` or the default keystore
pub fn verify_aab(aab_path: &Path) -> Result<()> {
    let mut verify = jarsigner_tool()?;
    verify.arg("-verify").arg("-verbose").arg(aab_path);
    verify.output_err(true)?;
    Ok(())
}

/// Signs and verifies `.aab` and Java Archive (JAR) files
fn jarsigner_tool() -> Result<Command> {
    if let Ok(jarsigner) = which::which(bin!("jarsigner")) {
        return Ok(Command::new(jarsigner));
    }
    if let Ok(java) = std::env::var("JAVA_HOME") {
        let keytool = PathBuf::from(java).join("bin").join(bin!("jarsigner.exe"));
        if keytool.exists() {
            return Ok(Command::new(keytool));
        }
    }
    Err(Error::CmdNotFound("jarsigner".to_string()))
}
