use crate::error::*;
use rust_embed::RustEmbed;
use std::{
    fs::File,
    io::Write,
    path::{Path, PathBuf},
};

#[derive(RustEmbed)]
#[folder = "../../platform/android/java/app"]
#[include = "src/*"]
#[include = "*.xml"]
#[include = "*.gradle"]
#[exclude = "build/"]
#[exclude = "libs/"]
pub struct CrossbowAppTemplate;

#[derive(Clone, Debug)]
pub struct GradleDependencyProject {
    include: String,
    project_dir: Option<PathBuf>,
}

pub fn gen_gradle_project(
    android_build_dir: &Path,
    resources_dir: Option<PathBuf>,
    assets_dir: Option<PathBuf>,
    dependencies: &[GradleDependencyProject],
) -> Result<PathBuf> {
    let gradle_project_path = android_build_dir.join("gradle");

    for file_name in CrossbowAppTemplate::iter() {
        let file_path = gradle_project_path.join(file_name.as_ref());
        if let Some(path) = file_path.parent() {
            std::fs::create_dir_all(path)?;
        }
        let mut build_gradle = File::create(file_path)?;
        let file = CrossbowAppTemplate::get(file_name.as_ref()).unwrap();
        write!(
            build_gradle,
            "{}",
            std::str::from_utf8(file.data.as_ref()).unwrap()
        )?;
    }

    let mut gradle_properties = File::create(gradle_project_path.join("gradle.properties"))?;
    write!(gradle_properties, "{}", get_gradle_properties())?;

    let mut settings_gradle = File::create(gradle_project_path.join("settings.gradle"))?;
    write!(settings_gradle, "{}", get_settings_gradle(dependencies)?)?;

    let mut options = fs_extra::dir::CopyOptions::new();
    options.overwrite = true;
    options.content_only = true;
    // Copy resources to gradle folder if provided
    if let Some(resources_dir) = resources_dir {
        fs_extra::dir::copy(resources_dir, &gradle_project_path.join("res"), &options)?;
    }
    // Copy assets to gradle folder if provided
    if let Some(assets_dir) = assets_dir {
        fs_extra::dir::copy(assets_dir, &gradle_project_path.join("assets"), &options)?;
    }

    Ok(gradle_project_path)
}

fn get_gradle_properties() -> String {
    r#"org.gradle.jvmargs=-Xmx2048m -Dfile.encoding=UTF-8
android.useAndroidX=true
android.enableJetifier=true
android.nonTransitiveRClass=true
"#
    .to_string()
}

fn get_settings_gradle(dependencies: &[GradleDependencyProject]) -> Result<String> {
    let mut result = "".to_owned();
    for dependency in dependencies {
        result += format!("include \"{}\"\n", dependency.include).as_str();
        if let Some(dir) = &dependency.project_dir {
            let dir_path = dunce::canonicalize(dir)
                .map_err(|_| AndroidError::GradleDependencyProjectNotFound(dir.to_path_buf()))?;
            if !dir_path.join("build.gradle").exists() {
                return Err(
                    AndroidError::GradleDependencyProjectNoBuildFile(dir.to_path_buf()).into(),
                );
            }
            result += format!(
                "project(\"{}\").projectDir = new File(\"{}\")\n",
                dependency.include,
                dir_path.to_string_lossy()
            )
            .as_str();
        }
    }
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_crossbow_app_template() {
        for file in CrossbowAppTemplate::iter() {
            println!("{}", file.as_ref());
        }
        assert!(
            CrossbowAppTemplate::get("src/com/crossbow/game/CrossbowApp.kt").is_some(),
            "CrossbowApp.kt should exist"
        );
        assert!(
            CrossbowAppTemplate::get("libs/debug/arm64-v8a/libcrossbow_android.so").is_none(),
            "libcrossbow_android.so shouldn't exist"
        );
    }

    #[test]
    fn test_crossbow_settings_gradle() {
        let dep = GradleDependencyProject {
            include: ":crossbow".to_string(),
            // Path converted to absolute from crossbundle/tools directory
            project_dir: Some(PathBuf::from("../../platform/android/java")),
        };
        assert_eq!(
            get_settings_gradle(&[dep.clone()]).unwrap(),
            format!(
                "include \":crossbow\"\nproject(\":crossbow\").projectDir = new File(\"{}\")\n",
                dunce::canonicalize(dep.project_dir.unwrap())
                    .unwrap()
                    .to_string_lossy()
            )
        );
    }
}
