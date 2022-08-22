use crate::error::*;
use crossbow_android::embed::CrossbowAndroidAppTemplate;
use serde::{Deserialize, Serialize};
use std::{
    fs::File,
    io::Write,
    path::{Path, PathBuf},
};

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct AndroidGradlePlugins {
    /// Android Gradle local plugins.
    #[serde(default, rename = "plugins_local")]
    pub local: Vec<PathBuf>,
    /// Android Gradle remote plugins.
    #[serde(default, rename = "plugins_remote")]
    pub remote: Vec<String>,
    /// Android Gradle custom maven repositories.
    #[serde(default, rename = "plugins_maven_repos")]
    pub maven_repos: Vec<String>,
    /// Android Gradle local plugins projects.
    #[serde(default, rename = "plugins_local_projects")]
    pub local_projects: Vec<GradleDependencyProject>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GradleDependencyProject {
    include: String,
    #[serde(default)]
    dont_implement: bool,
    project_dir: Option<PathBuf>,
}

pub fn gen_gradle_project(
    version_code: u32,
    version_name: &str,
    android_build_dir: &Path,
    assets_dir: &Option<PathBuf>,
    resources_dir: &Option<PathBuf>,
    plugins: &AndroidGradlePlugins,
) -> Result<PathBuf> {
    let gradle_project_path = android_build_dir.join("gradle");

    for file_name in CrossbowAndroidAppTemplate::iter() {
        let file_path = gradle_project_path.join(file_name.as_ref());
        if let Some(path) = file_path.parent() {
            std::fs::create_dir_all(path)?;
        }
        let mut build_gradle = File::create(file_path)?;
        let file = CrossbowAndroidAppTemplate::get(file_name.as_ref()).unwrap();
        write!(
            build_gradle,
            "{}",
            std::str::from_utf8(file.data.as_ref()).unwrap()
        )?;
    }

    let mut gradle_properties = File::create(gradle_project_path.join("gradle.properties"))?;
    write!(
        gradle_properties,
        "{}",
        get_gradle_properties(version_code, version_name, plugins)?
    )?;

    let mut settings_gradle = File::create(gradle_project_path.join("settings.gradle"))?;
    write!(
        settings_gradle,
        "{}",
        get_settings_gradle(&plugins.local_projects)?
    )?;

    let mut options = fs_extra::dir::CopyOptions::new();
    options.overwrite = true;
    options.content_only = true;
    // Copy resources to gradle folder if provided
    if let Some(resources_dir) = resources_dir {
        let path = gradle_project_path.join("res");
        std::fs::remove_dir_all(&path).ok();
        fs_extra::dir::copy(resources_dir, &path, &options)?;
    }
    // Copy assets to gradle folder if provided
    if let Some(assets_dir) = assets_dir {
        let path = gradle_project_path.join("assets");
        std::fs::remove_dir_all(&path).ok();
        fs_extra::dir::copy(assets_dir, &path, &options)?;
    }

    Ok(gradle_project_path)
}

const DEFAULT_GRADLE_PROPERTIES: &str = r#"org.gradle.jvmargs=-Xmx2048m -Dfile.encoding=UTF-8
android.useAndroidX=true
android.enableJetifier=true
android.nonTransitiveRClass=true
"#;

fn get_gradle_properties(
    version_code: u32,
    version_name: &str,
    plugins: &AndroidGradlePlugins,
) -> Result<String> {
    let mut result = DEFAULT_GRADLE_PROPERTIES.to_string();
    result = format!("{}export_version_code={}\n", result, version_code);
    result = format!("{}export_version_name={}\n", result, version_name);
    if !plugins.maven_repos.is_empty() {
        result = format!(
            "{}plugins_maven_repos={}\n",
            result,
            plugins.maven_repos.join("\\|")
        );
    }
    if !plugins.remote.is_empty() {
        result = format!(
            "{}plugins_remote_binaries={}\n",
            result,
            plugins.remote.join("\\|")
        );
    }
    if !plugins.local.is_empty() {
        let local = plugins
            .local
            .iter()
            .map(|p| dunce::simplified(p).to_string_lossy())
            .collect::<Vec<_>>()
            .join("\\|");
        result = format!("{}plugins_local_binaries={}\n", result, local);
    }
    if !plugins.local_projects.is_empty() {
        let projects = plugins
            .local_projects
            .iter()
            .filter(|p| !p.dont_implement)
            .map(|p| p.include.clone())
            .collect::<Vec<_>>()
            .join("\\|");
        result = format!("{}plugins_local_projects={}\n", result, projects);
    }
    Ok(result)
}

fn get_settings_gradle(dependencies: &[GradleDependencyProject]) -> Result<String> {
    let mut result = "".to_owned();
    for dependency in dependencies {
        result = format!("{}include \"{}\"\n", result, dependency.include);
        if let Some(dir) = &dependency.project_dir {
            let dir_path = dunce::canonicalize(dir)
                .map_err(|_| AndroidError::GradleDependencyProjectNotFound(dir.to_path_buf()))?;
            if !dir_path.join("build.gradle").exists() {
                return Err(
                    AndroidError::GradleDependencyProjectNoBuildFile(dir.to_path_buf()).into(),
                );
            }
            result = format!(
                "{}project(\"{}\").projectDir = new File({:?})\n",
                result,
                dependency.include,
                dir_path.to_string_lossy()
            );
        }
    }
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_crossbow_app_template() {
        for file in CrossbowAndroidAppTemplate::iter() {
            println!("{}", file.as_ref());
        }
        assert!(
            CrossbowAndroidAppTemplate::get("src/com/crossbow/game/CrossbowApp.kt").is_some(),
            "CrossbowApp.kt should exist"
        );
        assert!(
            CrossbowAndroidAppTemplate::get("libs/debug/arm64-v8a/libcrossbow_android.so")
                .is_none(),
            "libcrossbow_android.so shouldn't exist"
        );
    }

    #[test]
    fn test_crossbow_settings_gradle() {
        let dep = GradleDependencyProject {
            include: ":crossbow".to_string(),
            dont_implement: false,
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

    #[test]
    fn test_crossbow_gradle_properties() {
        let mut plugins = AndroidGradlePlugins {
            local: vec![],
            remote: vec![],
            maven_repos: vec![],
            local_projects: vec![],
        };
        assert_eq!(
            get_gradle_properties(1, "1.0", &plugins).unwrap(),
            DEFAULT_GRADLE_PROPERTIES
        );

        plugins.local.push(PathBuf::from("../../MyPlugin.aar"));
        assert_eq!(
            get_gradle_properties(1, "1.0", &plugins).unwrap(),
            format!(
                "{}{}",
                DEFAULT_GRADLE_PROPERTIES, "plugins_local_binaries=../../MyPlugin.aar\n"
            )
        );
    }
}
