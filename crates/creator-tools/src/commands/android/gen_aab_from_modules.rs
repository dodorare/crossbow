use crate::commands::android::{extract_apk, write_zip};
use crate::error::*;
use crate::tools::*;
use crate::{
    commands::{android, gen_minimal_project},
    tools::AndroidSdk,
    types::*,
};
use std::path::{Path, PathBuf};

pub fn gen_aab_from_modules(
    modules: &[PathBuf],
    project_path: &Path,
    build_dir: &Path,
) -> Result<PathBuf> {
    ///Bundletool::
    todo!();
}
