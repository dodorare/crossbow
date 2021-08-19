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
    package_label: &str,
    zip_modules: &[PathBuf],
    project_path: &Path,
    build_dir: &Path,
) -> Result<PathBuf> {
    let aab = build_dir.join(format!("{}_unsigned.aab", package_label));
    BuildBundle::new(zip_modules, &aab);
    Ok(aab)
}
