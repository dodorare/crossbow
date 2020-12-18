mod bundle_configuration;
mod user_interface;

pub use bundle_configuration::*;
pub use user_interface::*;

use serde::{Deserialize, Serialize, Serializer};

fn serialize_enum_option<S: Serializer, T: Serialize>(
    value: &Option<T>,
    s: S,
) -> Result<S::Ok, S::Error> {
    s.serialize_str(&serde_plain::to_string(value).unwrap())
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
}
