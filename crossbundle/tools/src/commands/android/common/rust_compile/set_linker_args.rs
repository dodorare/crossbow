use crate::types::*;

/// Add linker args for quad engine using NDK versions <=22
pub fn add_clinker_args(
    ndk: &AndroidNdk,
    build_target: &AndroidTarget,
    target_sdk_version: u32,
) -> cargo::CargoResult<Vec<std::ffi::OsString>> {
    let linker_args = vec![
        build_arg("-Clinker=", ndk.linker_path(build_target)?),
        "-Clinker-flavor=ld".into(),
        build_arg("-Clink-arg=--sysroot=", ndk.sysroot()?),
        build_arg(
            "-Clink-arg=-L",
            ndk.version_specific_libraries_path(target_sdk_version, build_target)?,
        ),
        build_arg(
            "-Clink-arg=-L",
            ndk.sysroot_lib_dir(build_target).map_err(|_| {
                anyhow::Error::msg(format!(
                    "Failed to get access to the {:?}",
                    ndk.sysroot_lib_dir(build_target)
                ))
            })?,
        ),
        build_arg("-Clink-arg=-L", ndk.gcc_lib_path(build_target)?),
        "-Crelocation-model=pic".into(),
    ];

    Ok(linker_args)
}

/// Helper function to build arguments composed of concatenating two strings
fn build_arg(start: &str, end: impl AsRef<std::ffi::OsStr>) -> std::ffi::OsString {
    let mut new_arg = std::ffi::OsString::new();
    new_arg.push(start);
    new_arg.push(end.as_ref());
    new_arg
}

/// Replace cmd with new arguments. For more information see the [`Target Selection`]
///
/// [Target Selection]: https://android.googlesource.com/platform/ndk/+/master/docs/BuildSystemMaintainers.md#target-selection
pub fn new_ndk_quad_args(
    tool_root: std::path::PathBuf,
    build_target: &AndroidTarget,
    target_sdk_version: u32,
) -> crate::error::Result<Vec<std::ffi::OsString>> {
    let mut new_args = super::linker_args(&tool_root)?;
    #[cfg(target_os = "windows")]
    let ext = ".cmd";
    #[cfg(not(target_os = "windows"))]
    let ext = "";
    let linker_path = tool_root.join("bin").join(format!(
        "{}{}-clang{}",
        build_target.rust_triple(),
        target_sdk_version,
        ext,
    ));
    new_args.push(build_arg("-Clinker=", linker_path));
    Ok(new_args)
}

/// Replace libgcc file with unwind. libgcc was removed in ndk versions >=23.
/// This is workaround for gcc not found issue.
pub fn linker_args(tool_root: &std::path::Path) -> crate::error::Result<Vec<std::ffi::OsString>> {
    let mut new_args = Vec::new();
    let link_dir = tool_root.join("libgcc");

    std::fs::create_dir_all(&link_dir)?;
    std::fs::write(link_dir.join("libgcc.a"), "INPUT(-lunwind)")?;
    new_args.push(build_arg("-L", link_dir));

    Ok(new_args)
}
