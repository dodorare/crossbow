use crate::error::*;
use apple_bundle::{plist, prelude::*};
use std::fs::File;
use std::path::Path;

/// Saves given InfoPlist in new `Info.plist` file.
pub fn save_info_plist(out_dir: &Path, properties: &InfoPlist, binary: bool) -> Result<()> {
    // Create Info.plist file
    let file_path = out_dir.join("Info.plist");
    let file = File::create(file_path)?;
    // Write to Info.plist file
    match binary {
        true => plist::to_writer_binary(file, properties)?,
        false => plist::to_writer_xml(file, properties)?,
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(test)]
    pub const PLIST_TEST_EXAMPLE: &str = r#"<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
    <key>CFBundlePackageType</key>
    <string>APPL</string>
    <key>CFBundleIdentifier</key>
    <string>com.test.test-id</string>
    <key>CFBundleName</key>
    <string>Test</string>
    <key>CFBundleVersion</key>
    <string>1</string>
    <key>CFBundleShortVersionString</key>
    <string>1.0</string>
    <key>CFBundleInfoDictionaryVersion</key>
    <string>1.0</string>
    <key>CFBundleDevelopmentRegion</key>
    <string>en</string>
    <key>UILaunchStoryboardName</key>
    <string>LaunchScreen</string>
    <key>UISupportedInterfaceOrientations</key>
    <array>
        <string>UIInterfaceOrientationPortrait</string>
        <string>UIInterfaceOrientationPortraitUpsideDown</string>
        <string>UIInterfaceOrientationLandscapeLeft</string>
        <string>UIInterfaceOrientationLandscapeRight</string>
    </array>
    <key>UIRequiresFullScreen</key>
    <false/>
    <key>CFBundleExecutable</key>
    <string>test</string>
</dict>
</plist>"#;

    #[test]
    fn test_plist_equality() {
        let dir = tempfile::tempdir().unwrap();
        let properties = InfoPlist {
            localization: Localization {
                bundle_development_region: Some("en".to_owned()),
                ..Default::default()
            },
            launch: Launch {
                bundle_executable: Some("test".to_owned()),
                ..Default::default()
            },
            identification: Identification {
                bundle_identifier: "com.test.test-id".to_owned(),
                ..Default::default()
            },
            bundle_version: BundleVersion {
                bundle_version: Some("1".to_owned()),
                bundle_info_dictionary_version: Some("1.0".to_owned()),
                bundle_short_version_string: Some("1.0".to_owned()),
                ..Default::default()
            },
            naming: Naming {
                bundle_name: Some("Test".to_owned()),
                ..Default::default()
            },
            categorization: Categorization {
                bundle_package_type: Some("APPL".to_owned()),
                ..Default::default()
            },
            launch_interface: LaunchInterface {
                launch_storyboard_name: Some("LaunchScreen".to_owned()),
                ..Default::default()
            },
            styling: Styling {
                requires_full_screen: Some(false),
                ..Default::default()
            },
            orientation: Orientation {
                supported_interface_orientations: Some(vec![
                    InterfaceOrientation::Portrait,
                    InterfaceOrientation::PortraitUpsideDown,
                    InterfaceOrientation::LandscapeLeft,
                    InterfaceOrientation::LandscapeRight,
                ]),
                ..Default::default()
            },
            ..Default::default()
        };
        save_info_plist(dir.path(), &properties, false).unwrap();
        let file_path = dir.path().join("Info.plist");
        let result = std::fs::read_to_string(file_path).unwrap();
        assert_eq!(result, PLIST_TEST_EXAMPLE.replace("    ", "\t"));
        // TODO: Fix this. Should be equivalent
        // let got_props: InfoPlist = plist::from_file(&file_path).unwrap();
        // assert_eq!(properties, got_props);
    }
}
