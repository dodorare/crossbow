use crate::error::*;
use crate::tools::*;
use std::fs;
use std::path::{Path, PathBuf};

pub fn gen_aapt2_apk(
    inputs_compile: &[PathBuf],
    o_compile: &Path,
    sdk: &AndroidSdk,
    inputs_link: &[PathBuf],
    o_link: &Path,
    manifest: &Path,
    target_sdk_version: u32,
) -> Result<()> {
    // Aapt2Compile::new(inputs_compile, o_compile).run();
    let changed_conpile = Vec::new();
    // let metadata = fs::metadata(&)?;
    if !changed_conpile.is_empty() {
        Aapt2Compile::new(&changed_conpile, o_compile).run()?;
    }
    Aapt2Link::new(inputs_link, o_link, manifest)
        .i(sdk.android_jar(target_sdk_version)?)
        .run()?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn aapt2_compile_test() {
        let sdk = AndroidSdk::from_env().unwrap();
        let _aapt2_compile = gen_aapt2_apk(
            &[Path::new("res\\mipmap\\Screenshot_2.png").to_owned()],
            Path::new("res\\mipmap\\"),
            &sdk,
            &[Path::new("res\\mipmap\\mipmap_Screenshot_2.png.flat").to_owned()],
            Path::new("res\\mipmap\\test.apk"),
            Path::new("res\\mipmap\\AndroidManifest.xml"),
            30,
        );
    }
}
