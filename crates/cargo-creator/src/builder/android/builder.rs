use super::apk::builder::ApkBuilder;
use crate::builder::shared::load_cargo_manifest;
use crate::error::Error;

use std::path::PathBuf;

pub struct AndroidBuilder;

impl AndroidBuilder {
    // pub fn apk(self, cargo_manifest_path: Option<PathBuf>) -> Result<ApkBuilder, Error> {
    //     let manifest = load_cargo_manifest(cargo_manifest_path)?;
    //     // println!("{:#?}", manifest);
    //     ApkBuilder::from_cargo_manifest(manifest).map_err(From::from)
    // }
}
