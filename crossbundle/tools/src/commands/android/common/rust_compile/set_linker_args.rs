use crate::types::*;

/// Add linker args for quad engine using NDK versions <=22
pub fn add_clinker_args(
    ndk: &AndroidNdk,
    build_target: &AndroidTarget,
    target_sdk_version: u32,
) -> cargo::CargoResult<Vec<std::ffi::OsString>> {
    let linker_args = vec![
        build_arg(
            "-Clinker=",
            ndk.linker_path(build_target, target_sdk_version)?,
        ),
        "-Clinker-flavor=ld".into(),
        build_arg("-Clink-arg=--sysroot=", ndk.sysroot()?),
        build_arg(
            "-Clink-arg=-L",
            ndk.ver_specific_lib_path(target_sdk_version, build_target)?,
        ),
        build_arg("-Clink-arg=-L", ndk.sysroot_lib_dir(build_target)?),
        build_arg("-Clink-arg=-L", ndk.gcc_lib_path(build_target)?),
        "-Crelocation-model=pic".into(),
    ];

    Ok(linker_args)
}

/// Helper function to build arguments composed of concatenating two strings
pub fn build_arg(start: &str, end: impl AsRef<std::ffi::OsStr>) -> std::ffi::OsString {
    let mut new_arg = std::ffi::OsString::new();
    new_arg.push(start);
    new_arg.push(end.as_ref());
    new_arg
}

/// Add path containing libgcc.a and libunwind.a for linker to search.
/// See https://github.com/rust-lang/rust/pull/85806 for discussion on libgcc.
/// The workaround to get to NDK r23 or newer is to create a libgcc.a file with
/// the contents of 'INPUT(-lunwind)' to link in libunwind.a instead of libgcc.a
pub fn search_for_libgcc_and_libunwind(
    build_target: &AndroidTarget,
    build_path: std::path::PathBuf,
    ndk: &AndroidNdk,
    target_sdk_version: u32,
) -> cargo::CargoResult<Vec<std::ffi::OsString>> {
    let mut new_args = Vec::new();
    let linker_path = ndk.linker_path(build_target, target_sdk_version)?;
    new_args.push(build_arg("-Clinker=", linker_path));

    let libgcc_dir = build_path.join("_libgcc_");
    std::fs::create_dir_all(&libgcc_dir)?;
    let libgcc = libgcc_dir.join("libgcc.a");
    std::fs::write(libgcc, "INPUT(-lunwind)")?;
    new_args.push(build_arg("-Clink-arg=-L", libgcc_dir));

    let libunwind_dir = ndk.find_libunwind_dir(build_target)?;
    new_args.push(build_arg("-Clink-arg=-L", libunwind_dir));
    Ok(new_args)
}
