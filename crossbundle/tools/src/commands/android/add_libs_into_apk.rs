use crate::{
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
/// Uses `readelf`, `aapt` tools
pub fn add_libs_into_apk(
    sdk: &AndroidSdk,
    ndk: &AndroidNdk,
    apk_path: &Path,
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
        &ndk.sysroot_lib_dir(&build_target)?.join("libc++_shared.so"),
        &system_libs,
        &dylibs_paths,
        &mut needed_libs,
    )?;
    let abi = build_target.android_abi();
    let out_dir = build_dir.join("libs").join(profile).join(abi);
    for (_lib_name, lib_path) in needed_libs {
        aapt_add_lib(sdk, apk_path, &lib_path, &out_dir, abi)?;
    }
    Ok(out_dir)
}

/// Copy lib into `out_dir` then add this lib into apk file
fn aapt_add_lib(
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
    let native_lib_path = apk_path.parent().unwrap().join("lib").join(abi);
    std::fs::create_dir_all(&native_lib_path)?;
    std::fs::copy(lib_path, &native_lib_path.join(&file_name))?;
    // `aapt a[dd] [-v] file.{zip,jar,apk} file1 [file2 ...]`
    // Add specified files to Zip-compatible archive
    let mut aapt = sdk.build_tool(bin!("aapt"), Some(apk_path.parent().unwrap()))?;
    aapt.arg("add")
        .arg(apk_path)
        .arg(format!("lib/{}/{}", abi, file_name.to_str().unwrap()));
    aapt.output_err(true)?;
    Ok(())
}

/// Search dylibs in given `deps_dir`
pub fn search_dylibs(deps_dir: &Path) -> Result<Vec<PathBuf>> {
    let mut paths = Vec::new();
    for dep_dir in deps_dir.read_dir()? {
        let output_file = dep_dir?.path().join("output");
        if output_file.is_file() {
            for line in BufReader::new(File::open(output_file)?).lines() {
                let line = line?;
                if let Some(link_search) = line.strip_prefix("cargo:rustc-link-search=") {
                    let mut pie = link_search.split('=');
                    let (kind, path) = match (pie.next(), pie.next()) {
                        (Some(kind), Some(path)) => (kind, path),
                        (Some(path), None) => ("all", path),
                        _ => unreachable!(),
                    };
                    match kind {
                        // FIXME: which kinds of search path we interested in
                        "dependency" | "native" | "all" => paths.push(path.into()),
                        _ => (),
                    };
                }
            }
        }
    }
    Ok(paths)
}

/// Update `needed_libs` hashset with given lib and all related libs.
/// Note: libc++ is not a system lib. If you use libc++_shared.so, it must be included in
/// your APK. https://developer.android.com/ndk/guides/cpp-support
pub fn recursively_define_needed_libs(
    (lib_name, lib_path): (String, PathBuf),
    readelf_path: &Path,
    libcpp_shared_path: &Path,
    system_libs: &[String],
    dylibs_paths: &[PathBuf],
    needed_libs: &mut Vec<(String, PathBuf)>,
) -> Result<()> {
    let shared_libs = readelf_list_shared_libs(readelf_path, &lib_path)?;
    needed_libs.push((lib_name, lib_path));
    for lib_name in shared_libs {
        if lib_name == "libc++_shared.so" {
            needed_libs.push((lib_name, libcpp_shared_path.to_owned()));
        } else if system_libs.contains(&lib_name) {
            continue;
        } else if !needed_libs.iter().any(|(name, _)| name == &lib_name) {
            if let Some(lib_path) = find_library_path(dylibs_paths, &lib_name)? {
                recursively_define_needed_libs(
                    (lib_name, lib_path),
                    readelf_path,
                    libcpp_shared_path,
                    system_libs,
                    dylibs_paths,
                    needed_libs,
                )?;
            } else {
                eprintln!("Shared library \"{}\" not found.", lib_name);
            }
        };
    }
    Ok(())
}

/// List all linked shared libraries
pub fn readelf_list_shared_libs(readelf_path: &Path, lib_path: &Path) -> Result<Vec<String>> {
    let mut readelf = std::process::Command::new(readelf_path);
    readelf.arg("-d").arg(lib_path);
    let output = readelf.output_err(false)?;
    let mut needed = Vec::new();
    for line in output.stdout.lines() {
        let line = line?;
        if line.contains("(NEEDED)") {
            let lib = line
                .split("Shared library: [")
                .last()
                .and_then(|line| line.split(']').next());
            if let Some(lib) = lib {
                needed.push(lib.to_owned());
            }
        }
    }
    Ok(needed)
}

/// Resolves native library using search paths
pub fn find_library_path<S: AsRef<Path>>(
    paths: &[PathBuf],
    lib_name: S,
) -> Result<Option<PathBuf>> {
    for path in paths {
        let lib_path = path.join(&lib_name);
        if lib_path.exists() {
            return Ok(Some(dunce::canonicalize(lib_path)?));
        }
    }
    Ok(None)
}

/// Return all files in directory with `.so` ending
pub fn get_libs_in_dir(dir: &Path) -> std::io::Result<Vec<String>> {
    let mut libs = Vec::new();
    if dir.is_dir() {
        for entry in std::fs::read_dir(dir)? {
            let entry = entry?;
            if !entry.path().is_dir() {
                if let Some(file_name) = entry.file_name().to_str() {
                    if file_name.ends_with(".so") {
                        libs.push(file_name.to_owned());
                    }
                }
            }
        }
    };
    Ok(libs)
}
