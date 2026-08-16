use crate::error::*;
use crate::{
    commands::CargoProject,
    types::{AndroidGradlePlugins, AndroidRuntime, GradleDependencyProject},
};
use crossbow_android::embed::CrossbowAndroidAppTemplate;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AndroidSdkVersions {
    pub min_sdk: u32,
    pub target_sdk: u32,
}

pub fn gen_gradle_project(
    package_name: &str,
    version_code: u32,
    version_name: &str,
    sdk_versions: AndroidSdkVersions,
    android_build_dir: &Path,
    assets_dir: &Option<PathBuf>,
    resources_dir: &Option<PathBuf>,
    plugins: &AndroidGradlePlugins,
    runtime: AndroidRuntime,
    library_name: &str,
    cargo_project: &CargoProject,
    crossbow_bridge: bool,
) -> Result<PathBuf> {
    let gradle_project_path = android_build_dir.join("gradle");

    for file_name in CrossbowAndroidAppTemplate::iter() {
        let file_path = gradle_project_path.join(file_name.as_ref());
        if let Some(path) = file_path.parent() {
            std::fs::create_dir_all(path)?;
        }
        let file = CrossbowAndroidAppTemplate::get(file_name.as_ref())
            .expect("embedded template entry disappeared during iteration");
        std::fs::write(file_path, file.data.as_ref())?;
    }

    if runtime == AndroidRuntime::Miniquad || !crossbow_bridge {
        std::fs::remove_file(gradle_project_path.join("src/com/crossbow/game/CrossbowApp.kt"))?;
    }
    if runtime == AndroidRuntime::Miniquad {
        install_miniquad_runtime(
            &gradle_project_path,
            package_name,
            library_name,
            cargo_project,
            crossbow_bridge,
        )?;
    }

    std::fs::write(
        gradle_project_path.join("gradle.properties"),
        get_gradle_properties(
            package_name,
            version_code,
            version_name,
            sdk_versions,
            plugins,
            crossbow_bridge,
        ),
    )?;

    std::fs::write(
        gradle_project_path.join("settings.gradle"),
        get_settings_gradle(&plugins.local_projects)?,
    )?;

    let mut options = fs_extra::dir::CopyOptions::new();
    options.overwrite = true;
    options.content_only = true;
    // Copy resources to gradle folder if provided
    if let Some(resources_dir) = resources_dir {
        let path = gradle_project_path.join("res");
        if path.exists() {
            std::fs::remove_dir_all(&path)?;
        }
        fs_extra::dir::copy(resources_dir, &path, &options)?;
    }
    // Copy assets to gradle folder if provided
    if let Some(assets_dir) = assets_dir {
        let path = gradle_project_path.join("assets");
        if path.exists() {
            std::fs::remove_dir_all(&path)?;
        }
        fs_extra::dir::copy(assets_dir, &path, &options)?;
    }

    Ok(gradle_project_path)
}

fn install_miniquad_runtime(
    gradle_project_path: &Path,
    package_name: &str,
    library_name: &str,
    cargo_project: &CargoProject,
    crossbow_bridge: bool,
) -> Result<()> {
    if crossbow_bridge {
        cargo_project.dependency("crossbow").map_err(|error| {
            anyhow::anyhow!(
                "Miniquad permissions and plugins require the `crossbow` crate: {error}"
            )
        })?;
    }
    let miniquad = cargo_project.dependency("miniquad")?;
    let crate_root = miniquad
        .manifest_path
        .parent()
        .ok_or_else(|| anyhow::anyhow!("Miniquad's Cargo.toml has no parent directory"))?;
    let java_root = crate_root.join("java");
    let main_activity = java_root.join("MainActivity.java");
    let quad_native = java_root.join("QuadNative.java");
    for source in [&main_activity, &quad_native] {
        if !source.is_file() {
            return Err(anyhow::anyhow!(
                "Miniquad {} does not contain expected Android source `{}`",
                miniquad.version,
                source.display()
            )
            .into());
        }
    }

    let package_dir = gradle_project_path
        .join("src")
        .join(package_name.replace('.', "/"));
    std::fs::create_dir_all(&package_dir)?;
    let activity = std::fs::read_to_string(main_activity)?;
    if !activity.contains("TARGET_PACKAGE_NAME") || !activity.contains("LIBRARY_NAME") {
        return Err(anyhow::anyhow!(
            "Miniquad {} has an unsupported Android MainActivity.java template",
            miniquad.version
        )
        .into());
    }
    let activity = activity
        .replace("TARGET_PACKAGE_NAME", package_name)
        .replace("LIBRARY_NAME", library_name);
    std::fs::write(package_dir.join("MainActivity.java"), activity)?;

    let quad_native_dir = gradle_project_path.join("src/quad_native");
    std::fs::create_dir_all(&quad_native_dir)?;
    std::fs::copy(quad_native, quad_native_dir.join("QuadNative.java"))?;
    let crossbow_activity = package_dir.join("CrossbowApp.kt");
    if crossbow_bridge {
        std::fs::write(crossbow_activity, miniquad_crossbow_activity(package_name))?;
    } else if crossbow_activity.exists() {
        std::fs::remove_file(crossbow_activity)?;
    }
    Ok(())
}

fn miniquad_crossbow_activity(package_name: &str) -> String {
    format!(
        r#"@file:Suppress("DEPRECATION", "OVERRIDE_DEPRECATION")

package {package_name}

import android.content.Intent
import android.os.Bundle
import com.crossbow.library.Crossbow
import com.crossbow.library.CrossbowHost
import com.crossbow.library.CrossbowLib

open class CrossbowApp : MainActivity(), CrossbowHost {{
    private var crossbow: Crossbow? = null

    override fun onCreate(savedInstanceState: Bundle?) {{
        CrossbowLib.initializeAndroidContext(this)
        super.onCreate(savedInstanceState)
        crossbow = if (savedInstanceState == null) {{
            Crossbow().also {{
                fragmentManager.beginTransaction().add(android.R.id.content, it).commit()
            }}
        }} else {{
            fragmentManager.findFragmentById(android.R.id.content) as? Crossbow
        }}
    }}

    override fun onNewIntent(intent: Intent) {{
        super.onNewIntent(intent)
        crossbow?.onNewIntent(intent)
    }}

    override fun onActivityResult(requestCode: Int, resultCode: Int, data: Intent?) {{
        super.onActivityResult(requestCode, resultCode, data)
        crossbow?.onActivityResult(requestCode, resultCode, data)
    }}

    override fun onRequestPermissionsResult(
        requestCode: Int,
        permissions: Array<String>,
        grantResults: IntArray
    ) {{
        super.onRequestPermissionsResult(requestCode, permissions, grantResults)
        crossbow?.onRequestPermissionsResult(requestCode, permissions, grantResults)
    }}

    override fun onBackPressed() {{
        crossbow?.onBackPressed() ?: super.onBackPressed()
    }}

    override fun onDestroy() {{
        super.onDestroy()
        CrossbowLib.releaseAndroidContext()
    }}
}}
"#
    )
}

fn get_default_gradle_props(
    package_name: &str,
    version_code: u32,
    version_name: &str,
    sdk_versions: AndroidSdkVersions,
) -> String {
    format!(
        r#"org.gradle.jvmargs=-Xmx2048m -Dfile.encoding=UTF-8
android.useAndroidX=true
android.nonTransitiveRClass=true
export_package_name={package_name}
export_version_code={version_code}
export_version_name={version_name}
export_version_min_sdk={}
export_version_target_sdk={}
"#,
        sdk_versions.min_sdk, sdk_versions.target_sdk
    )
}

fn get_gradle_properties(
    package_name: &str,
    version_code: u32,
    version_name: &str,
    sdk_versions: AndroidSdkVersions,
    plugins: &AndroidGradlePlugins,
    crossbow_bridge: bool,
) -> String {
    let mut result =
        get_default_gradle_props(package_name, version_code, version_name, sdk_versions);
    result.push_str(&format!("crossbow_bridge={crossbow_bridge}\n"));
    if !plugins.maven_repos.is_empty() {
        result.push_str(&format!(
            "plugins_maven_repos={}\n",
            plugins.maven_repos.join("\\|")
        ));
    }
    if !plugins.remote.is_empty() {
        result.push_str(&format!(
            "plugins_remote_binaries={}\n",
            plugins.remote.join("\\|")
        ));
    }
    if !plugins.local.is_empty() {
        let local = plugins
            .local
            .iter()
            .map(|p| dunce::simplified(p).to_string_lossy())
            .collect::<Vec<_>>()
            .join("\\|");
        result.push_str(&format!("plugins_local_binaries={local}\n"));
    }
    if !plugins.local_projects.is_empty() {
        let projects = plugins
            .local_projects
            .iter()
            .filter(|p| !p.dont_implement)
            .map(|p| p.include.clone())
            .collect::<Vec<_>>()
            .join("\\|");
        result.push_str(&format!("plugins_local_projects={projects}\n"));
    }
    result
}

fn get_settings_gradle(dependencies: &[GradleDependencyProject]) -> Result<String> {
    let mut result = String::new();
    for dependency in dependencies {
        result.push_str(&format!("include \"{}\"\n", dependency.include));
        if let Some(dir) = &dependency.project_dir {
            let dir_path = dunce::canonicalize(dir)
                .map_err(|_| AndroidError::GradleDependencyProjectNotFound(dir.to_path_buf()))?;
            if !dir_path.join("build.gradle").exists() {
                return Err(
                    AndroidError::GradleDependencyProjectNoBuildFile(dir.to_path_buf()).into(),
                );
            }
            result.push_str(&format!(
                "project(\"{}\").projectDir = new File({:?})\n",
                dependency.include,
                dir_path.to_string_lossy()
            ));
        }
    }
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn miniquad_fixture() -> (tempfile::TempDir, CargoProject) {
        let root = tempfile::tempdir().unwrap();
        for package in ["app", "miniquad"] {
            std::fs::create_dir_all(root.path().join(package).join("src")).unwrap();
            std::fs::write(root.path().join(package).join("src/lib.rs"), "").unwrap();
        }
        std::fs::write(
            root.path().join("Cargo.toml"),
            "[workspace]\nresolver = \"3\"\nmembers = [\"app\", \"miniquad\"]\n",
        )
        .unwrap();
        std::fs::write(
            root.path().join("app/Cargo.toml"),
            "[package]\nname = \"app\"\nversion = \"0.1.0\"\nedition = \"2024\"\n\
             [dependencies]\nminiquad = { path = \"../miniquad\" }\n",
        )
        .unwrap();
        std::fs::write(
            root.path().join("miniquad/Cargo.toml"),
            "[package]\nname = \"miniquad\"\nversion = \"1.2.3\"\nedition = \"2024\"\n",
        )
        .unwrap();
        std::fs::create_dir(root.path().join("miniquad/java")).unwrap();
        std::fs::write(
            root.path().join("miniquad/java/MainActivity.java"),
            "package TARGET_PACKAGE_NAME; class MainActivity { static { System.loadLibrary(\"LIBRARY_NAME\"); } }",
        )
        .unwrap();
        std::fs::write(
            root.path().join("miniquad/java/QuadNative.java"),
            "package quad_native; class QuadNative {}",
        )
        .unwrap();
        let project = CargoProject::load(&root.path().join("app/Cargo.toml")).unwrap();
        (root, project)
    }

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
            get_settings_gradle(std::slice::from_ref(&dep)).unwrap(),
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
        let sdk_versions = AndroidSdkVersions {
            min_sdk: 23,
            target_sdk: 36,
        };
        let mut plugins = AndroidGradlePlugins {
            local: vec![],
            remote: vec![],
            maven_repos: vec![],
            local_projects: vec![],
        };
        assert_eq!(
            get_gradle_properties("com.crossbow.test", 1, "1.0", sdk_versions, &plugins, true,),
            format!(
                "{}crossbow_bridge=true\n",
                get_default_gradle_props("com.crossbow.test", 1, "1.0", sdk_versions)
            ),
        );

        plugins.local.push(PathBuf::from("../../MyPlugin.aar"));
        assert_eq!(
            get_gradle_properties("com.crossbow.test", 1, "1.0", sdk_versions, &plugins, true,),
            format!(
                "{}{}{}",
                get_default_gradle_props("com.crossbow.test", 1, "1.0", sdk_versions),
                "crossbow_bridge=true\n",
                "plugins_local_binaries=../../MyPlugin.aar\n"
            )
        );
    }

    #[test]
    fn miniquad_bridge_uses_the_application_package() {
        let source = miniquad_crossbow_activity("dev.crossbow.game");
        assert!(source.contains("package dev.crossbow.game"));
        assert!(source.contains("class CrossbowApp : MainActivity(), CrossbowHost"));
        assert!(source.contains("onRequestPermissionsResult"));
    }

    #[test]
    fn installs_sources_from_the_resolved_miniquad_package() {
        let (root, project) = miniquad_fixture();
        let output = root.path().join("output");
        install_miniquad_runtime(&output, "dev.crossbow.game", "mobile_game", &project, false)
            .unwrap();

        let activity =
            std::fs::read_to_string(output.join("src/dev/crossbow/game/MainActivity.java"))
                .unwrap();
        assert!(activity.contains("package dev.crossbow.game"));
        assert!(activity.contains("System.loadLibrary(\"mobile_game\")"));
        assert!(output.join("src/quad_native/QuadNative.java").is_file());
        assert!(!output.join("src/dev/crossbow/game/CrossbowApp.kt").exists());
    }

    #[test]
    fn bridge_requires_crossbow_and_miniquad_requires_its_java_sources() {
        let (root, project) = miniquad_fixture();
        let bridge_error = install_miniquad_runtime(
            &root.path().join("bridge"),
            "dev.crossbow.game",
            "game",
            &project,
            true,
        )
        .unwrap_err()
        .to_string();
        assert!(bridge_error.contains("require the `crossbow` crate"));

        std::fs::remove_file(root.path().join("miniquad/java/QuadNative.java")).unwrap();
        let source_error = install_miniquad_runtime(
            &root.path().join("missing-source"),
            "dev.crossbow.game",
            "game",
            &project,
            false,
        )
        .unwrap_err()
        .to_string();
        assert!(source_error.contains("QuadNative.java"));
        std::fs::write(
            root.path().join("miniquad/java/QuadNative.java"),
            "package quad_native; class QuadNative {}",
        )
        .unwrap();
        std::fs::write(
            root.path().join("miniquad/java/MainActivity.java"),
            "class MainActivity {}",
        )
        .unwrap();
        let error = install_miniquad_runtime(
            &root.path().join("output"),
            "dev.crossbow.game",
            "game",
            &project,
            false,
        )
        .unwrap_err()
        .to_string();
        assert!(error.contains("unsupported Android MainActivity.java template"));
    }
}
