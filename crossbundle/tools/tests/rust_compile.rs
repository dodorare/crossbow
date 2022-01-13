// #[cfg(target_os = "macos")]
// use crossbundle_tools::commands::apple::*;
// use crossbundle_tools::{
//     commands::{android::*, gen_minimal_project},
//     tools::{AndroidNdk, AndroidSdk},
//     types::*,
// };

// #[test]
// fn test_compile_android() {
//     let tempdir = tempfile::tempdir().unwrap();
//     let dir = tempdir.path();
//     let _name = gen_minimal_project(dir).unwrap();

//     let sdk = AndroidSdk::from_env().unwrap();
//     let ndk = AndroidNdk::from_env(Some(sdk.sdk_path())).unwrap();
//     compile_rust_for_android(
//         &ndk,
//         Target::Lib,
//         AndroidTarget::Aarch64LinuxAndroid,
//         dir,
//         Profile::Release,
//         vec![],
//         false,
//         false,
//         30,
//     )
//     .unwrap();
// }

// #[test]
// #[cfg(target_os = "macos")]
// fn test_compile_apple() {
//     let tempdir = tempfile::tempdir().unwrap();
//     let dir = tempdir.path();
//     let name = gen_minimal_project(dir).unwrap();

//     compile_rust_for_ios(
//         Target::Bin(name),
//         AppleTarget::Aarch64AppleIos,
//         dir,
//         Profile::Release,
//         vec![],
//         false,
//         false,
//     )
//     .unwrap();
// }
