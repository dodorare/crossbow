use crate::error::*;
use crate::tools::*;
// use chrono::DateTime;
// use chrono::Local;
use std::fs;
use std::path::{Path, PathBuf};

pub fn aapt2_compile(inputs: &[PathBuf], o: &Path) -> Result<()> {
    // let aapt2 = Aapt2Compile::new(inputs, o).run();
    let metadata = fs::metadata(o)?;
    let created = metadata.created()?;
    let modified = metadata.modified()?;
    if modified != created {
        let aapt2 = Aapt2Compile::new(inputs, o).run();
    }
    // aapt2.output_err(true)?;
    Ok(())
}

// pub fn aapt2_link(
//     aapt2: &Aapt2,
//     inputs: &Path,
//     manifest_path: &Path,
//     o: &Path,
//     flat_file: &Path,
//     build_dir: &Path,
//     assets: Option<PathBuf>,
//     package_label: String,
//     target_sdk_version: u32,
// ) -> Result<()> {
//     let mut aapt2_link = aapt2.link(&[inputs.to_path_buf()], o, manifest_path);
//     Command::new("aapt2")
//         .arg("link")
//         .arg("-o")
//         .arg(o)
//         .arg("-I")
//         .arg(aapt2.android_jar(target_sdk_version)?)
//         .arg("--manifest")
//         .arg(manifest_path)
//         .arg("-R")
//         .arg(flat_file)
//         .arg("--java")
//         .arg(project_path)
//         .arg("--auto-add-overlay");
//     if let Some(assets) = &assets {
//         aapt2_link
//             .arg("--proto-format")
//             .arg(dunce::simplified(assets));
//     }
//     aapt2_link.output_err(true)?;
//     Ok(())
// }

#[cfg(test)]
mod tests {
    use crate::commands::android;

    use super::*;

    #[test]
    fn aapt2_compile_test() {
        let aapt2_compile = aapt2_compile(
            &[Path::new("C:\\Users\\den99\\Desktop\\Work\\DodoRare\\creator\\examples\\3d\\res\\android\\mipmap-hdpi\\ic_launcher.png").to_owned(),
            Path::new("C:\\Users\\den99\\Desktop\\Work\\DodoRare\\creator\\examples\\3d\\res\\android\\mipmap-hdpi\\ic_launcher1.png").to_owned(),
            Path::new("C:\\Users\\den99\\Desktop\\Work\\DodoRare\\creator\\examples\\3d\\res\\android\\mipmap-hdpi\\ic_launcher2.png").to_owned(),
            Path::new("C:\\Users\\den99\\Desktop\\Work\\DodoRare\\creator\\examples\\3d\\res\\android\\mipmap-hdpi\\ic_launcher3.png").to_owned()],
         Path::new("C:\\Users\\den99\\Desktop\\Work\\DodoRare\\creator\\examples\\3d\\res\\android\\mipmap-hdpi\\")
    );
    }
}

//     #[test]
//     fn aapt2_link_test() {
//         let sdk = AndroidSdk::from_env().unwrap();
//         let package_label = "test";
//         let target_sdk_version = 30;
//         let manifest = android::gen_minimal_android_manifest(
//             &package_label.to_string(),
//             None,
//             "0.0.1".to_string(),
//             None,
//             None,
//             target_sdk_version,
//             None,
//             None,
//         );
//         let manifest_path = android::save_android_manifest(Path::new("C:\\Users\\den99\\Desktop\\Work\\DodoRare\\creator\\examples\\3d\\res\\android\\mipmap-xxhdpi\\"), &manifest).unwrap();
//         assert!(manifest_path.exists());
//         let _aapt2_link = aapt2_link(
//             &sdk,
//             Path::new("C:\\Users\\den99\\Desktop\\Work\\DodoRare\\creator\\examples\\3d\\res\\android\\mipmap-xxhdpi\\"),
//             &manifest_path,
//             Path::new("C:\\Users\\den99\\Desktop\\Work\\DodoRare\\creator\\examples\\3d\\res\\android\\mipmap-xxhdpi\\test.apk"),
//             Path::new("C:\\Users\\den99\\Desktop\\Work\\DodoRare\\creator\\examples\\3d\\res\\android\\mipmap-xxhdpi\\mipmap-xxhdpi_ic_launcher.png.flat"),
//             Path::new("D:C:\\Users\\den99\\Desktop\\Work\\DodoRare\\creator\\examples\\3d\\res\\android\\mipmap-xxhdpi\\"),
//             None,
//             package_label.to_string(),
//             target_sdk_version).unwrap();
//     }
// }
