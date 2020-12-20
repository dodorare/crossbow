mod bundle_configuration;
mod user_interface;

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
}
