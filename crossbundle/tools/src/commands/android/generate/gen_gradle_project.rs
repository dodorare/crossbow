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
struct CrossbowAppTemplate;

pub fn gen_gradle_project(
    android_build_dir: &Path,
    resources_dir: Option<PathBuf>,
    assets_dir: Option<PathBuf>,
) -> crate::error::Result<PathBuf> {
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
    write!(gradle_properties, "{}", crossbow_app_gradle_properties())?;

    let mut settings_gradle = File::create(gradle_project_path.join("settings.gradle"))?;
    write!(settings_gradle, "{}", crossbow_app_settings_gradle())?;

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

fn crossbow_app_gradle_properties() -> String {
    r#"
# Project-wide Gradle settings.

# IDE (e.g. Android Studio) users:
# Gradle settings configured through the IDE *will override*
# any settings specified in this file.
# For more details on how to configure your build environment visit
# http://www.gradle.org/docs/current/userguide/build_environment.html

# Specifies the JVM arguments used for the daemon process.
# The setting is particularly useful for tweaking memory settings.
org.gradle.jvmargs=-Xmx2048m -Dfile.encoding=UTF-8

# When configured, Gradle will run in incubating parallel mode.
# This option should only be used with decoupled projects. More details, visit
# http://www.gradle.org/docs/current/userguide/multi_project_builds.html#sec:decoupled_projects
# org.gradle.parallel=true
# AndroidX package structure to make it clearer which packages are bundled with the
# Android operating system, and which are packaged with your app"s APK
# https://developer.android.com/topic/libraries/support-library/androidx-rn
android.useAndroidX=true
android.enableJetifier=true

# Enables namespacing of each library's R class so that its R class includes only the
# resources declared in the library itself and none from the library's dependencies,
# thereby reducing the size of the R class for that library
android.nonTransitiveRClass=true
"#
    .to_string()
}

fn crossbow_app_settings_gradle() -> String {
    r#"
include ":crossbow"
project(":crossbow").projectDir = new File("../../../../platform/android/java/")
include ":crossbow:lib"
"#
    .to_string()
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
}
