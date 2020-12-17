use creator_build::*;

#[test]
fn test_compile_android() -> error::StdResult<()> {
    let dir = tempfile::tempdir()?;
    let generate_minimal_project = GenerateMinimalProject {
        out_dir: dir.path().to_owned().clone(),
    };
    generate_minimal_project.run((), ())?;
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
