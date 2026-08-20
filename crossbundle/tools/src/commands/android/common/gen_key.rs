use android_tools::java_tools::{Key, KeyAlgorithm, Keytool};
use std::path::PathBuf;

/// Generates keystore with default configuration. You can manage configuration with
/// options
pub fn gen_key(
    sign_key_path: Option<PathBuf>,
    sign_key_pass: Option<String>,
    sign_key_alias: Option<String>,
) -> crate::error::Result<Key> {
    let key = match (sign_key_path, sign_key_pass, sign_key_alias) {
        (Some(key_path), Some(key_pass), Some(key_alias)) => Key {
            key_path,
            key_pass,
            key_alias,
        },
        (Some(_), _, _) => {
            return Err(
                anyhow::anyhow!("a signing key requires both a password and an alias").into(),
            );
        }
        (None, _, _) => Key::new_default()?,
    };
    if key.key_path.exists() {
        return Ok(key);
    }
    Keytool::new()
        .genkeypair(true)
        .v(true)
        .keystore(&key.key_path)
        .alias(&key.key_alias)
        .keypass(&key.key_pass)
        .storepass(&key.key_pass)
        .dname(&["CN=Android Debug,O=Android,C=US".to_owned()])
        .keyalg(KeyAlgorithm::RSA)
        .keysize(2048)
        .validity(10000)
        .run()?
        .ok_or_else(|| anyhow::anyhow!("keytool did not return the generated signing key").into())
}
