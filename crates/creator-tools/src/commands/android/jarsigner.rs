use crate::error::*;
use std::path::{Path, PathBuf};
use std::process::Command;

pub fn jarsigner(
    aab_path: &Path,
    key_path: Option<PathBuf>,
    key_pass: Option<String>,
    key_alias: Option<String>,
    android_build_dir: PathBuf,
) -> Result<()> {
    let mut jarsigner = jarsigner_tool()?;
    jarsigner
        .arg("-verbose")
        .arg("-sigalg")
        .arg("SHA256withRSA")
        .arg("-digestalg")
        .arg("SHA-256")
        .arg(aab_path);
    if let Some(key_path) = &key_path {
        jarsigner.arg("-keystore").arg(&key_path);
    } else {
        log::debug!("Using default keystore for generating aab key");
        let path = android_build_dir.join("aab.keystore");
        jarsigner.arg("-keystore").arg(&path);
    }
    if let Some(key_pass) = &key_pass {
        jarsigner.arg("-storepass").arg(&key_pass);
    } else {
        log::debug!("Using default key password for generating aab key");
        let password = "android".to_string();
        jarsigner.arg("-storepass").arg(&password);
    }
    if let Some(key_alias) = key_alias {
        jarsigner.arg(&key_alias);
    } else {
        log::debug!("Using default key alias for generating aab key");
        let alias = "androiddebugkey".to_string();
        jarsigner.arg(&alias);
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

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test_command_run() {
//         // TODO: Fix this test
//         jarsigner(
//             Path::new("C:\\Users\\den99\\Desktop\\Work\\DodoRare\\creator\\target\\android\\debug\\threed_unsigned.aab"),
//             // Some("android".to_string()),
//             // Path::new("C:\\Users\\den99\\Desktop\\Work\\DodoRare\\creator\\target\\android\\debug\\threed_unsigned.aab"),
//             // Some("androiddebugkey".to_string()),
//             None,
//             None,
//             None,
//         )
//         .unwrap();
//     }
// }
