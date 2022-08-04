pub use apple_bundle;

use apple_bundle::prelude::*;

/// Updates [`InfoPlist`](InfoPlist) with default values.
pub fn update_info_plist_with_default(
    props: &mut InfoPlist,
    package_name: &str,
    app_name: Option<String>,
) {
    if props.launch.bundle_executable.is_none() {
        props.launch.bundle_executable = Some(package_name.to_owned());
    }
    if props.localization.bundle_development_region.is_none() {
        props.localization.bundle_development_region = Some("en".to_owned());
    }
    if props.identification.bundle_identifier.is_empty() {
        props.identification.bundle_identifier =
            format!("com.crossbow.{}", package_name.replace('-', "_"));
    }
    if props.bundle_version.bundle_version.is_none() {
        props.bundle_version.bundle_version = Some("0.1.0".to_owned());
    }
    if props
        .bundle_version
        .bundle_info_dictionary_version
        .is_none()
    {
        props.bundle_version.bundle_info_dictionary_version = Some("0.1.0".to_owned());
    }
    if props.bundle_version.bundle_short_version_string.is_none() {
        props.bundle_version.bundle_short_version_string = Some("0.1.0".to_owned());
    }
    if props.naming.bundle_name.is_none() {
        props.naming.bundle_name = Some(app_name.unwrap_or_else(|| package_name.to_owned()));
    }
    if props.categorization.bundle_package_type.is_none() {
        props.categorization.bundle_package_type = Some("APPL".to_owned());
    }
    if props.launch_interface.launch_storyboard_name.is_none() {
        props.launch_interface.launch_storyboard_name = Some("LaunchScreen".to_owned());
    }
}
