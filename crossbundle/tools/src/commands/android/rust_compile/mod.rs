mod compile_bevy;
mod compile_macroquad;
mod consts;

pub use compile_bevy::*;
pub use compile_macroquad::*;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{tools::*, types::*};

    #[test]
    fn test_compile_rust_with_macroquad() {
        // Specify path to user directory
        let user_dirs = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let project_path = user_dirs.parent().unwrap().parent().unwrap();

        // Specify path to macroquad project example
        let project_path = project_path.join("examples").join("macroquad-3d");

        // Provide path to Android SDK and Android NDK
        let sdk = AndroidSdk::from_env().unwrap();
        let ndk = AndroidNdk::from_env(Some(sdk.sdk_path())).unwrap();

        compile_rust_for_android_with_mq(
            &ndk,
            AndroidTarget::Aarch64LinuxAndroid,
            &project_path,
            Profile::Debug,
            vec![],
            false,
            false,
            30,
            "libbevy-2d.so",
        )
        .unwrap();
    }

    #[test]
    fn test_compile_rust_with_bevy() {
        // Specify path to user directory
        let user_dirs = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let project_path = user_dirs.parent().unwrap().parent().unwrap();

        // Specify path to macroquad project example
        let project_path = project_path.join("examples").join("bevy-2d");

        // Provide path to Android SDK and Android NDK
        let sdk = AndroidSdk::from_env().unwrap();
        let ndk = AndroidNdk::from_env(Some(sdk.sdk_path())).unwrap();

        compile_rust_for_android_with_bevy(
            &ndk,
            AndroidTarget::Aarch64LinuxAndroid,
            &project_path,
            Profile::Release,
            vec![],
            false,
            false,
            30,
            "libbevy-2d.so",
        )
        .unwrap();
    }
}
