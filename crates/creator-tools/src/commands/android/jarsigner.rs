use crate::commands::android::android_dir;
use crate::error::*;
use std::path::{Path, PathBuf};
use std::process::Command;

use super::AabKey;

pub fn jarsigner(aab_path: &Path, key: &AabKey) -> Result<()> {
    let mut jarsigner = jarsigner_tool()?;
    // let path = android_dir()?.join("aab.keystore");
    // let password = "android".to_string();
    // let alias = "androiddebugkey".to_string();
    jarsigner
        .arg("-verbose")
        .arg("-sigalg")
        .arg("SHA256withRSA")
        .arg("-digestalg")
        .arg("SHA-256")
        .arg(aab_path)
        .arg("-keystore")
        .arg(&key.key_path)
        // .arg("-keystore")
        // .arg(&path)
        .arg("-storepass")
        .arg(&key.key_pass)
        // .arg("-storepass")
        // .arg(&password)
        .arg(&key.key_alias);
    // .arg(&alias);
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
