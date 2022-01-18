use crate::{
    commands::android::{get_libs_in_dir, recursively_define_needed_libs, search_dylibs},
    error::*,
    tools::AndroidNdk,
    types::{AndroidTarget, IntoRustTriple, Profile},
};
use std::path::{Path, PathBuf};

/// Adds given lib and all reletad libs into APK
pub fn add_libs_into_aapt2(
    ndk: &AndroidNdk,
    lib_path: &Path,
    build_target: AndroidTarget,
    profile: Profile,
    min_sdk_version: u32,
    build_dir: &Path,
    target_dir: &Path,
) -> Result<PathBuf> {
    // Get list of android system libs (https://developer.android.com/ndk/guides/stable_apis)
    let mut system_libs = Vec::new();
    let sysroot_platform_lib_dir = ndk.sysroot_platform_lib_dir(build_target, min_sdk_version)?;
    for lib in get_libs_in_dir(&sysroot_platform_lib_dir)? {
        system_libs.push(lib);
    }

    // Get list of dylibs_paths
    let build_path = target_dir
        .join(build_target.rust_triple())
        .join(profile.as_ref());
    let mut dylibs_paths = search_dylibs(&build_path.join("build"))?;
    dylibs_paths.push(build_path.join("tools"));

    // Get list of libs that main lib need for work
    let lib_name = lib_path.file_name().unwrap().to_str().unwrap().to_owned();
    let mut needed_libs = vec![];
    recursively_define_needed_libs(
        (lib_name, lib_path.to_owned()),
        &ndk.toolchain_bin("readelf", build_target)?,
        &ndk.sysroot_lib_dir(build_target)?.join("libc++_shared.so"),
        &system_libs,
        &dylibs_paths,
        &mut needed_libs,
    )?;

    // Add all needed libs into apk archive
    for (_lib_name, lib_path) in needed_libs {
        add_lib_aapt2(&lib_path, &build_dir)?;
    }
    Ok(build_dir.to_path_buf())
}

/// Copy lib into dir then add this lib into apk file
pub fn add_lib_aapt2(lib_path: &Path, copy_lib_into_dir: &Path) -> Result<()> {
    if !lib_path.exists() {
        return Err(Error::PathNotFound(lib_path.to_owned()));
    }
    std::fs::create_dir_all(&copy_lib_into_dir)?;
    let filename = lib_path.file_name().unwrap();
    let mut options = fs_extra::file::CopyOptions::new();
    options.overwrite = true;
    fs_extra::file::copy(&lib_path, copy_lib_into_dir.join(&filename), &options)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        commands::{
            android::{self, compile_rust_for_android_with_bevy},
            find_package_cargo_manifest_path,
        },
        tools::AndroidSdk,
    };

    #[test]
    fn add_libs_into_aapt2_test() {
        // Specifies path to workspace
        let user_dirs = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let current_dir = user_dirs.parent().unwrap().parent().unwrap().to_path_buf();
        let package_manifest_path = find_package_cargo_manifest_path(&current_dir).unwrap();
        let project_path = package_manifest_path.parent().unwrap().to_owned();

        // Assigns configuration for project
        let sdk = AndroidSdk::from_env().unwrap();
        let ndk = AndroidNdk::from_env(Some(sdk.sdk_path())).unwrap();
        let build_target = AndroidTarget::Aarch64LinuxAndroid;
        let profile = Profile::Debug;
        let target_sdk_version = 30;
        let package_name = "bevy";
        let lib_name = format!("lib{}.so", package_name.replace("-", "_"));

        // Compile rust code for android with bevy engine
        compile_rust_for_android_with_bevy(
            &ndk,
            build_target,
            &project_path,
            profile,
            vec![],
            false,
            false,
            target_sdk_version,
            &lib_name,
        )
        .unwrap();

        // Specifies needed directories
        let target_dir = project_path.join("target");
        let out_dir = target_dir
            .join(build_target.rust_triple())
            .join(profile.as_ref());
        let compiled_lib = out_dir.join(format!("lib{}.so", package_name));
        assert!(compiled_lib.exists());
        let android_abi = build_target.android_abi();
        let android_compiled_lib = target_dir
            .join("android")
            .join(profile.to_string())
            .join("lib")
            .join(android_abi);

        // Adds libs into specified directory
        let lib = android::add_libs_into_aapt2(
            &ndk,
            &compiled_lib,
            build_target,
            profile,
            target_sdk_version,
            &android_compiled_lib,
            &target_dir,
        )
        .unwrap();
        assert!(lib.exists());
        println!("library saved in {:?}", lib);
    }
}
