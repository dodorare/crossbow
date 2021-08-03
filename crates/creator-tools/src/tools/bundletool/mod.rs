mod build_apks;
mod build_bundle;
mod extract_apks;
mod get_size_total;

pub use build_apks::*;
pub use build_bundle::*;
pub use extract_apks::*;
pub use get_size_total::*;

use std::path::{Path, PathBuf};

#[derive(Clone, Copy)]
pub struct Bundletool;

impl Bundletool {
    pub fn build_apks(self, bundle: &Path, output: &Path) -> BuildApks {
        BuildApks::new(bundle, output)
    }

    pub fn build_bundle(self, modules: &[PathBuf], output: &Path) -> BuildBundle {
        BuildBundle::new(modules, output)
    }

    pub fn get_size_total(self, apks: &Path) -> GetSizeTotal {
        GetSizeTotal::new(apks)
    }

    pub fn extract_apks(self, apks: &Path, output_dir: &Path, device_spec: &Path) -> ExtractApks {
        ExtractApks::new(apks, output_dir, device_spec)
    }
}