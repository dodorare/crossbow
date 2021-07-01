use crate::error::*;
use crate::tools::*;
use std::fs::create_dir_all;
use std::path::{Path, PathBuf};

pub fn aapt2_compile(
    sdk: &AndroidSdk,
    project_path: &Path,
    input: &Path,
    output_directory: &Path,
    build_dir: &Path,
    // assets: Option<PathBuf>,
    // res: Option<PathBuf>,
    package_label: String,
) -> Result<PathBuf> {
    if !build_dir.exists() {
        create_dir_all(&build_dir)?;
    }
    let apk_path = build_dir.join(format!("{}", package_label));
    let mut aapt2_compile = sdk.build_tool(bin!("aapt2"), Some(project_path))?;
    aapt2_compile
        .arg("compile")
        .arg(input)
        .arg("-o")
        .arg(output_directory);
    // if let Some(res) = &res {
    //     aapt.arg("-S").arg(dunce::simplified(res));
    // }
    // if let Some(assets) = &assets {
    //     aapt.arg("-A").arg(dunce::simplified(assets));
    // }
    aapt2_compile.output_err(true)?;
    Ok(apk_path)
}

fn aapt2_link(
    sdk: &AndroidSdk,
    project_path: &Path,
    input: &Path,
    output_apk: &Path,
    output_filename: &Path,
    build_dir: &Path,
    // assets: Option<PathBuf>,
    // res: Option<PathBuf>,
    package_label: String,
    target_sdk_version: u32,
) -> Result<PathBuf> {
    let apk_path = build_dir.join(format!("{}", package_label));
    let mut aapt2_link = sdk.build_tool(bin!("aapt2"), Some(project_path))?;
    aapt2_link
        .arg("link")
        .arg("-o")
        .arg(input)
        .arg(output_apk)
        .arg("-I")
        .arg(sdk.android_jar(target_sdk_version)?)
        .arg(output_filename);
    aapt2_link.output_err(true)?;
    Ok(apk_path)
}

// aapt2 compile C:\Users\den99\Desktop\Work\DodoRare\creator\examples\3d\res\android\mipmap-hdpi\ic_launcher.png -o C:\Users\den99\Desktop\Work\DodoRare\creator\examples\3d\res\android\mipmap-hdpi\ -I

// aapt2 link -o project/apk/unsigned_app.apk -I sdk/platforms/android-28/android.jar --manifest project/src/AndroidManifest.xml
// -R project/compiled_res/*.flat --java project/src --auto-add-overlay
