use android_tools::java_tools::{Key, KeyAlgorithm, Keytool};
use std::path::PathBuf;

/// Generates keystore with default configuration. You can manage configuration with
/// options
pub fn gen_key(
    sign_key_path: Option<PathBuf>,
    sign_key_pass: Option<String>,
    sign_key_alias: Option<String>,
) -> crate::error::Result<Key> {
    let key = if let Some(key_path) = sign_key_path {
        let aab_key = Key {
            key_path,
            key_pass: sign_key_pass.unwrap(),
            key_alias: sign_key_alias.unwrap(),
        };
        if aab_key.key_path.exists() {
            aab_key
        } else {
            Keytool::new()
                .genkeypair(true)
                .v(true)
                .keystore(&aab_key.key_path)
                .alias(&aab_key.key_alias)
                .keypass(&aab_key.key_pass)
                .storepass(&aab_key.key_pass)
                .dname(&["CN=Android Debug,O=Android,C=US".to_owned()])
                .keyalg(KeyAlgorithm::RSA)
                .keysize(2048)
                .validity(10000)
                .run()?
                // This will never panic because of AabKey always returned if help flag not set
                .unwrap()
        }
    } else {
        let aab_key = Key::new_default()?;
        if aab_key.key_path.exists() {
            aab_key
        } else {
            Keytool::new()
                .genkeypair(true)
                .v(true)
                .keystore(&aab_key.key_path)
                .alias(&aab_key.key_alias)
                .keypass(&aab_key.key_pass)
                .storepass(&aab_key.key_pass)
                .dname(&["CN=Android Debug,O=Android,C=US".to_owned()])
                .keyalg(KeyAlgorithm::RSA)
                .keysize(2048)
                .validity(10000)
                .run()?
                // This will never panic because of AabKey always returned if help flag not set
                .unwrap()
        }
    };
    Ok(key)
}
