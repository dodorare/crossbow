use crate::error::*;
use crate::tools::*;
use std::path::{Path, PathBuf};

/// Aligns APK on 4-byte memory boundary.
/// Uses `zipalign` build tools
pub fn align_apk(
    sdk: &AndroidSdk,
    unaligned_apk_path: &Path,
    package_name: &str,
    build_dir: &Path,
) -> Result<PathBuf> {
    let unsigned_apk_path = build_dir.join(format!("{}.apk", package_name));
    let mut zipalign = sdk.build_tool(bin!("zipalign"), None)?;
    zipalign
        .arg("-f")
        .arg("-v")
        .arg("4")
        .arg(unaligned_apk_path)
        .arg(&unsigned_apk_path);
    zipalign.output_err(true)?;
    Ok(unsigned_apk_path)
}
