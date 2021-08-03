use crate::error::*;
use crate::tools::*;
use std::path::{Path, PathBuf};

pub fn gen_aapt2_apk(
    inputs_compile: &[PathBuf],
    o_compile: &Path,
    sdk: &AndroidSdk,
    inputs_link: &[PathBuf],
    o_link: &Path,
    manifest: &Path,
    target_sdk_version: u32,
    extract_path: &Path,
) -> Result<()> {
    Aapt2Compile::new(inputs_compile, o_compile).run()?;
    Aapt2Link::new(inputs_link, o_link, manifest)
        .i(sdk.android_jar(target_sdk_version)?)
        .proto_format(true)
        .auto_add_overlay(true)
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
            Path::new("src\\main\\AndroidManifest.xml"),
            30,
            Path::new("res\\mipmap\\"),
        );
    }
}
// java -jar $BUNDLETOOL_PATH build-bundle  --modules=C:\\Users\\den99\\Desktop\\Work\\DodoRare\\creator\\crates\\creator-tools\\res\\mipmap\\base.zip --output=C:\\Users\\den99\\Desktop\\Work\\DodoRare\\creator\\crates\\creator-tools\\res\\mipmap\\test.aab