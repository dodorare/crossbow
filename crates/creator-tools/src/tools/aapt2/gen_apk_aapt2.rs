use crate::error::*;
use crate::tools::*;
// use chrono::DateTime;
// use chrono::Local;
use std::fs;
use std::path::{Path, PathBuf};

pub fn gen_aapt2_apk(path_to_input_files: &[PathBuf], output_directory: &Path) -> Result<()> {
    if !output_directory.exists() {
        Aapt2Compile::new(path_to_input_files, output_directory)
            .output_text_symbols("text.txt".to_string())
            .run();
    }
    let metadata = fs::metadata(output_directory)?;
    let created = metadata.created()?;
    let modified = metadata.modified()?;
    if modified > created {
        Aapt2Compile::new(path_to_input_files, output_directory)
            .output_text_symbols("text.txt".to_string())
            .run();
    }
    Ok(())
}

pub fn aapt2_link(
    sdk: &AndroidSdk,
    inputs: &[PathBuf],
    o: &Path,
    manifest: &Path,
    target_sdk_version: u32,
) -> Result<()> {
    Aapt2Link::new(inputs, o, manifest)
        .i(sdk.android_jar(target_sdk_version)?)
        .run()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn aapt2_compile_test() {
        let _aapt2_compile = gen_aapt2_apk(
            &[Path::new("D:\\programing\\work\\creator-rs\\creator\\examples\\3d\\res\\android\\mipmap-xxhdpi\\ic_launcher.png").to_owned()],
                Path::new("D:\\programing\\work\\creator-rs\\creator\\examples\\3d\\res\\android\\mipmap-xxhdpi")
    );
    }

    fn aapt2_link_test() {
        let sdk = AndroidSdk::from_env().unwrap();
        let _aapt2_link_test = aapt2_link(
            &sdk,
            &[Path::new("D:\\programing\\work\\creator-rs\\creator\\examples\\3d\\res\\android\\mipmap-xxhdpi\\mipmap-xxhdpi_ic_launcher.png.flat").to_owned()],
                  Path::new("D:\\programing\\work\\creator-rs\\creator\\examples\\3d\\res\\android\\mipmap-xxhdpi\\Test_apk.apk"),
            Path::new("D:\\programing\\work\\creator-rs\\creator\\examples\\3d\\res\\android\\mipmap-xxhdpi\\AndroidManifest.xml"),
            30
        );
    }
    #[test]
    fn main() {
        aapt2_compile_test();
        aapt2_link_test();
    }
}
