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
    let abi = build_target.android_abi();
    let out_dir = build_dir.join("lib").join(abi);
    for (_lib_name, lib_path) in needed_libs {
        add_lib_aapt2(&lib_path, &out_dir)?;
    }
    Ok(out_dir)
}

/// Copy lib into `out_dir` then add this lib into apk file
pub fn add_lib_aapt2(lib_path: &Path, out_dir: &Path) -> Result<()> {
    if !lib_path.exists() {
        return Err(Error::PathNotFound(lib_path.to_owned()));
    }
    std::fs::create_dir_all(&out_dir)?;
    let filename = lib_path.file_name().unwrap();
    let mut options = fs_extra::file::CopyOptions::new();
    options.overwrite = true;
    fs_extra::file::copy(&lib_path, out_dir.join(&filename), &options)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        commands::{
            android::{self, compile_rust_for_android},
            gen_minimal_project,
        },
        tools::AndroidSdk,
        types::ApplicationWrapper,
    };

    #[test]
    fn add_libs_into_aapt2_test() {
        // Creates temporary directory
        let tempdir = tempfile::tempdir().unwrap();
        let project_path = tempdir.path();

        // Assigns configuration for project
        let macroquad_project = true;
        let package_name = gen_minimal_project(&project_path, macroquad_project).unwrap();

        // Assigns configuration for project
        let sdk = AndroidSdk::from_env().unwrap();
        let ndk = AndroidNdk::from_env(Some(sdk.sdk_path())).unwrap();
        let build_target = AndroidTarget::Aarch64LinuxAndroid;
        let profile = Profile::Debug;
        let target_sdk_version = 30;
        let lib_name = format!("lib{}.so", package_name.replace("-", "_"));
        let target_dir = project_path.join("target");
        let android_build_dir = target_dir.join("android").join(profile.to_string());

        // Compile rust code for android with bevy engine
        compile_rust_for_android(
            &ndk,
            build_target,
            &project_path,
            profile,
            vec![],
            false,
            false,
            target_sdk_version,
            &lib_name,
            ApplicationWrapper::NdkGlue,
        )
        .unwrap();

        // Specifies needed directories to manage library location
        let mut libs = Vec::new();
        let out_dir = target_dir
            .join(build_target.rust_triple())
            .join(profile.as_ref());
        let compiled_lib = out_dir.join(lib_name);
        libs.push((compiled_lib, build_target));

        // Adds libs into specified directory
        for (compiled_lib, build_target) in libs {
            let lib = android::add_libs_into_aapt2(
                &ndk,
                &compiled_lib,
                build_target,
                profile,
                target_sdk_version,
                &android_build_dir,
                &target_dir,
            )
            .unwrap();
            assert!(lib.exists());
            println!("library saved in {:?}", lib);
        }
    }
}
