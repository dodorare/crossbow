use creator_build::*;

#[test]
fn test_command_run() -> error::StdResult<()> {
    let dir = tempfile::tempdir()?;
    let generate_minimal_project = GenerateMinimalProject {
        out_dir: dir.path().to_owned().clone(),
    };
    generate_minimal_project.run(())?;
    // Run android rust compile
    let android_rust_compile = RustCompile {
        target: BinOrLib::Lib,
        targets: AndroidOrAppleTargets::Android(vec![AndroidTarget::Aarch64LinuxAndroid]),
        project_path: dir.path().to_owned(),
        release: true,
        cargo_args: vec![],
        crate_types: vec![CrateType::Cdylib],
    };
    android_rust_compile.run(())?;
    Ok(())
}
