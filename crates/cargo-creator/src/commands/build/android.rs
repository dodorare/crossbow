use crate::*;
use clap::Clap;
use creator_tools::types::*;
use creator_tools::*;

#[derive(Clap)]
pub struct AndroidBuildCommand {
    /// Build profile. Can be one of: debug, release
    #[clap(short, long, default_value = "debug")]
    pub profile: Profile,
}

impl AndroidBuildCommand {
    pub fn run(&self) -> Result<()> {
        let path = std::env::current_dir().unwrap();
        let target_dir = path.join("..").join("..").join("target");
        let manifest = Manifest::from_path_with_metadata(path.join("Cargo.toml"))?;
        let package = manifest.package.unwrap();
        let metadata = package.metadata.unwrap().android;
        let name = package.name;

        println!("Metadata: {:?}", metadata);

        // Create dependencies
        let sdk = AndroidSdk::from_env().unwrap();
        let ndk = AndroidNdk::from_env(Some(sdk.sdk_path())).unwrap();

        // Compile rust lib for android
        let target_sdk_version = 30;
        let build_target = AndroidTarget::Aarch64LinuxAndroid;
        compile_rust_for_android(
            &ndk,
            build_target,
            &path,
            self.profile,
            vec![],
            target_sdk_version,
        )
        .unwrap();
        let out_dir = target_dir
            .join(build_target.rust_triple())
            .join(&self.profile);
        let compiled_lib = out_dir.join(format!("lib{}.so", name));
        println!("Compiled lib: {:?}", compiled_lib);
        assert!(compiled_lib.exists());

        // Gen android manifest
        let android_manifest = AndroidManifest {
            package_name: format!("rust.{}", name.replace("-", "_")),
            package_label: name.to_owned(),
            version_name: "1.2.3".to_owned(),
            version_code: VersionCode::from_semver("1.2.3").unwrap().to_code(1),
            split: None,
            target_name: name.replace("-", "_"),
            debuggable: false,
            target_sdk_version,
            min_sdk_version: 23,
            opengles_version: (3, 1),
            features: vec![],
            permissions: vec![],
            intent_filters: vec![],
            icon: None,
            fullscreen: false,
            orientation: None,
            application_metadatas: vec![],
            activity_metadatas: vec![],
        };
        let apk_build_dir = out_dir.join("apk");
        let manifest_path = gen_android_manifest(&apk_build_dir, &android_manifest).unwrap();
        assert!(manifest_path.exists());

        // Gen unaligned apk
        let unaligned_apk_path = gen_unaligned_apk(
            &sdk,
            &apk_build_dir,
            &manifest_path,
            None,
            None,
            &android_manifest,
        )
        .unwrap();
        assert!(unaligned_apk_path.exists());

        // Add all needed libs into apk
        add_libs_into_apk(
            &sdk,
            &ndk,
            &unaligned_apk_path,
            &compiled_lib,
            build_target,
            self.profile,
            23,
            &apk_build_dir,
            &target_dir,
        )
        .unwrap();

        // Align apk
        let aligned_apk_path = align_apk(
            &sdk,
            &unaligned_apk_path,
            &android_manifest.package_label,
            &apk_build_dir,
        )
        .unwrap();
        assert!(aligned_apk_path.exists());

        // Gen debug key for signing apk
        let key = gen_debug_key().unwrap();
        // Sign apk
        sign_apk(&sdk, &aligned_apk_path, key).unwrap();
        Ok(())
    }
}
