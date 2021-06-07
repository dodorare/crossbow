use std::{path::PathBuf};

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
    /// Specifies the output path for the linked resource APK.
    ///
    /// This is a required flag because you must specify the path for the output APK that
    /// can hold the linked resources.
    path: PathBuf,
    /// Specifies the path to the Android manifest file to build.
    ///
    /// This is a required flag because the manifest file encloses essential information
    /// about your app like package name and application ID.
    manifest_file: PathBuf,
    /// Provides the path to the platform's android.jar or other APKs like
    /// framework-res.apk  which might be useful while building features. This flag is
    /// required if you are using attributes with android namespace (for example,
    /// android:id) in your resource files.
    jar_path: bool,
    /// Specifies an assets directory to be included in the APK.
    ///
    /// You can use this directory to store original unprocessed files. To learn more,
    /// read [`Accessing original`] files.
    ///
    /// [`Accessing original`]: https://developer.android.com/guide/topics/resources/providing-resources#OriginalFiles
    directory_res_apk: Option<PathBuf>,
    /// Pass individual .flat file to link, using `overlay` semantics without using the
    /// `<add-resource>` tag.
    ///
    /// When you a provide a resource file that overlays (extends or modifies) an existing
    /// file, the last conflicting resource given is used.
    r_file: Option<PathBuf>,
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
    allow_reserved_package_id: Option<bool>,
    /// Specifies the directory in which to generate R.java.
    java_directory: Option<PathBuf>,
    /// Generates output file for ProGuard rules.
    proguard_options: Option<PathBuf>,
    /// Generates output file for ProGuard rules for the main dex.
    proguard_conditional_keep_rules: Option<bool>,
    /// Disables automatic style and layout SDK versioning.
    no_auto_version: Option<bool>,
    /// Disables automatic versioning of vector drawables. Use this only when building
    /// your APK with the Vector Drawable Library.
    no_version_vectors: Option<bool>,
    /// Disables automatic versioning of transition resources. Use this only when building
    /// your APK with Transition Support library.
    no_version_transitions: Option<bool>,
    /// Disables automatic de-duplication of resources with identical values across
    /// compatible configurations.
    no_resource_deduping: Option<bool>,
    /// Enables encoding of sparse entries using a binary search tree. This is useful for
    /// optimization of APK size, but at the cost of resource retrieval performance.
    enable_sparse_encoding: Option<bool>,
    /// Requires localization of strings marked 'suggested'.
    z: Option<bool>,
    /// Provides a list of configurations separated by commas.
    ///
    /// For example, if you have dependencies on the support library (which contains
    /// translations for multiple languages), you can filter resources just for the given
    /// language configuration, like English or Spanish.
    ///
    /// You must define the language configuration by a two-letter ISO 639-1 language
    /// code, optionally followed by a two letter ISO 3166-1-alpha-2 region code preceded
    /// by lowercase 'r' (for example, en-rUS).
    config: Option<Vec<String>>,
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
    output_to_dir: Option<bool>,
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
    proto_format: Option<bool>,
    /// Generates `R.java` with non-final resource IDs (references to the IDs from app’s
    /// code will not get inlined during kotlinc/javac compilation).
    non_final_ids: Option<bool>,
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
    /// Allows the addition of new resources in overlays without using the <add-resource> tag.
    auto_add_overlay: Option<bool>,
    /// Renames the package in AndroidManifest.xml.
    rename_manifest_package: Option<String>,
    /// Changes the name of the target package for [`instrumentation`].
    ///
    /// It should be used in conjunction with --rename-manifest-package.
    ///
    /// [`instrumentation`]: https://developer.android.com/reference/android/app/Instrumentation
    rename_instrumentation_target_package: Option<String>,
    /// Specifies the extensions of files that you do not want to compress.
    extension: Option<String>,
    /// Splits resources based on a set of configurations to generate a different version of the APK.
    ///
    /// You must specify the path to the output APK along with the set of configurations.
    split: Option<PathBuf>,
    /// Enables increased verbosity of the output.
    v: Option<bool>,
}

impl Aapt2Link {
    pub fn new() {}
}
