use creator_build::types::*;
use creator_build::*;
use creator_build::{AndroidNdk, AndroidSdk};

#[test]
fn test_compile_android() {
    let dir = tempfile::tempdir().unwrap();
    let generate_minimal_project = GenMinimalProject::new(dir.path().to_owned().clone());
    let _name = generate_minimal_project.run().unwrap();
    let sdk = AndroidSdk::init().unwrap();
    let ndk = AndroidNdk::init(Some(sdk.sdk_path())).unwrap();
    let android_rust_compile = AndroidRustCompile::new(
        ndk.clone(),
        AndroidTarget::Aarch64LinuxAndroid,
        dir.path().to_owned(),
        Profile::Release,
        vec![],
        30,
    );
    android_rust_compile.run().unwrap();
}

#[test]
#[cfg(target_os = "macos")]
fn test_compile_apple() {
    let dir = tempfile::tempdir().unwrap();
    let generate_minimal_project = GenMinimalProject::new(dir.path().to_owned().clone());
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
