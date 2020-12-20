/// Information Property List.
///
/// A resource containing key-value pairs that identify and configure a bundle.
///
/// Bundles, which represent executables of different kinds, contain an information property list file. This collection
/// of key-value pairs specifies how the system should interpret the associated bundle. Some key-value pairs characterize
/// the bundle itself, while others configure the app, framework, or other entity that the bundle represents. Some keys are
/// required, while others are specific to particular features of the executable.
///
/// The information property list file always has the name Info.plist. The file name is case-sensitive and must begin with
/// a capital letter I. Its location within the bundle depends on both the bundle type and the platform. For example, iOS
/// app bundles store the file in the bundle’s root directory, whereas macOS app bundles place the Info.plist file in the
/// Contents directory.
///
/// To access an information property list, you use an instance of the Bundle class, which represents a bundle on disk.
/// You can get the value for a few common keys by accessing properties of the bundle instance. For example, the bundleIdentifier
/// property contains the value associated with the CFBundleIdentifier key. You can obtain the value for an arbitrary key using
/// the object(forInfoDictionaryKey:) method.
///
/// Official documentation: https://developer.apple.com/documentation/bundleresources/information_property_list
///
mod app_execution;
mod bundle_configuration;
mod user_interface;

pub use app_execution::*;
pub use bundle_configuration::*;
pub use user_interface::*;

use serde::{ser::SerializeSeq, Deserialize, Serialize, Serializer};

fn serialize_enum_option<S: Serializer, T: Serialize>(
    value: &Option<T>,
    s: S,
) -> Result<S::Ok, S::Error> {
    s.serialize_str(&serde_plain::to_string(value).unwrap())
}

fn serialize_vec_enum_option<S: Serializer, T: Serialize>(
    value: &Option<Vec<T>>,
    s: S,
) -> Result<S::Ok, S::Error> {
    match value {
        Some(ref val) => {
            let mut seq = s.serialize_seq(Some(val.len()))?;
            for element in val.iter() {
                seq.serialize_element(&serde_plain::to_string(element).unwrap())?;
            }
            seq.end()
        }
        None => panic!("unsupported"),
    }
}

/// Information property list.
/// https://developer.apple.com/documentation/bundleresources/information_property_list
#[derive(Deserialize, Serialize, Debug, Default)]
pub struct InfoPlist {
    // Bundle Configuration.
    #[serde(flatten)]
    pub categorization: Categorization,
    #[serde(flatten)]
    pub identification: Identification,
    #[serde(flatten)]
    pub naming: Naming,
    #[serde(flatten)]
    pub bundle_version: BundleVersion,
    #[serde(flatten)]
    pub operating_system_version: OperatingSystemVersion,
    #[serde(flatten)]
    pub localization: Localization,
    #[serde(flatten)]
    pub help: Help,
    // User Interface.
    #[serde(flatten)]
    pub main_user_interface: MainUserInterface,
    #[serde(flatten)]
    pub launch_interface: LaunchInterface,
    #[serde(flatten)]
    pub icons: Icons,
    #[serde(flatten)]
    pub orientation: Orientation,
    #[serde(flatten)]
    pub styling: Styling,
    #[serde(flatten)]
    pub status_bar: StatusBar,
    #[serde(flatten)]
    pub preferences: Preferences,
    #[serde(flatten)]
    pub graphics: Graphics,
    #[serde(flatten)]
    pub quick_look: QuickLook,
    // App Execution.
    #[serde(flatten)]
    pub launch: Launch,
    #[serde(flatten)]
    pub launch_conditions: LaunchConditions,
    #[serde(flatten)]
    pub extensions_and_services: ExtensionsAndServices,
    #[serde(flatten)]
    pub app_clips: AppClips,
    #[serde(flatten)]
    pub background_execution: BackgroundExecution,
    #[serde(flatten)]
    pub endpoint_security: EndpointSecurity,
    #[serde(flatten)]
    pub plugin_support: PluginSupport,
    #[serde(flatten)]
    pub plugin_configuration: PluginConfiguration,
    #[serde(flatten)]
    pub termination: Termination,
}
