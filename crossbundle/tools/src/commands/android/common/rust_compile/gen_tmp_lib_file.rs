use cargo::util::CargoResult;
use std::{fs, io::Write, path::Path};
use tempfile::{Builder, NamedTempFile};

/// Generate source file that will be built
pub fn generate_lib_file(path: &Path, extra_code: &'static str) -> CargoResult<NamedTempFile> {
    let original_src_filepath = path;

    // Determine the name of the temporary file
    let tmp_lib_file_prefix = format!(
        "__cargo_apk_{}",
        original_src_filepath
            .file_stem()
            .map(|s| s.to_string_lossy().into_owned())
            .unwrap_or_default()
    );

    // Create the temporary file
    let mut tmp_file = Builder::new()
        .prefix(&tmp_lib_file_prefix)
        .suffix(".tmp")
        .tempfile_in(original_src_filepath.parent().unwrap())?;

    let original_contents = fs::read_to_string(original_src_filepath)?;
    writeln!(tmp_file, "{}\n{}", original_contents, extra_code)?;
    Ok(tmp_file)
}
