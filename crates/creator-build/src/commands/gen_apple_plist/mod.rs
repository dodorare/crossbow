use super::Command;
use crate::{error::*, types::*};
use std::fs::File;
use std::path::PathBuf;

pub struct GenApplePlist {
    pub out_dir: PathBuf,
    pub properties: InfoPlist,
    pub binary: bool,
}

impl Command for GenApplePlist {
    type Deps = ();
    type Output = ();

    fn run(&self) -> Result<()> {
        // Create Info.plist file
        let file_path = self.out_dir.join("Info.plist");
        let file = File::create(file_path)?;
        // Write to Info.plist file
        match self.binary {
            true => plist::to_writer_binary(file, &self.properties)?,
            false => plist::to_writer_xml(file, &self.properties)?,
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_command_run() {
        let dir = tempfile::tempdir().unwrap();
        let cmd = GenApplePlist {
            out_dir: dir.path().to_owned(),
            properties: InfoPlist {
                categorization: Categorization {
                    bundle_package_type: None,
                    application_category_type: Some(AppCategoryType::Business),
                },
                orientation: Orientation {
                    interface_orientation: None,
                    supported_interface_orientations: Some(vec![InterfaceOrientation::Portrait]),
                },
                ..Default::default()
            },
            binary: false,
        };
        cmd.run().unwrap();
        let file_path = dir.path().join("Info.plist");
        let result = std::fs::read_to_string(&file_path).unwrap();
        println!("{}", result);
        let properties: InfoPlist = plist::from_file(&file_path).unwrap();
        println!("{:?}", properties);
    }
}
