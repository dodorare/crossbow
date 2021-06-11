use apple_bundle::prelude::*;

/// Generates minimal [`InfoPlist`](apple_bundle::prelude::InfoPlist) with given changes.
pub fn gen_minimal_info_plist(
    package_name: &String,
    app_name: Option<String>,
    version_name: String,
) -> InfoPlist {
    InfoPlist {
        localization: Localization {
            bundle_development_region: Some("en".to_owned()),
            ..Default::default()
        },
        launch: Launch {
            bundle_executable: Some(package_name.to_owned()),
            ..Default::default()
        },
        identification: Identification {
            bundle_identifier: format!("com.rust.{}", package_name),
            ..Default::default()
        },
        bundle_version: BundleVersion {
            bundle_version: Some(version_name.clone()),
            bundle_info_dictionary_version: Some(version_name.clone()),
            bundle_short_version_string: Some(version_name.clone()),
            ..Default::default()
        },
        naming: Naming {
            bundle_name: Some(app_name.unwrap_or(package_name.clone())),
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
    }
}
