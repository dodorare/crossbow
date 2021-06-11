use std::{
    path::{Path, PathBuf},
    process::Command,
};

/// ## Link
/// In the link phase, AAPT2 merges all the intermediate files generated from the
/// compilation phase such as resource tables, binary XML files, and processed
/// PNG files and packages them into a single APK. Additionally, other auxiliary
/// files like `R.java` and ProGuard rules files can be generated during this phase.
/// However, the generated APK does not contain DEX bytecode and is unsigned.
/// That is, you can't deploy this APK to a device. If you're not using the Android
/// Gradle Plugin to [`build your app from the command line`], you can use other command
/// line tools, such as [`d8`] to compile Java bytecode into DEX bytecode and
/// [`apksigner`] to sign your APK.
///
/// ## Link syntax
/// The general syntax for using link is as follows:
///
/// ```
/// aapt2 link path-to-input-files [options] -o
/// outputdirectory/outputfilename.apk --manifest AndroidManifest.xml
/// ```
///
/// In the following example, AAPT2 merges the two intermediate files -
/// `drawable_Image.flat` and `values_values.arsc.flat`, and the `AndroidManifest.xml`
/// file. AAPT2 links the result against `android.jar` file which holds the resources
/// defined in the android package:
///
/// ```
///  aapt2 link -o output.apk
///  -I android_sdk/platforms/android_version/android.jar
///     compiled/res/values_values.arsc.flat
///     compiled/res/drawable_Image.flat --manifest /path/to/AndroidManifest.xml -v
/// ```
///
/// [`d8`]: https://developer.android.com/studio/command-line/d8
/// [`apksigner`]: https://developer.android.com/studio/command-line/apksigner
/// [`build your app from the command line`]: https://developer.android.com/studio/build/building-cmdline
#[derive(Debug, PartialEq)]
pub struct Aapt2Link {
    inputs: Vec<PathBuf>,
    /// Specifies the output path for the linked resource APK.
    ///
    /// This is a required flag because you must specify the path for the output APK that
    /// can hold the linked resources.
    o: PathBuf,
    /// Specifies the path to the Android manifest file to build.
    ///
    /// This is a required flag because the manifest file encloses essential information
    /// about your app like package name and application ID.
    manifest: PathBuf,
    /// Provides the path to the platform's android.jar or other APKs like
    /// framework-res.apk  which might be useful while building features. This flag is
    /// required if you are using attributes with android namespace (for example,
    /// android:id) in your resource files.
    i: Option<PathBuf>,
    /// Specifies an assets directory to be included in the APK.
    ///
    /// You can use this directory to store original unprocessed files. To learn more,
    /// read [`Accessing original`] files.
    ///
    /// [`Accessing original`]: https://developer.android.com/guide/topics/resources/providing-resources#OriginalFiles
    a: Option<PathBuf>,
    /// Pass individual .flat file to link, using `overlay` semantics without using the
    /// `<add-resource>` tag.
    ///
    /// When you a provide a resource file that overlays (extends or modifies) an existing
    /// file, the last conflicting resource given is used.
    r: Option<PathBuf>,
    /// Specifies the package ID to use for your app.
    ///
    /// The package ID that you specify must be greater than or equal to 0x7f unless used
    /// in combination with `--allow-reserved-package-id`.
    package_id: Option<String>,
    /// Allows the use of a reserved package ID.
    ///
    /// Reserved package IDs are IDs that are normally assigned to shared libraries and
    /// are in the range from 0x02 to 0x7e inclusive. By using
    /// --allow-reserved-package-id, you can assign IDs that fall in the range of reserved
    /// package IDs.
    ///
    /// This should only be used for packages with a min-sdk version of 26 or lower.
    allow_reserved_package_id: bool,
    /// Specifies the directory in which to generate R.java.
    java_directory: Option<PathBuf>,
    /// Generates output file for ProGuard rules.
    proguard_options: Option<PathBuf>,
    /// Generates output file for ProGuard rules for the main dex.
    proguard_conditional_keep_rules: bool,
    /// Disables automatic style and layout SDK versioning.
    no_auto_version: bool,
    /// Disables automatic versioning of vector drawables. Use this only when building
    /// your APK with the Vector Drawable Library.
    no_version_vectors: bool,
    /// Disables automatic versioning of transition resources. Use this only when building
    /// your APK with Transition Support library.
    no_version_transitions: bool,
    /// Disables automatic de-duplication of resources with identical values across
    /// compatible configurations.
    no_resource_deduping: bool,
    /// Enables encoding of sparse entries using a binary search tree. This is useful for
    /// optimization of APK size, but at the cost of resource retrieval performance.
    enable_sparse_encoding: bool,
    /// Requires localization of strings marked 'suggested'.
    z: bool,
    /// Provides a list of configurations separated by commas.
    ///
    /// For example, if you have dependencies on the support library (which contains
    /// translations for multiple languages), you can filter resources just for the given
    /// language configuration, like English or Spanish.
    ///
    /// You must define the language configuration by a two-letter ISO 639-1 language
    /// code, optionally followed by a two letter ISO 3166-1-alpha-2 region code preceded
    /// by lowercase 'r' (for example, en-rUS).
    config: Vec<String>,
    /// Allows AAPT2 to select the closest matching density and strip out all others.
    ///
    /// There are several pixel density qualifiers available to use in your app, such as
    /// ldpi, hdpi, and xhdpi. When you specify a preferred density, AAPT2 selects and
    /// stores the closest matching density in the resource table and removes all others.
    preferred_density: Option<i32>,
    /// Outputs the APK contents to a directory specified by -o.
    ///
    /// If you get any errors using this flag, you can resolve them by upgrading to
    /// [`Android SDK Build Tools 28.0.0 or higher`].
    ///
    /// [`Android SDK Build Tools 28.0.0 or higher`]: https://developer.android.com/studio/releases/build-tools
    output_to_dir: bool,
    /// Sets the default minimum SDK version to use for `AndroidManifest.xml`.
    min_sdk_version: Option<i32>,
    ///	Sets the default target SDK version to use for `AndroidManifest.xml`.
    target_sdk_version: Option<i32>,
    /// Specifies the version code (integer) to inject into the AndroidManifest.xml if
    /// none is present.
    version_code: Option<String>,
    /// Specifies the version name to inject into the AndroidManifest.xml if none is
    /// present.
    compile_sdk_version_name: Option<String>,
    /// Generates compiled resources in Protobuf format.
    /// Suitable as input to the [`bundle tool`] for generating an Android App Bundle.
    ///
    /// [`bundle tool`]: https://developer.android.com/studio/build/building-cmdline#bundletool-build
    proto_format: bool,
    /// Generates `R.java` with non-final resource IDs (references to the IDs from app’s
    /// code will not get inlined during kotlinc/javac compilation).
    non_final_ids: bool,
    /// Emits a file at the given path with a list of names of resource types and their ID
    /// mappings. It is suitable to use with --stable-ids.
    emit_ids: Option<PathBuf>,
    /// Consumes the file generated with --emit-ids containing the list of names of
    /// resource types and their assigned IDs.
    ///
    /// This option allows assigned IDs to remain stable even when you delete or add new
    /// resources while linking
    stable_ids: Option<PathBuf>,
    /// Specifies custom Java package under which to generate R.java.
    custom_package: Option<PathBuf>,
    /// Generates the same R.java file but with different package names.
    extra_packages: Option<PathBuf>,
    /// Adds a JavaDoc annotation to all generated Java classes.
    add_javadoc_annotation: Option<String>,
    /// Generates a text file containing the resource symbols of the R class in the
    /// specified file.
    ///
    /// You must specify the path to the output file.
    output_text_symbols: Option<PathBuf>,
    /// Allows the addition of new resources in overlays without using the <add-resource>
    /// tag.
    auto_add_overlay: bool,
    /// Renames the package in AndroidManifest.xml.
    rename_manifest_package: Option<String>,
    /// Changes the name of the target package for [`instrumentation`].
    ///
    /// It should be used in conjunction with --rename-manifest-package.
    ///
    /// [`instrumentation`]: https://developer.android.com/reference/android/app/Instrumentation
    rename_instrumentation_target_package: Option<String>,
    /// Specifies the extensions of files that you do not want to compress.
    extensions: Vec<String>,
    /// Splits resources based on a set of configurations to generate a different version
    /// of the APK.
    ///
    /// You must specify the path to the output APK along with the set of configurations.
    split: Option<PathBuf>,
    /// Enables increased verbosity of the output.
    v: bool,
}

impl Aapt2Link {
    pub fn new(inputs: &[PathBuf], o: &Path, manifest: &Path) -> Self {
        Self {
            inputs: inputs.to_vec(),
            o: o.to_owned(),
            manifest: manifest.to_owned(),
            i: None,
            a: None,
            r: None,
            package_id: None,
            allow_reserved_package_id: false,
            java_directory: None,
            proguard_options: None,
            proguard_conditional_keep_rules: false,
            no_auto_version: false,
            no_version_vectors: false,
            no_version_transitions: false,
            no_resource_deduping: false,
            enable_sparse_encoding: false,
            z: false,
            config: Vec::new(),
            preferred_density: None,
            output_to_dir: false,
            min_sdk_version: None,
            target_sdk_version: None,
            version_code: None,
            compile_sdk_version_name: None,
            proto_format: false,
            non_final_ids: false,
            emit_ids: None,
            stable_ids: None,
            custom_package: None,
            extra_packages: None,
            add_javadoc_annotation: None,
            output_text_symbols: None,
            auto_add_overlay: false,
            rename_manifest_package: None,
            rename_instrumentation_target_package: None,
            extensions: Vec::new(),
            split: None,
            v: false,
        }
    }

    pub fn i(&mut self, i: PathBuf) -> &mut Self {
        self.i = Some(i);
        self
    }

    pub fn a(&mut self, a: PathBuf) -> &mut Self {
        self.a = Some(a);
        self
    }

    pub fn r(&mut self, r: PathBuf) -> &mut Self {
        self.r = Some(r);
        self
    }

    pub fn package_id(&mut self, package_id: String) -> &mut Self {
        self.package_id = Some(package_id);
        self
    }

    pub fn allow_reserved_package_id(&mut self, allow_reserved_package_id: bool) -> &mut Self {
        self.allow_reserved_package_id = allow_reserved_package_id;
        self
    }

    pub fn java_directory(&mut self, java_directory: PathBuf) -> &mut Self {
        self.java_directory = Some(java_directory);
        self
    }

    pub fn proguard_options(&mut self, proguard_options: PathBuf) -> &mut Self {
        self.proguard_options = Some(proguard_options);
        self
    }

    pub fn proguard_conditional_keep_rules(
        &mut self,
        proguard_conditional_keep_rules: bool,
    ) -> &mut Self {
        self.proguard_conditional_keep_rules = proguard_conditional_keep_rules;
        self
    }

    pub fn no_auto_version(&mut self, no_auto_version: bool) -> &mut Self {
        self.no_auto_version = no_auto_version;
        self
    }

    pub fn no_version_vectors(&mut self, no_version_vectors: bool) -> &mut Self {
        self.no_version_vectors = no_version_vectors;
        self
    }

    pub fn no_version_transitions(&mut self, no_version_transitions: bool) -> &mut Self {
        self.no_version_transitions = no_version_transitions;
        self
    }

    pub fn no_resource_deduping(&mut self, no_resource_deduping: bool) -> &mut Self {
        self.no_resource_deduping = no_resource_deduping;
        self
    }

    pub fn enable_sparse_encoding(&mut self, enable_sparse_encoding: bool) -> &mut Self {
        self.enable_sparse_encoding = enable_sparse_encoding;
        self
    }

    pub fn z(&mut self, z: bool) -> &mut Self {
        self.z = z;
        self
    }

    pub fn config(&mut self, config: String) -> &mut Self {
        self.config.push(config);
        self
    }

    pub fn preferred_density(&mut self, preferred_density: i32) -> &mut Self {
        self.preferred_density = Some(preferred_density);
        self
    }

    pub fn output_to_dir(&mut self, output_to_dir: bool) -> &mut Self {
        self.output_to_dir = output_to_dir;
        self
    }

    pub fn min_sdk_version(&mut self, min_sdk_version: i32) -> &mut Self {
        self.min_sdk_version = Some(min_sdk_version);
        self
    }

    pub fn target_sdk_version(&mut self, target_sdk_version: i32) -> &mut Self {
        self.target_sdk_version = Some(target_sdk_version);
        self
    }

    pub fn version_code(&mut self, version_code: String) -> &mut Self {
        self.version_code = Some(version_code);
        self
    }

    pub fn compile_sdk_version_name(&mut self, compile_sdk_version_name: String) -> &mut Self {
        self.compile_sdk_version_name = Some(compile_sdk_version_name);
        self
    }

    pub fn proto_format(&mut self, proto_format: bool) -> &mut Self {
        self.proto_format = proto_format;
        self
    }

    pub fn non_final_ids(&mut self, non_final_ids: bool) -> &mut Self {
        self.non_final_ids = non_final_ids;
        self
    }

    pub fn emit_ids(&mut self, emit_ids: PathBuf) -> &mut Self {
        self.emit_ids = Some(emit_ids);
        self
    }

    pub fn stable_ids(&mut self, stable_ids: PathBuf) -> &mut Self {
        self.stable_ids = Some(stable_ids);
        self
    }

    pub fn custom_package(&mut self, custom_package: PathBuf) -> &mut Self {
        self.custom_package = Some(custom_package);
        self
    }

    pub fn extra_packages(&mut self, extra_packages: PathBuf) -> &mut Self {
        self.extra_packages = Some(extra_packages);
        self
    }

    pub fn add_javadoc_annotation(&mut self, add_javadoc_annotation: String) -> &mut Self {
        self.add_javadoc_annotation = Some(add_javadoc_annotation);
        self
    }

    pub fn output_text_symbols(&mut self, output_text_symbols: PathBuf) -> &mut Self {
        self.output_text_symbols = Some(output_text_symbols);
        self
    }

    pub fn auto_add_overlay(&mut self, auto_add_overlay: bool) -> &mut Self {
        self.auto_add_overlay = auto_add_overlay;
        self
    }

    pub fn rename_manifest_package(&mut self, rename_manifest_package: String) -> &mut Self {
        self.rename_manifest_package = Some(rename_manifest_package);
        self
    }

    pub fn rename_instrumentation_target_package(
        &mut self,
        rename_instrumentation_target_package: String,
    ) -> &mut Self {
        self.rename_instrumentation_target_package = Some(rename_instrumentation_target_package);
        self
    }

    pub fn extension(&mut self, extension: String) -> &mut Self {
        self.extensions.push(extension);
        self
    }

    pub fn split(&mut self, split: PathBuf) -> &mut Self {
        self.split = Some(split);
        self
    }

    pub fn v(&mut self, v: bool) -> &mut Self {
        self.v = v;
        self
    }

    pub fn run(&self) {
        let mut aapt2 = Command::new("aapt2");
        aapt2.arg("compile");
        self.inputs.iter().for_each(|input| {
            aapt2.arg(input);
        });
        aapt2.arg("-o").arg(&self.o);
        aapt2.arg("--manifest").arg(&self.manifest);
        if let Some(i) = &self.i {
            aapt2.arg("-I").arg(i);
        }
        if let Some(a) = &self.a {
            aapt2.arg("-A").arg(a);
        }
        if let Some(r) = &self.r {
            aapt2.arg("-R").arg(r);
        }
        if let Some(package_id) = &self.package_id {
            aapt2.arg("--package-id").arg(package_id);
        }
        if self.allow_reserved_package_id {
            aapt2.arg("--allow-reserved-package-id");
        }
        if let Some(java_directory) = &self.java_directory {
            aapt2.arg("--java").arg(java_directory);
        }
        if let Some(proguard_options) = &self.proguard_options {
            aapt2.arg("--proguard").arg(proguard_options);
        }
        if self.proguard_conditional_keep_rules {
            aapt2.arg("--proguard-conditional-keep-rules");
        }
        if self.no_auto_version {
            aapt2.arg("--no-auto-version");
        }
        if self.no_version_vectors {
            aapt2.arg("--no-version-vectors");
        }
        if self.no_version_transitions {
            aapt2.arg("--no-version-transitions");
        }
        if self.no_resource_deduping {
            aapt2.arg("--no-resource-deduping");
        }
        if self.enable_sparse_encoding {
            aapt2.arg("--enable-sparse-encoding");
        }
        if self.z {
            aapt2.arg("-z");
        }
        if self.config.len() > 0 {
            aapt2.arg("-c").arg(self.config.join(","));
        }
        if let Some(preferred_density) = self.preferred_density {
            aapt2
                .arg("--preferred-density")
                .arg(preferred_density.to_string());
        }
        if self.output_to_dir {
            aapt2.arg("--output-to-dir");
        }
        if let Some(min_sdk_version) = self.min_sdk_version {
            aapt2
                .arg("--min-sdk-version")
                .arg(min_sdk_version.to_string());
        }
        if let Some(target_sdk_version) = self.target_sdk_version {
            aapt2
                .arg("--target-sdk-version")
                .arg(target_sdk_version.to_string());
        }
        if let Some(version_code) = &self.version_code {
            aapt2.arg("--version-code").arg(version_code);
        }
        if let Some(compile_sdk_version_name) = &self.compile_sdk_version_name {
            aapt2
                .arg("--compile-sdk-version-name")
                .arg(compile_sdk_version_name);
        }
        if self.proto_format {
            aapt2.arg("--proto-format");
        }
        if self.non_final_ids {
            aapt2.arg("--non-final-ids");
        }
        if let Some(emit_ids) = &self.emit_ids {
            aapt2.arg("--emit-ids").arg(emit_ids);
        }
        if let Some(stable_ids) = &self.stable_ids {
            aapt2.arg("--stable-ids").arg(stable_ids);
        }
        if let Some(custom_package) = &self.custom_package {
            aapt2.arg("--custom-package").arg(custom_package);
        }
        if let Some(extra_packages) = &self.extra_packages {
            aapt2.arg("--extra-packages").arg(extra_packages);
        }
        if let Some(add_javadoc_annotation) = &self.add_javadoc_annotation {
            aapt2
                .arg("--add-javadoc-annotation")
                .arg(add_javadoc_annotation);
        }
        if let Some(output_text_symbols) = &self.output_text_symbols {
            aapt2.arg("--output-text-symbols").arg(output_text_symbols);
        }
        if self.auto_add_overlay {
            aapt2.arg("--auto-add-overlay");
        }
        if let Some(rename_manifest_package) = &self.rename_manifest_package {
            aapt2
                .arg("--rename-manifest-package")
                .arg(rename_manifest_package);
        }
        if let Some(rename_instrumentation_target_package) =
            &self.rename_instrumentation_target_package
        {
            aapt2
                .arg("--rename-instrumentation-target-package")
                .arg(rename_instrumentation_target_package);
        }
        self.extensions.iter().for_each(|extension| {
            aapt2.arg("-0").arg(extension);
        });
        if let Some(split) = &self.split {
            aapt2.arg("--split").arg(split);
        }
        if self.v {
            aapt2.arg("-v");
        }
        aapt2.output().expect("failed to execute process"); // TODO: Fix this expect
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builder_test() {
        let aapt2 = Aapt2Link::new(
            &[Path::new("bla/bla/bla").to_owned()],
            &Path::new("bla/bla/bla"),
            &Path::new("bla/bla/bla"),
        );
        aapt2.run();
    }
}
