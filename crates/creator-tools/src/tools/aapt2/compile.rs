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
/// ```
/// aapt2 compile path-to-input-files [options] -o output-directory/
/// ```
/// ### Note
/// For resource files, the path to input files must match the following structure:
/// ```
/// path/resource-type[-config]/file
/// ```
///
/// In the following example, AAPT2 compiles resource files named values.xml and
/// myImage.png individually: ```
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
pub struct Aapt2Compile {
    input: PathBuf,
    /// Specifies the output path for the compiled resource(s).
    ///
    /// This is a required flag because you must specify a path to a directory where AAPT2
    /// can output and store the compiled resources.
    o: PathBuf,
    /// Specifies the directory to scan for resources.
    ///
    /// Although you can use this flag to compile multiple resource files with one
    /// command, it disables the benefits of incremental compilation and thus, should not
    /// be used for large projects.
    dir: Option<PathBuf>,
    /// Generates pseudo-localized versions of default strings, such as en-XA and en-XB.
    pseudo_localize: bool,
    /// Disables PNG processing.
    ///
    /// Use this option if you have already processed the PNG files, or if you are
    /// creating debug builds that do not require file size reduction. Enabling this
    /// option results in a faster execution, but increases the output file size.
    no_crunch: bool,
    /// Treats errors that are permissible when using earlier versions of AAPT as
    /// warnings.
    ///
    /// This flag should be used for unexpected compile time errors.
    /// To resolve known behavior changes that you might get while using AAPT2, read
    /// [Behavior changes in AAPT2.](https://developer.android.com/studio/command-line/aapt2#aapt2_changes)
    legacy: bool,
    /// Enable verbose logging.
    v: bool,
    /// Displays this help menu
    h: bool,
}

impl Aapt2Compile {
    pub fn new(input: &Path, o: &Path) -> Self {
        Self {
            input: input.to_owned(),
            o: o.to_owned(),
            dir: None,
            pseudo_localize: false,
            no_crunch: false,
            legacy: false,
            v: false,
            h: false,
        }
    }

    pub fn dir(&mut self, dir: &Path) -> &mut Self {
        self.dir = Some(dir.to_owned());
        self
    }

    pub fn pseudo_localize(&mut self) -> &mut Self {
        self.pseudo_localize = true;
        self
    }

    pub fn no_crunch(&mut self) -> &mut Self {
        self.no_crunch = true;
        self
    }

    pub fn legacy(&mut self) -> &mut Self {
        self.legacy = true;
        self
    }

    pub fn v(&mut self) -> &mut Self {
        self.v = true;
        self
    }

    pub fn h(&mut self, h: bool) -> &mut Self {
        self.h = h;
        self
    }

    pub fn run(&self) -> Result<()> {
        let mut aapt2 = Command::new("aapt2");
        aapt2.arg("compile");
        aapt2.arg(&self.input);
        aapt2.arg("-o").arg(&self.o);
        if let Some(dir) = &self.dir {
            aapt2.arg("--dir").arg(dir);
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
        if self.v {
            aapt2.arg("-v");
        }
        if self.h {
            aapt2.arg("-h");
        }
        aapt2.output_err(true)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builder_test() {
        let mut aapt2 = Aapt2Compile::new(
            &Path::new("C:/Users/den99/AndroidStudioProjects/"),
            &Path::new("C:/Users/den99/AndroidStudioProjects/"),
        );
        aapt2.dir(&Path::new("C:/Users/den99/AndroidStudioProjects/"));
        aapt2.pseudo_localize();
        aapt2.run();
    }
}
