//! This file is workaround for this RustEmbed issue:
//! https://github.com/pyrossh/rust-embed/issues/144

use rust_embed::RustEmbed;

#[derive(RustEmbed)]
#[folder = "./java/app"]
#[include = "src/*"]
#[include = "*.xml"]
#[include = "*.gradle"]
#[exclude = "build/"]
#[exclude = "libs/"]
pub struct CrossbowAndroidAppTemplate;
