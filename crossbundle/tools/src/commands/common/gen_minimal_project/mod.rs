mod consts;
mod gen_minimal_project;

pub use consts::*;
pub use gen_minimal_project::*;

use std::{fs::File, io::Write};

/// Creates resource folder with string.xml resource inside to minimal project
pub fn create_res_folder(out_dir: &std::path::Path) -> crate::error::Result<()> {
    // Create res/values folder
    let res_path = out_dir.join("res").join("values");
    std::fs::create_dir_all(res_path.clone())?;
    // Create strings.xml
    let strings_xml_path = res_path.join("strings.xml");
    let mut strings_xml = File::create(strings_xml_path)?;
    strings_xml.write_all(STRINGS_XML_VALUE.as_bytes())?;
    Ok(())
}
