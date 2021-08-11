use crate::{
    commands::android::{get_libs_in_dir, recursively_define_needed_libs, search_dylibs},
    error::*,
    tools::{AndroidNdk, AndroidSdk},
    types::{AndroidTarget, IntoRustTriple, Profile},
};
use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::{Path, PathBuf},
};

/// Adds given lib and all reletad libs into APK.
pub fn add_libs_into_aapt2(
    sdk: &AndroidSdk,
    ndk: &AndroidNdk,
    apk_path: &Path,
    lib_path: &Path,
    build_target: AndroidTarget,
    profile: Profile,
    min_sdk_version: u32,
    build_dir: &Path,
    target_dir: &Path,
) -> Result<()> {
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
        aapt2_add_lib(sdk, apk_path, &lib_path, &out_dir, abi)?;
    }
    Ok(())
}

/// Copy lib into `out_dir` then add this lib into apk file.
fn aapt2_add_lib(
    sdk: &AndroidSdk,
    apk_path: &Path,
    lib_path: &Path,
    out_dir: &Path,
    abi: &str,
) -> Result<()> {
    if !lib_path.exists() {
        return Err(Error::PathNotFound(lib_path.to_owned()));
    }
    std::fs::create_dir_all(&out_dir)?;
    let file_name = lib_path.file_name().unwrap();
    std::fs::copy(lib_path, &out_dir.join(&file_name))?;
    // `aapt2 a[dd] [-v] file.{zip,jar,apk} file1 [file2 ...]`
    // Add specified files to Zip-compatible archive.
    let mut aapt2 = sdk.build_tool(bin!("aapt2"), Some(apk_path.parent().unwrap()))?;
    aapt2.arg("add")
        .arg(apk_path)
        .arg(format!("lib/{}/{}", abi, file_name.to_str().unwrap()));
    aapt2.output_err(true)?;
    Ok(())
}
