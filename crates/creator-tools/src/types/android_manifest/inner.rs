use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Feature {
    pub name: String,
    pub required: bool,
}

impl fmt::Display for Feature {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::write(
            f,
            format_args!(
                r#"<uses-feature android:name="{}" android:required="{}"/>"#,
                &self.name, self.required,
            ),
        )
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ApplicationMetadata {
    pub name: String,
    pub value: String,
}

impl fmt::Display for ApplicationMetadata {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::write(
            f,
            format_args!(
                r#"<meta-data android:name="{}" android:value="{}"/>"#,
                self.name, self.value
            ),
        )
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ActivityMetadata {
    pub name: String,
    pub value: String,
}

impl fmt::Display for ActivityMetadata {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::write(
            f,
            format_args!(
                r#"<meta-data android:name="{}" android:value="{}"/>"#,
                self.name, self.value
            ),
        )
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Permission {
    pub name: String,
    pub max_sdk_version: Option<u32>,
}

impl fmt::Display for Permission {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let max_sdk_version = self
            .max_sdk_version
            .as_ref()
            .map(|max_sdk_version| format!(r#"android:maxSdkVersion="{}""#, max_sdk_version))
            .unwrap_or_default();
        fmt::write(
            f,
            format_args!(
                r#"<uses-permission android:name="{}" {}/>"#,
                &self.name, max_sdk_version,
            ),
        )
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct IntentFilterData {
    pub scheme: Option<String>,
    pub host: Option<String>,
    pub prefix: Option<String>,
}

impl fmt::Display for IntentFilterData {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let host = if let Some(host) = self.host.as_ref() {
            format!(" android:host=\"{}\"", host)
        } else {
            "".into()
        };
        let prefix = if let Some(prefix) = self.prefix.as_ref() {
            format!(" android:pathPrefix=\"{}\"", prefix)
        } else {
            "".into()
        };
        let scheme = if let Some(scheme) = self.scheme.as_ref() {
            format!(" android:scheme=\"{}\"", scheme)
        } else {
            "".into()
        };
        fmt::write(f, format_args!("<data {} {} {}/>", scheme, &host, &prefix))
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct IntentFilter {
    pub name: String,
    pub categories: Vec<String>,
    pub data: Vec<IntentFilterData>,
}

impl fmt::Display for IntentFilter {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut categories = "".to_string();
        for category in &self.categories {
            categories = format!("{}<category android:name=\"{}\"/>", categories, category)
        }
        let mut data = "".to_string();
        for d in &self.data {
            data = format!("{}{}", data, d.to_string())
        }
        fmt::write(
            f,
            format_args!(
                "<intent-filter>
                \t{}
                \t{}
                \t<action android:name=\"{}\"/>
                </intent-filter>",
                &categories, &data, &self.name,
            ),
        )
    }
}
