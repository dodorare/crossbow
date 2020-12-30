use creator_tools::types::*;
use creator_tools::*;

#[test]
fn test_compile_android() {
    let tempdir = tempfile::tempdir().unwrap();
    let dir = tempdir.path();
    let _name = gen_minimal_project(dir).unwrap();

    let sdk = AndroidSdk::from_env().unwrap();
    let ndk = AndroidNdk::from_env(Some(sdk.sdk_path())).unwrap();
    compile_rust_for_android(
        &ndk,
        AndroidTarget::Aarch64LinuxAndroid,
        dir,
        Profile::Release,
        vec![],
        30,
    )
    .unwrap();
}

#[test]
#[cfg(target_os = "macos")]
fn test_compile_apple() {
    let dir = tempfile::tempdir().unwrap();
    let generate_minimal_project = GenMinimalProject::new(dir.path().to_owned());
    let name = generate_minimal_project.run().unwrap();
    let apple_rust_compile = AppleRustCompile::new(
        name,
        AppleTarget::Aarch64AppleIos,
        dir.path().to_owned(),
        Profile::Release,
        vec![],
    );
    apple_rust_compile.run().unwrap();
}
