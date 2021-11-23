use crate::error::*;
use std::path::{Path, PathBuf};
use std::process::Command;

/// # Compile
/// AAPT2 supports compilation of all Android resource types, such as drawables and XML
/// files. When you invoke AAPT2 for compilation, you should pass a single resource file
/// as an input per invocation. AAPT2 then parses the file and generates an intermediate
/// binary file with a .flat extension.
///
/// Although you can pass resource directories containing more than one resource files to
/// AAPT2 using the --dir flag, you do not gain the benefits of incremental resource
/// compilation when doing so. That is, when passing whole directories, AAPT2 recompiles
/// all files in the directory even when only one resource has changed.
///
/// The output file types can differ based on the input you provide for compilation.
/// The files AAPT2 outputs are not executables and you must later include these binary
/// files as input in the link phase to generate an APK. However, the generated APK file
/// is not an executable that you can deploy on an Android device right away, as it does
/// not contain DEX files (compiled bytecode) and is not signed.
///
/// ## Compile syntax
/// The general syntax for using compile is as follows:
///
/// ```sh
/// aapt2 compile path-to-input-files [options] -o output-directory/
/// ```
/// ### Note
/// For resource files, the path to input files must match the following structure:
/// ```sh
/// path/resource-type[-config]/file
/// ```
///
/// In the following example, AAPT2 compiles resource files named values.xml and
/// myImage.png individually:
///
/// ```sh
/// aapt2 compile project_root/module_root/src/main/res/values-en/strings.xml -o compiled/
/// aapt2 compile project_root/module_root/src/main/res/drawable/myImage.png -o compiled/
/// ```
///
/// As shown in the table above, the name of the output file depends on the input file
/// name and the name of its parent directory (the resource type and configuration).
/// For the example above with strings.xml as input, aapt2 automatically names the output
/// file as values-en_strings.arsc.flat. On the other hand, the file name for the compiled
/// drawable file stored in the drawable directory will be drawable_img.png.flat.
///
/// ## [Compile options](https://developer.android.com/studio/command-line/aapt2#compile_options)
#[derive(Clone, Default)]
pub struct Aapt2Compile {
    res_path: Option<PathBuf>,
    compiled_res: PathBuf,
    res_dir: Option<PathBuf>,
    res_zip: Option<PathBuf>,
    output_text_symbols: Option<String>,
    pseudo_localize: bool,
    no_crunch: bool,
    legacy: bool,
    preserve_visibility_of_styleables: bool,
    visibility: Option<Visibility>,
    verbose: bool,
    trace_folder: Option<PathBuf>,
    help: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Visibility {
    Public,
    Private,
    Default,
}

impl std::fmt::Display for Visibility {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Self::Public => write!(f, "public"),
            Self::Private => write!(f, "private"),
            Self::Default => write!(f, "default"),
        }
    }
}

impl Aapt2Compile {
    /// Initialize struct Aapt2Compile then specifies input resource(s) to compile and
    /// specifies the output path for the compiled resource(s)
    pub fn new(res_path: &Path, compiled_res: &Path) -> Self {
        Self {
            res_path: Some(res_path.to_owned()),
            compiled_res: compiled_res.to_owned(),
            ..Default::default()
        }
    }

    /// Specifies the directory to scan for resources and specifies the output path for
    /// the compiled resource(s).
    ///
    /// Although you can use this flag to compile multiple resource files with one
    /// command, it disables the benefits of incremental compilation and thus, should not
    /// be used for large projects
    pub fn new_from_res_dir(res_dir: &Path, compiled_res: &Path) -> Self {
        Self {
            res_path: None,
            compiled_res: compiled_res.to_owned(),
            res_dir: Some(res_dir.to_owned()),
            ..Default::default()
        }
    }

    /// Specifies zip file containing the res directory to scan for resources and
    /// specifies the output path for the compiled resource(s)
    pub fn new_from_res_zip(res_zip: &Path, compiled_res: &Path) -> Self {
        Self {
            res_path: None,
            compiled_res: compiled_res.to_owned(),
            res_dir: None,
            res_zip: Some(res_zip.to_owned()),
            ..Default::default()
        }
    }

    /// Generates a text file containing the resource symbols in the specified file
    pub fn output_text_symbols(&mut self, output_text_symbols: String) -> &mut Self {
        self.output_text_symbols = Some(output_text_symbols);
        self
    }

    /// Generates pseudo-localized versions of default strings, such as en-XA and en-XB
    pub fn pseudo_localize(&mut self, pseudo_localize: bool) -> &mut Self {
        self.pseudo_localize = pseudo_localize;
        self
    }

    /// Disables PNG processing.
    ///
    /// Use this option if you have already processed the PNG files, or if you are
    /// creating debug builds that do not require file size reduction. Enabling this
    /// option results in a faster execution, but increases the output file size
    pub fn no_crunch(&mut self, no_crunch: bool) -> &mut Self {
        self.no_crunch = no_crunch;
        self
    }

    /// Treats errors that are permissible when using earlier versions of AAPT as
    /// warnings.
    ///
    /// This flag should be used for unexpected compile time errors.
    /// To resolve known behavior changes that you might get while using AAPT2, read
    /// [Behavior changes in AAPT2.](https://developer.android.com/studio/command-line/aapt2#aapt2_changes)
    pub fn legacy(&mut self, legacy: bool) -> &mut Self {
        self.legacy = legacy;
        self
    }

    /// If specified, apply the same visibility rules for styleables as are used for all
    /// other resources. Otherwise, all stylesables will be made public
    pub fn preserve_visibility_of_styleables(
        &mut self,
        preserve_visibility_of_styleables: bool,
    ) -> &mut Self {
        self.preserve_visibility_of_styleables = preserve_visibility_of_styleables;
        self
    }

    /// Sets the visibility of the compiled resources to the specified level.
    /// Accepted levels: public, private, default
    pub fn visibility(&mut self, visibility: Visibility) -> &mut Self {
        self.visibility = Some(visibility);
        self
    }

    /// Enable verbose logging
    pub fn verbose(&mut self, verbose: bool) -> &mut Self {
        self.verbose = verbose;
        self
    }

    /// Generate systrace json trace fragment to specified folder
    pub fn trace_folder(&mut self, trace_folder: &Path) -> &mut Self {
        self.trace_folder = Some(trace_folder.to_owned());
        self
    }

    /// Displays this help menu
    pub fn help(&mut self, help: bool) -> &mut Self {
        self.help = help;
        self
    }

    /// Opens the command line and launches aapt2 compile with arguments
    pub fn run(&self) -> Result<PathBuf> {
        let mut aapt2 = Command::new("aapt2");
        aapt2.arg("compile");
        if let Some(res_path) = &self.res_path {
            walkdir::WalkDir::new(res_path)
                .into_iter()
                .filter_map(|e| e.ok())
                .for_each(|input| {
                    if input.file_type().is_file() {
                        println!("{}", input.path().to_string_lossy());
                        aapt2.arg(input.path());
                    }
                });
        }
        aapt2.arg("-o");
        aapt2.arg(&self.compiled_res);
        if let Some(res_dir) = &self.res_dir {
            aapt2.arg("--dir").arg(res_dir);
        }
        if let Some(visibility) = &self.visibility {
            aapt2.arg("--visibility").arg(visibility.to_string());
        }
        if let Some(res_zip) = &self.res_zip {
            aapt2.arg("--zip").arg(res_zip);
        }
        if let Some(output_text_symbols) = &self.output_text_symbols {
            aapt2.arg("--output-text-symbols").arg(output_text_symbols);
        }
        if self.pseudo_localize {
            aapt2.arg("--pseudo-localize");
        }
        if self.no_crunch {
            aapt2.arg("--no-crunch");
        }
        if self.legacy {
            aapt2.arg("--legacy");
        }
        if self.preserve_visibility_of_styleables {
            aapt2.arg("--preserve-visibility-of-styleables");
        }
        if let Some(trace_folder) = &self.trace_folder {
            aapt2.arg("--trace-folder").arg(trace_folder);
        }
        if self.verbose {
            aapt2.arg("-v");
        }
        if self.help {
            aapt2.arg("-h");
        }
        aapt2.output_err(true)?;
        Ok(self.compiled_res.clone())
    }
}
