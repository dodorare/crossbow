use crate::commands::android::{extract_apk, write_zip};
use crate::error::*;
use crate::tools::*;
use std::path::{Path, PathBuf};

pub fn gen_aab(
    inputs_compile: &[PathBuf],
    o_compile: &Path,
    sdk: &AndroidSdk,
    inputs_link: &[PathBuf],
    o_link: &Path,
    manifest: &Path,
    target_sdk_version: u32,
    modules: &[PathBuf],
    save_aab: &Path,
    extracted_apk: &Path,
    zip_path: &Path,
) -> Result<()> {
    Aapt2Compile::new(inputs_compile, o_compile).run()?;

    Aapt2Link::new(inputs_link, o_link, manifest)
        .i(sdk.android_jar(target_sdk_version)?)
        .proto_format(true)
        .auto_add_overlay(true)
        .run()?;

    extract_apk::extract_apk(o_link, extracted_apk).unwrap();

    write_zip::dirs_to_write(&extracted_apk.to_owned())?;
    write_zip::write(&extracted_apk.to_owned(), zip_path).unwrap();

    BuildBundle::new(modules, save_aab).run()?;
    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test() {
        let sdk = AndroidSdk::from_env().unwrap();
        gen_aab(
            &[Path::new("res\\mipmap\\Screenshot_2.png").to_owned()],
            Path::new("res\\mipmap\\"),
            &sdk,
            &[Path::new("res\\mipmap\\mipmap_Screenshot_2.png.flat").to_owned()],
            Path::new("res\\mipmap\\test.apk"),
            Path::new("res\\mipmap\\AndroidManifest.xml"),
            30,
            &[Path::new("res\\test\\base.zip").to_owned()],
            Path::new("res\\mipmap\\test.aab"),
            Path::new("res\\extracted_files\\"),
            Path::new("res\\test\\base.zip"),
        )
        .unwrap();
    }
}
