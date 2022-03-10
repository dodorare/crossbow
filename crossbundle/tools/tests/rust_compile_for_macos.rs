#[cfg(target_os = "macos")]
use crossbundle_tools::{
    commands::{apple::*, gen_minimal_project},
    types::{AppleTarget, Profile, Target},
};

#[test]
#[cfg(target_os = "macos")]
fn test_compile_apple() {
    let tempdir = tempfile::tempdir().unwrap();
    let dir = tempdir.path();
    let name = gen_minimal_project(dir, false).unwrap();

    compile_rust_for_ios(
        Target::Bin(name),
        AppleTarget::Aarch64AppleIos,
        dir,
        Profile::Release,
        vec![],
        false,
        false,
    )
    .unwrap();
}
