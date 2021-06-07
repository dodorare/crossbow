use std::path::PathBuf;
/// # Compile
/// AAPT2 supports compilation of all Android resource types, such as drawables and XML files.
/// When you invoke AAPT2 for compilation, you should pass a single resource file as an input per invocation.
/// AAPT2 then parses the file and generates an intermediate binary file with a .flat extension.
///
/// Although you can pass resource directories containing more than one resource files to AAPT2 using the --dir flag, you do not gain the benefits of incremental resource compilation when doing so.
/// That is, when passing whole directories, AAPT2 recompiles all files in the directory even when only one resource has changed.
///
/// The output file types can differ based on the input you provide for compilation.
/// The files AAPT2 outputs are not executables and you must later include these binary files as input in the link phase to generate an APK.
/// However, the generated APK file is not an executable that you can deploy on an Android device right away, as it does not contain DEX files (compiled bytecode) and is not signed.
///
/// ## Compile syntax
/// The general syntax for using compile is as follows:
///
/// ```
/// aapt2 compile path-to-input-files [options] -o output-directory/
/// ```
/// ### Note
/// For resource files, the path to input files must match the following structure:
/// ```
/// path/resource-type[-config]/file
/// ```
///
/// In the following example, AAPT2 compiles resource files named values.xml and myImage.png individually:
/// ```
/// aapt2 compile project_root/module_root/src/main/res/values-en/strings.xml -o compiled/
/// aapt2 compile project_root/module_root/src/main/res/drawable/myImage.png -o compiled/
/// ```
///
/// As shown in the table above, the name of the output file depends on the input file name and the name of its parent directory (the resource type and configuration).
/// For the example above with strings.xml as input, aapt2 automatically names the output file as values-en_strings.arsc.flat.
/// On the other hand, the file name for the compiled drawable file stored in the drawable directory will be drawable_img.png.flat.
///
/// ## [Compile options](https://developer.android.com/studio/command-line/aapt2#compile_options)
pub struct Aapt2Compile {
    /// Specifies the output path for the compiled resource(s).
    ///
    /// This is a required flag because you must specify a path to a directory where AAPT2 can output and store the compiled resources.
    path: PathBuf,
    /// Specifies the directory to scan for resources.
    ///
    /// Although you can use this flag to compile multiple resource files with one command, it disables the benefits of incremental compilation and thus, should not be used for large projects.
    directory: Option<PathBuf>,
    /// Generates pseudo-localized versions of default strings, such as en-XA and en-XB.
    pseudo_localize: bool,
    /// Disables PNG processing.
    ///
    /// Use this option if you have already processed the PNG files, or if you are creating debug builds that do not require file size reduction.
    /// Enabling this option results in a faster execution, but increases the output file size.
    no_crunch: bool,
    /// Treats errors that are permissible when using earlier versions of AAPT as warnings.
    ///
    /// This flag should be used for unexpected compile time errors.
    /// To resolve known behavior changes that you might get while using AAPT2, read
    /// [Behavior changes in AAPT2.](https://developer.android.com/studio/command-line/aapt2#aapt2_changes)
    legacy: bool,
    /// Enable verbose logging.
    v: bool,
}

impl Aapt2Compile {
    pub fn run(self) {
        todo!();
    }
}
