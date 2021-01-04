use super::{BuildContext, SharedBuildCommand};
use crate::*;
use clap::Clap;
use creator_tools::types::*;
use creator_tools::*;
use std::path::PathBuf;

#[derive(Clap)]
pub struct AndroidBuildCommand {
    #[clap(flatten)]
    pub shared: SharedBuildCommand,
    /// Build for the given android architecture. Supported targets are: `armv7-linux-androideabi`,
    /// `aarch64-linux-android`, `i686-linux-android`, `x86_64-linux-android`
    #[clap(long, default_value = "aarch64-linux-android")]
    pub target: Vec<AndroidTarget>,
}

impl AndroidBuildCommand {
    pub fn run(&self, current_dir: PathBuf) -> Result<()> {
        let build_context = BuildContext::init(&current_dir, self.shared.target_dir.clone())?;
        self.execute(&build_context)?;
        Ok(())
    }

    pub fn execute(
        &self,
        build_context: &BuildContext,
    ) -> Result<(String, AndroidSdk, AndroidMetadata, PathBuf)> {
        log::info!("Starting build process");
        let package = build_context
            .manifest
            .package
            .as_ref()
            .ok_or(Error::InvalidManifest)?;
        let metadata = package
            .metadata
            .clone()
            .ok_or(Error::InvalidManifestMetadata)?
            .android;
        let project_path = build_context.project_path.clone();
        let target_dir = build_context.target_dir.clone();
        let profile = match self.shared.release {
            true => Profile::Release,
            false => Profile::Debug,
        };
        let package_name;
        let target = if let Some(example) = self.shared.example.clone() {
            package_name = example.clone();
            Target::Example(example)
        } else {
            package_name = package.name.clone();
            Target::Lib
        };
        // Create dependencies
        let sdk = AndroidSdk::from_env().unwrap();
        let ndk = AndroidNdk::from_env(Some(sdk.sdk_path())).unwrap();
        let target_sdk_version = metadata
            .manifest
            .target_sdk_version
            .unwrap_or_else(|| sdk.default_platform());

        // Compile rust libs for android
        let mut compiled_libs = Vec::new();
        for build_target in self.target.iter() {
            compile_rust_for_android(
                &ndk,
                target.clone(),
                *build_target,
                &project_path,
                profile,
                self.shared.features.clone(),
                self.shared.all_features,
                self.shared.no_default_features,
                target_sdk_version,
            )
            .unwrap();
            let out_dir = target_dir.join(build_target.rust_triple()).join(&profile);
            let compiled_lib = out_dir.join(format!("lib{}.so", package_name));
            compiled_libs.push((compiled_lib, build_target))
        }

        // Gen android manifest
        let android_manifest = AndroidManifest {
            package_name: format!("rust.{}", package_name.replace("-", "_")),
            package_label: package_name.to_owned(),
            version_name: "1.2.3".to_owned(),
            version_code: VersionCode::from_semver("1.2.3").unwrap().to_code(1),
            split: None,
            target_name: package_name.replace("-", "_"),
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
        let apk_build_dir = target_dir.join("android").join(&profile);
        let manifest_path = gen_android_manifest(&apk_build_dir, &android_manifest).unwrap();

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

        // For every compiled target lib add all needed libs into apk
        for (compiled_lib, build_target) in compiled_libs {
            add_libs_into_apk(
                &sdk,
                &ndk,
                &unaligned_apk_path,
                &compiled_lib,
                *build_target,
                profile,
                metadata.manifest.min_sdk_version.unwrap_or(23),
                &apk_build_dir,
                &target_dir,
            )
            .unwrap();
        }

        // Align apk
        let aligned_apk_path = align_apk(
            &sdk,
            &unaligned_apk_path,
            &android_manifest.package_label,
            &apk_build_dir,
        )
        .unwrap();

        // Gen debug key for signing apk
        let key = gen_debug_key().unwrap();
        // Sign apk
        sign_apk(&sdk, &aligned_apk_path, key).unwrap();
        Ok((package_name, sdk, metadata, aligned_apk_path))
    }
}
