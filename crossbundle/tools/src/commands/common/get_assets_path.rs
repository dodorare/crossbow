use std::path::PathBuf;

/// Workaround. Failed to get assets on windows from the .load() method through the
/// relative path to asset
pub fn get_assets_path(relative_path: PathBuf) -> PathBuf {
    let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let assets_path = manifest_dir.parent().unwrap().parent().unwrap().to_owned();
    let font_path = PathBuf::from(assets_path)
        .join("assets")
        .join(relative_path);
    font_path
}
