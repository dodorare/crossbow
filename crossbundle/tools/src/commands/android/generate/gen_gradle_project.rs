use std::{
    fs::File,
    io::Write,
    path::{Path, PathBuf},
};

pub fn gen_gradle_project(
    android_build_dir: &Path,
    package_id: &str,
    resources_dir: Option<PathBuf>,
    assets_dir: Option<PathBuf>,
) -> crate::error::Result<PathBuf> {
    let gradle_project_path = android_build_dir.join("gradle");

    let java_file_path = package_id
        .split('.')
        .fold(gradle_project_path.join("src"), |path, name| {
            path.join(name)
        });
    if !java_file_path.exists() {
        std::fs::create_dir_all(&java_file_path)?;
    }

    let mut java_file = File::create(java_file_path.join("CrossbowApp.java"))?;
    write!(java_file, "{}", crossbow_app_java_code(package_id))?;

    let mut build_gradle = File::create(gradle_project_path.join("build.gradle"))?;
    write!(build_gradle, "{}", crossbow_app_build_gradle(package_id))?;

    let mut gradle_properties = File::create(gradle_project_path.join("gradle.properties"))?;
    write!(gradle_properties, "{}", crossbow_app_gradle_properties())?;

    let mut settings_gradle = File::create(gradle_project_path.join("settings.gradle"))?;
    write!(settings_gradle, "{}", crossbow_app_settings_gradle())?;

    // Copy resources to gradle folder if provided
    std::fs::remove_dir_all(&gradle_project_path.join("res")).unwrap();
    let mut options = fs_extra::dir::CopyOptions::new();
    options.skip_exist = true;
    options.content_only = true;
    if let Some(resources_dir) = resources_dir {
        fs_extra::dir::copy(resources_dir, &gradle_project_path.join("res"), &options)?;
    }
    // Copy assets to gradle folder if provided
    std::fs::remove_dir_all(&gradle_project_path.join("assets")).unwrap();
    if let Some(assets_dir) = assets_dir {
        fs_extra::dir::copy(assets_dir, &gradle_project_path.join("assets"), &options)?;
    }

    Ok(gradle_project_path)
}

fn crossbow_app_java_code(package_id: &str) -> String {
    format!(
        r#"
package {package_id};

import android.os.Bundle;
import com.dodorare.crossbow.CrossbowNativeActivity;

/**
 * Template activity for Crossbow Android custom builds.
 * Feel free to extend and modify this class for your custom logic.
 */
public class CrossbowApp extends CrossbowNativeActivity {{
    @Override
	public void onCreate(Bundle savedInstanceState) {{
		super.onCreate(savedInstanceState);
	}}
}}
"#
    )
}

fn crossbow_app_build_gradle(package_id: &str) -> String {
    format!(
        r#"
buildscript {{
    repositories {{
        google()
        mavenCentral()
        maven {{ url "https://plugins.gradle.org/m2/" }}
    }}
    dependencies {{
        classpath "com.android.tools.build:gradle:7.0.0"
        classpath "org.jetbrains.kotlin:kotlin-gradle-plugin:1.6.21"
    }}
}}

repositories {{
    google()
    mavenCentral()
    maven {{ url "https://plugins.gradle.org/m2/" }}
}}

apply plugin: "com.android.application"

android {{
    compileSdkVersion 31
    buildToolsVersion "30.0.3"
    ndkVersion "23.1.7779620"

    defaultConfig {{
        applicationId "{package_id}"
        versionCode 1
        versionName "1.0"

        minSdkVersion 19
        targetSdkVersion 30
    }}
    sourceSets {{
        main {{
            manifest.srcFile "AndroidManifest.xml"
            java.srcDirs = ["src"]
            assets.srcDirs = ["assets"]
            res.srcDirs = ["res"]
        }}
        debug.jniLibs.srcDirs = ["../libs/debug/"]
        release.jniLibs.srcDirs = ["../libs/release/"]
    }}
}}

dependencies {{
    implementation "org.jetbrains.kotlin:kotlin-stdlib:1.6.21"
    implementation "androidx.appcompat:appcompat:1.4.1"
    implementation "androidx.games:games-activity:1.1.0"
    implementation "androidx.startup:startup-runtime:1.1.1"
    implementation project(":crossbow:lib")
}}
"#
    )
}

fn crossbow_app_gradle_properties() -> String {
    format!(
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
    )
}

fn crossbow_app_settings_gradle() -> String {
    format!(
        r#"
include ":crossbow"
project(":crossbow").projectDir = new File("../../../../platform/android/java/")
include ":crossbow:lib"
"#
    )
}
