use crate::types::*;
use std::io::Write;

/// Returns the environment variables needed by CMake build scripts.
pub fn cmake_env(
    build_target: crate::types::AndroidTarget,
    ndk: &AndroidNdk,
    min_sdk_version: u32,
    build_target_dir: &std::path::Path,
) -> cargo::CargoResult<Vec<(String, std::ffi::OsString)>> {
    // Return path to toolchain cmake file
    let cmake_toolchain_path = write_cmake_toolchain(
        min_sdk_version,
        ndk.ndk_path(),
        build_target_dir,
        build_target,
    )?;

    Ok(vec![
        (
            "CMAKE_TOOLCHAIN_FILE".to_owned(),
            cmake_toolchain_path.into_os_string(),
        ),
        (
            "CMAKE_GENERATOR".to_owned(),
            std::ffi::OsString::from("Unix Makefiles"),
        ),
        (
            "CMAKE_MAKE_PROGRAM".to_owned(),
            make_path(ndk.ndk_path()).into_os_string(),
        ),
    ])
}

/// Returns path to NDK provided make
pub fn make_path(ndk_path: &std::path::Path) -> std::path::PathBuf {
    ndk_path
        .join("prebuild")
        .join(super::consts::HOST_TAG)
        .join("make")
}

/// Write a CMake toolchain which will remove references to the rustc build target before
/// including the NDK provided toolchain. The NDK provided android toolchain will set the
/// target appropriately Returns the path to the generated toolchain file
pub fn write_cmake_toolchain(
    min_sdk_version: u32,
    ndk_path: &std::path::Path,
    build_target_dir: &std::path::Path,
    build_target: crate::types::AndroidTarget,
) -> cargo::util::CargoResult<std::path::PathBuf> {
    let toolchain_path = build_target_dir.join("cargo-apk.toolchain.cmake");
    let mut toolchain_file = std::fs::File::create(&toolchain_path).unwrap();
    writeln!(
        toolchain_file,
        r#"set(ANDROID_PLATFORM android-{min_sdk_version})
        set(ANDROID_ABI {abi})
        string(REPLACE "--target={build_target}" "" CMAKE_C_FLAGS "${{CMAKE_C_FLAGS}}")
        string(REPLACE "--target={build_target}" "" CMAKE_CXX_FLAGS "${{CMAKE_CXX_FLAGS}}")
        unset(CMAKE_C_COMPILER CACHE)
        unset(CMAKE_CXX_COMPILER CACHE)
        include("{ndk_path}/build/cmake/android.toolchain.cmake")"#,
        min_sdk_version = min_sdk_version,
        ndk_path = dunce::simplified(ndk_path).to_string_lossy(),
        build_target = build_target.rust_triple(),
        abi = build_target.android_abi(),
    )?;
    Ok(toolchain_path)
}
