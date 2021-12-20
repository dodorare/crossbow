use super::AabKey;
use crate::error::*;
use std::path::{Path, PathBuf};
use std::process::Command;

/// Sign aab with key
pub fn jarsigner(aab_path: &Path, key: &AabKey) -> Result<PathBuf> {
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
    Ok(aab_path.to_path_buf())
}

/// The `-verify` option can take zero or more keystore alias names after the JAR file
/// name. When the `-verify` option is specified, the `jarsigner` command checks that the
/// certificate used to verify each signed entry in the JAR file matches one of the
/// keystore aliases. The aliases are defined in the keystore specified by `-keystore` or
/// the default keystore
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        commands::android::{gen_aab_key, gen_minimal_unsigned_aab, remove},
        tools::AndroidSdk,
    };

    use gen_aab_key::android_dir;

    #[test]
    fn test_jarsigner() {
        // Creates a temporary directory
        let tempdir = tempfile::tempdir().unwrap();
        let aab_build_dir = tempdir.path();

        // Assigns configuration for aab generation
        let sdk = AndroidSdk::from_env().unwrap();
        let package_name = "minimal_unsigned_aab";
        let target_sdk_version = 30;

        // Generates minimal unsigned aab
        let aab_path =
            gen_minimal_unsigned_aab(sdk, package_name, target_sdk_version, aab_build_dir).unwrap();

        // Removes old keystore if it exists
        let android_dir = android_dir().unwrap();
        let target = vec![android_dir.join("aab.keystore")];
        remove(target).unwrap();

        // Creates new keystore to sign aab
        let aab_key = AabKey::default();
        let key_path = gen_aab_key(aab_key).unwrap();

        // Signs aab with key
        jarsigner(&aab_path, &key_path).unwrap();
    }
}
