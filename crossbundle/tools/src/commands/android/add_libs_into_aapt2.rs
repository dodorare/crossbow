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

