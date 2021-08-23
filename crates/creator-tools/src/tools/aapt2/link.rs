use crate::error::{CommandExt, Result};
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
    output_apk: PathBuf,
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
    assets: Option<PathBuf>,
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
    java: Option<PathBuf>,
    /// Generates output file for ProGuard rules.
    proguard_options: Option<PathBuf>,
    /// Generates output file for ProGuard rules for the main dex.
    proguard_main_dex: Option<PathBuf>,
    /// Output file for generated Proguard rules for the main dex.
    proguard_conditional_keep_rules: bool,
    /// Generate a minimal set of Proguard keep rules.
    proguard_minimal_keep_rules: bool,
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
    /// Disables automatic removal of resources without
    no_resource_removal: bool,
    /// Enables encoding of sparse entries using a binary search tree. This is useful for
    /// optimization of APK size, but at the cost of resource retrieval performance.
    enable_sparse_encoding: bool,
    /// Legacy flag that specifies to use the package identifier 0x01.
    x: bool,
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
    /// Comma separated list of product names to keep
    product: Option<PathBuf>,
    /// Outputs the APK contents to a directory specified by -o.
    ///
    /// If you get any errors using this flag, you can resolve them by upgrading to
    /// [`Android SDK Build Tools 28.0.0 or higher`].
    ///
    /// [`Android SDK Build Tools 28.0.0 or higher`]: https://developer.android.com/studio/releases/build-tools
    output_to_dir: bool,
    /// Removes XML namespace prefix and URI information
    no_xml_namespaces: bool,
    /// Sets the default minimum SDK version to use for `AndroidManifest.xml`.
    min_sdk_version: Option<u32>,
    ///	Sets the default target SDK version to use for `AndroidManifest.xml`.
    target_sdk_version: Option<u32>,
    /// Specifies the version code (integer) to inject into the AndroidManifest.xml if
    /// none is present.
    version_code: Option<u32>,
    /// Version code major (integer) to inject into the AndroidManifest.xml if none is
    /// present.
    version_code_major: Option<u32>,
    /// Version name to inject into the AndroidManifest.xml if none is present.
    version_name: Option<String>,
    /// If --version-code and/or --version-name are specified, these values will replace
    /// any value already in the manifest. By default, nothing is changed if the manifest
    /// already defines these attributes.
    replace_version: bool,
    ///  Version code (integer) to inject into the AndroidManifest.xml if none is present.
    compile_sdk_version_code: Option<u32>,
    /// Specifies the version name to inject into the AndroidManifest.xml if none is
    /// present.
    compile_sdk_version_name: Option<String>,
    /// Generates a shared Android runtime library.
    shared_lib: bool,
    /// Generate a static Android library.
    static_lib: bool,
    /// Generates compiled resources in Protobuf format.
    /// Suitable as input to the [`bundle tool`] for generating an Android App Bundle.
    ///
    /// [`bundle tool`]: https://developer.android.com/studio/build/building-cmdline#bundletool-build
    proto_format: bool,
    /// Merge all library resources under the app's package.
    no_static_lib_packages: bool,
    /// Generates `R.java` with non-final resource IDs (references to the IDs from app’s
    /// code will not get inlined during kotlinc/javac compilation).
    non_final_ids: bool,
    /// Keep proguard rules files from having a reference to the source file
    no_proguard_location_reference: bool,
    /// Emits a file at the given path with a list of names of resource types and their ID
    /// mappings. It is suitable to use with --stable-ids.
    emit_ids: Option<PathBuf>,
    /// Consumes the file generated with --emit-ids containing the list of names of
    /// resource types and their assigned IDs.
    ///
    /// This option allows assigned IDs to remain stable even when you delete or add new
    /// resources while linking
    stable_ids: Option<PathBuf>,
    /// Package name to use when generating R.java for private symbols. If not specified,
    /// public and private symbols will use the application's package name.
    private_symbols: Option<String>,
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
    /// Causes styles defined in -R resources to replace previous definitions instead of
    /// merging into them
    override_styles_instead_of_overlaying: bool,
    /// Renames the package in AndroidManifest.xml.
    rename_manifest_package: Option<String>,
    /// Renames the package in resources table
    rename_resources_package: Option<String>,
    /// Changes the name of the target package for [`instrumentation`].
    ///
    /// It should be used in conjunction with --rename-manifest-package.
    ///
    /// [`instrumentation`]: https://developer.android.com/reference/android/app/Instrumentation
    rename_instrumentation_target_package: Option<String>,
    /// Specifies the extensions of files that you do not want to compress.
    extensions: Vec<String>,
    /// Do not compress any resources.
    no_compress: bool,
    /// Preserve raw attribute values in xml files.
    keep_raw_values: bool,
    /// Do not compress extensions matching the regular expression. Remember to use the
    /// '$' symbol for end of line. Uses a case-sensitive ECMAScriptregular expression
    /// grammar.
    no_compress_regex: Option<String>,
    /// Treat manifest validation errors as warnings.
    warn_manifest_validation: bool,
    /// Splits resources based on a set of configurations to generate a different version
    /// of the APK.
    ///
    /// You must specify the path to the output APK along with the set of configurations.
    split: Option<PathBuf>,
    /// Do not allow overlays with different visibility levels.
    strict_visibility: bool,
    /// Do not serialize source file information when generating resources in Protobuf
    /// format.
    exclude_sources: bool,
    /// Generate systrace json trace fragment to specified folder.
    trace_folder: Option<String>,
    /// Only merge the resources, without verifying resource references. This flag should
    /// only be used together with the --static-lib flag.
    merge_only: bool,
    /// Enables increased verbosity of the output.
    v: bool,
    /// Displays this help menu
    h: bool,
}

impl Aapt2Link {
    pub fn new(inputs: &[PathBuf], output_apk: PathBuf, manifest: &Path) -> Self {
        Self {
            inputs: inputs.to_vec(),
            output_apk: output_apk.to_owned(),
            manifest: manifest.to_owned(),
            i: None,
            assets: None,
            r: None,
            package_id: None,
            allow_reserved_package_id: false,
            java: None,
            proguard_options: None,
            proguard_main_dex: None,
            proguard_minimal_keep_rules: false,
            proguard_conditional_keep_rules: false,
            no_auto_version: false,
            no_version_vectors: false,
            no_version_transitions: false,
            no_resource_deduping: false,
            no_resource_removal: false,
            enable_sparse_encoding: false,
            x: false,
            z: false,
            config: Vec::new(),
            preferred_density: None,
            product: None,
            output_to_dir: false,
            no_xml_namespaces: false,
            min_sdk_version: None,
            target_sdk_version: None,
            version_code: None,
            version_code_major: None,
            version_name: None,
            replace_version: false,
            compile_sdk_version_code: None,
            compile_sdk_version_name: None,
            shared_lib: false,
            static_lib: false,
            proto_format: false,
            no_static_lib_packages: false,
            non_final_ids: false,
            no_proguard_location_reference: false,
            emit_ids: None,
            stable_ids: None,
            private_symbols: None,
            custom_package: None,
            extra_packages: None,
            add_javadoc_annotation: None,
            output_text_symbols: None,
            auto_add_overlay: false,
            override_styles_instead_of_overlaying: false,
            rename_manifest_package: None,
            rename_resources_package: None,
            rename_instrumentation_target_package: None,
            extensions: Vec::new(),
            no_compress: false,
            keep_raw_values: false,
            no_compress_regex: None,
            warn_manifest_validation: false,
            split: None,
            strict_visibility: false,
            exclude_sources: false,
            trace_folder: None,
            merge_only: false,
            v: false,
            h: false,
        }
    }

    pub fn proguard_main_dex(&mut self, proguard_main_dex: PathBuf) -> &mut Self {
        self.proguard_main_dex = Some(proguard_main_dex);
        self
    }

    pub fn proguard_minimal_keep_rules(&mut self, proguard_minimal_keep_rules: bool) -> &mut Self {
        self.proguard_minimal_keep_rules = proguard_minimal_keep_rules;
        self
    }

    pub fn no_resource_removal(&mut self, no_resource_removal: bool) -> &mut Self {
        self.no_resource_removal = no_resource_removal;
        self
    }

    pub fn x(&mut self, x: bool) -> &mut Self {
        self.x = x;
        self
    }

    pub fn product(&mut self, product: PathBuf) -> &mut Self {
        self.product = Some(product);
        self
    }

    pub fn no_xml_namespaces(&mut self, no_xml_namespaces: bool) -> &mut Self {
        self.no_xml_namespaces = no_xml_namespaces;
        self
    }

    pub fn version_code_major(&mut self, version_code_major: u32) -> &mut Self {
        self.version_code_major = Some(version_code_major);
        self
    }

    pub fn version_name(&mut self, version_name: String) -> &mut Self {
        self.version_name = Some(version_name);
        self
    }

    pub fn replace_version(&mut self, replace_version: bool) -> &mut Self {
        self.replace_version = replace_version;
        self
    }

    pub fn compile_sdk_version_code(&mut self, compile_sdk_version_code: u32) -> &mut Self {
        self.compile_sdk_version_code = Some(compile_sdk_version_code);
        self
    }

    pub fn shared_lib(&mut self, shared_lib: bool) -> &mut Self {
        self.shared_lib = shared_lib;
        self
    }

    pub fn static_lib(&mut self, static_lib: bool) -> &mut Self {
        self.static_lib = static_lib;
        self
    }

    pub fn no_static_lib_packages(&mut self, no_static_lib_packages: bool) -> &mut Self {
        self.no_static_lib_packages = no_static_lib_packages;
        self
    }

    pub fn no_proguard_location_reference(
        &mut self,
        no_proguard_location_reference: bool,
    ) -> &mut Self {
        self.no_proguard_location_reference = no_proguard_location_reference;
        self
    }

    pub fn private_symbols(&mut self, private_symbols: String) -> &mut Self {
        self.private_symbols = Some(private_symbols);
        self
    }

    pub fn override_styles_instead_of_overlaying(
        &mut self,
        override_styles_instead_of_overlaying: bool,
    ) -> &mut Self {
        self.override_styles_instead_of_overlaying = override_styles_instead_of_overlaying;
        self
    }

    pub fn rename_resources_package(&mut self, rename_resources_package: String) -> &mut Self {
        self.rename_resources_package = Some(rename_resources_package);
        self
    }

    pub fn i(&mut self, i: PathBuf) -> &mut Self {
        self.i = Some(i);
        self
    }

    pub fn assets(&mut self, assets: PathBuf) -> &mut Self {
        self.assets = Some(assets);
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

    pub fn java(&mut self, java: PathBuf) -> &mut Self {
        self.java = Some(java);
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

    pub fn min_sdk_version(&mut self, min_sdk_version: u32) -> &mut Self {
        self.min_sdk_version = Some(min_sdk_version);
        self
    }

    pub fn target_sdk_version(&mut self, target_sdk_version: u32) -> &mut Self {
        self.target_sdk_version = Some(target_sdk_version);
        self
    }

    pub fn version_code(&mut self, version_code: u32) -> &mut Self {
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

    pub fn no_compress(&mut self, no_compress: bool) -> &mut Self {
        self.no_compress = no_compress;
        self
    }

    pub fn keep_raw_values(&mut self, keep_raw_values: bool) -> &mut Self {
        self.keep_raw_values = keep_raw_values;
        self
    }

    pub fn extension(&mut self, extension: String) -> &mut Self {
        self.extensions.push(extension);
        self
    }
    pub fn no_compress_regex(&mut self, no_compress_regex: String) -> &mut Self {
        self.no_compress_regex = Some(no_compress_regex);
        self
    }

    pub fn warn_manifest_validation(&mut self, warn_manifest_validation: bool) -> &mut Self {
        self.warn_manifest_validation = warn_manifest_validation;
        self
    }

    pub fn split(&mut self, split: PathBuf) -> &mut Self {
        self.split = Some(split);
        self
    }
    pub fn strict_visibility(&mut self, strict_visibility: bool) -> &mut Self {
        self.strict_visibility = strict_visibility;
        self
    }

    pub fn trace_folder(&mut self, trace_folder: String) -> &mut Self {
        self.trace_folder = Some(trace_folder);
        self
    }

    pub fn exclude_sources(&mut self, exclude_sources: bool) -> &mut Self {
        self.exclude_sources = exclude_sources;
        self
    }

    pub fn merge_only(&mut self, merge_only: bool) -> &mut Self {
        self.merge_only = merge_only;
        self
    }

    pub fn v(&mut self, v: bool) -> &mut Self {
        self.v = v;
        self
    }

    pub fn h(&mut self, h: bool) -> &mut Self {
        self.h = h;
        self
    }

    pub fn run(&self) -> Result<()> {
        let mut aapt2 = Command::new("aapt2");
        aapt2.arg("link");
        self.inputs.iter().for_each(|input| {
            aapt2.arg(input);
        });
        aapt2.arg("-o").arg(&self.output_apk);
        aapt2.arg("--manifest").arg(&self.manifest);
        if let Some(i) = &self.i {
            aapt2.arg("-I").arg(i);
        }
        if let Some(assets) = &self.assets {
            aapt2.arg("-A").arg(assets);
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
        if let Some(java) = &self.java {
            aapt2.arg("--java").arg(java);
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
            aapt2.arg("--version-code").arg(version_code.to_string());
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
        if self.h {
            aapt2.arg("-h");
        }
        if let Some(proguard_main_dex) = &self.proguard_main_dex {
            aapt2.arg("--proguard-main-dex").arg(proguard_main_dex);
        }
        if self.proguard_minimal_keep_rules {
            aapt2.arg("--proguard-minimal-keep-rules");
        }
        if self.no_resource_removal {
            aapt2.arg("--no-resource-removal");
        }
        if self.x {
            aapt2.arg("-x");
        }
        if let Some(product) = &self.product {
            aapt2.arg("--product").arg(product);
        }
        if self.no_xml_namespaces {
            aapt2.arg("--no-xml-namespaces");
        }
        if let Some(version_code_major) = &self.version_code_major {
            aapt2
                .arg("--version-code-major")
                .arg(version_code_major.to_string());
        }
        if let Some(version_name) = &self.version_name {
            aapt2.arg("--version-name").arg(version_name);
        }
        if self.replace_version {
            aapt2.arg("--replace-version");
        }
        if let Some(compile_sdk_version_code) = &self.compile_sdk_version_code {
            aapt2
                .arg("--compile-sdk-version-code")
                .arg(compile_sdk_version_code.to_string());
        }
        if self.shared_lib {
            aapt2.arg("--shared-lib");
        }
        if self.static_lib {
            aapt2.arg("--static-lib");
        }
        if self.no_static_lib_packages {
            aapt2.arg("--no-static-lib-packages");
        }
        if self.no_proguard_location_reference {
            aapt2.arg("--no-proguard-location-reference");
        }
        if let Some(private_symbols) = &self.private_symbols {
            aapt2.arg("--private-symbols").arg(private_symbols);
        }
        if self.override_styles_instead_of_overlaying {
            aapt2.arg("--override-styles-instead-of-overlaying");
        }
        if let Some(rename_resources_package) = &self.rename_resources_package {
            aapt2
                .arg("--rename-resources-package")
                .arg(rename_resources_package);
        }
        if self.no_compress {
            aapt2.arg("--no-compress");
        }
        if self.keep_raw_values {
            aapt2.arg("--keep-raw-values");
        }
        if let Some(no_compress_regex) = &self.no_compress_regex {
            aapt2.arg("--no-compress-regex").arg(no_compress_regex);
        }
        if self.warn_manifest_validation {
            aapt2.arg("--warn-manifest-validation");
        }
        if self.strict_visibility {
            aapt2.arg("--strict-visibility");
        }
        if self.exclude_sources {
            aapt2.arg("--exclude-sources");
        }
        if let Some(trace_folder) = &self.trace_folder {
            aapt2.arg("--trace-folder").arg(trace_folder);
        }
        if self.merge_only {
            aapt2.arg("--merge-only");
        }
        aapt2.output_err(true)?;
        Ok(())
    }
}
