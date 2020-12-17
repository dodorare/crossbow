use creator_build::*;

#[test]
fn test_compile_android() -> error::StdResult<()> {
    let dir = tempfile::tempdir()?;
    let generate_minimal_project = GenerateMinimalProject {
        out_dir: dir.path().to_owned().clone(),
    };
    let _name = generate_minimal_project.run((), ())?;
    // Run android rust compile
    let android_rust_compile = RustCompile {
        target: Target::Lib,
        build_target: AndroidTarget::Aarch64LinuxAndroid.into(),
        project_path: dir.path().to_owned(),
        release: true,
        cargo_args: vec![],
        crate_types: vec![CrateType::Cdylib],
    };
    android_rust_compile.run((), (None,))?;
    Ok(())
}

#[test]
fn test_compile_apple() -> error::StdResult<()> {
    let dir = tempfile::tempdir()?;
    let generate_minimal_project = GenerateMinimalProject {
        out_dir: dir.path().to_owned().clone(),
    };
    let name = generate_minimal_project.run((), ())?;
    // Run apple rust compile
    let apple_rust_compile = RustCompile {
        target: Target::Bin(name),
        build_target: AppleTarget::Aarch64AppleIos.into(),
        project_path: dir.path().to_owned(),
        release: true,
        cargo_args: vec![],
        crate_types: vec![],
    };
    apple_rust_compile.run((), (None,))?;
    Ok(())
}
