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
) -> Result<()> {
    Aapt2Compile::new(inputs_compile, &o_compile.to_owned()).run()?;
    Aapt2Link::new(inputs_link, o_link, manifest)
        .i(sdk.android_jar(target_sdk_version)?)
        .run()?;
    Ok(())
}
