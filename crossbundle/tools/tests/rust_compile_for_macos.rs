#[cfg(target_os = "macos")]
use crossbundle_tools::{
    commands::{apple::*, gen_minimal_project},
    types::{IosTarget, Profile, Target},
};

#[test]
#[cfg(target_os = "macos")]
fn test_compile_apple() {
    let tempdir = tempfile::tempdir().unwrap();
    let dir = tempdir.path();
    let name = gen_minimal_project(dir, false, true).unwrap();

    compile_rust_for_ios(
        Target::Bin(name),
        IosTarget::Aarch64,
        dir,
        Profile::Release,
        vec![],
        false,
        false,
    )
    .unwrap();
}
