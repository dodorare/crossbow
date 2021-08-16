use crate::error::*;
use std::path::{Path, PathBuf};
use std::process::Command;

pub fn gen_debug_key_aab(keystore_path: &Path, alias: String) -> Result<()> {
    let path = keystore_path.join("keystore");
    if !path.exists() {
        let mut keytool = keytool()?;
        keytool
            .arg("-genkey")
            .arg("-v")
            .arg("-keystore")
            .arg(&path)
            .arg("-keyalg")
            .arg("RSA")
            .arg("-keysize")
            .arg("2048")
            .arg("-validity")
            .arg("10000")
            .arg("-alias")
            .arg(alias);
        keytool.output_err(true)?;
    }
    Ok(())
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

    fn test() {
        gen_debug_key_aab(Path::new("res\\mipmap"), "devtool".to_string()).unwrap();
    }
}
