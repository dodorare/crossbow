use crate::error::*;
use serde::{Deserialize, Serialize};
use std::{
    fs::File,
    path::{Path, PathBuf},
};

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Xcent {
    #[serde(rename(serialize = "application-identifier"))]
    application_identifier: String,
    #[serde(rename(serialize = "com.apple.developer.team-identifier"))]
    team_identifier: String,
}

pub fn gen_xcent(
    app_path: &Path,
    project_name: &str,
    team_identifier: &str,
    app_bundle_identifier: &str,
    binary: bool,
) -> Result<PathBuf> {
    // Create project .xcent file
    let file_path = app_path.join(format!("{}.xcent", project_name));
    let file = File::create(&file_path)?;
    // Write to xcent file
    let xcent = Xcent {
        application_identifier: format!("{}.{}", team_identifier, app_bundle_identifier),
        team_identifier: team_identifier.to_owned(),
    };
    match binary {
        true => plist::to_writer_binary(file, &xcent)?,
        false => plist::to_writer_xml(file, &xcent)?,
    }
    Ok(file_path)
}
