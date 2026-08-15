use semver::{Version, VersionReq};
use serde::Deserialize;
use std::collections::BTreeMap;

const TOOLS_MANIFEST: &str = include_str!("../../Cargo.toml");

#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub struct CompatibilityPolicy {
    pub schema_version: u32,
    pub host: BTreeMap<String, VersionPolicy>,
    pub android: BTreeMap<String, VersionPolicy>,
    #[serde(default)]
    pub apple: BTreeMap<String, VersionPolicy>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
pub struct VersionPolicy {
    pub preferred: String,
    pub supported: String,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Compatibility {
    Preferred,
    Supported,
    Unsupported,
    Unknown,
}

#[derive(Deserialize)]
struct Manifest {
    package: Package,
}
#[derive(Deserialize)]
struct Package {
    metadata: Metadata,
}
#[derive(Deserialize)]
struct Metadata {
    crossbundle: CrossbundleMetadata,
}
#[derive(Deserialize)]
struct CrossbundleMetadata {
    compatibility: CompatibilityPolicy,
}

impl CompatibilityPolicy {
    pub fn embedded() -> Self {
        toml::from_str::<Manifest>(TOOLS_MANIFEST)
            .expect("crossbundle-tools compatibility metadata must be valid")
            .package
            .metadata
            .crossbundle
            .compatibility
    }

    pub fn tool(&self, id: &str) -> Option<&VersionPolicy> {
        self.android.get(id)
    }

    pub fn host_tool(&self, id: &str) -> Option<&VersionPolicy> {
        self.host.get(id)
    }

    pub fn apple_tool(&self, id: &str) -> Option<&VersionPolicy> {
        self.apple.get(id)
    }
}

impl VersionPolicy {
    pub fn classify(&self, found: &str) -> Compatibility {
        if preferred_matches(&self.preferred, found) {
            return Compatibility::Preferred;
        }
        let Some(found) = normalize_version(found) else {
            return Compatibility::Unknown;
        };
        VersionReq::parse(&self.supported)
            .map(|requirement| {
                if requirement.matches(&found) {
                    Compatibility::Supported
                } else {
                    Compatibility::Unsupported
                }
            })
            .unwrap_or(Compatibility::Unknown)
    }
}

fn preferred_matches(preferred: &str, found: &str) -> bool {
    let Some(preferred) = version_components(preferred) else {
        return false;
    };
    let Some(found) = version_components(found) else {
        return false;
    };
    found.starts_with(&preferred)
}

fn normalize_version(value: &str) -> Option<Version> {
    let mut parts = version_components(value)?;
    parts.resize(3, 0);
    Some(Version::new(parts[0], parts[1], parts[2]))
}

fn version_components(value: &str) -> Option<Vec<u64>> {
    let numeric = value
        .trim()
        .trim_start_matches('v')
        .split_whitespace()
        .find(|part| part.chars().next().is_some_and(|c| c.is_ascii_digit()))?
        .trim_matches(|c: char| !c.is_ascii_digit() && c != '.');
    numeric
        .split('.')
        .map(str::parse)
        .collect::<Result<_, _>>()
        .ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reads_policy_from_the_published_crate_manifest() {
        let policy = CompatibilityPolicy::embedded();
        assert_eq!(policy.schema_version, 1);
        assert_eq!(policy.tool("ndk").unwrap().preferred, "28.2.13676358");
    }

    #[test]
    fn classifies_versions() {
        let policy = VersionPolicy {
            preferred: "17".into(),
            supported: ">=17, <22".into(),
        };
        assert_eq!(policy.classify("17"), Compatibility::Preferred);
        assert_eq!(policy.classify("17.0.19"), Compatibility::Preferred);
        assert_eq!(policy.classify("openjdk 21.0.2"), Compatibility::Supported);
        assert_eq!(policy.classify("11.0.4"), Compatibility::Unsupported);
        assert_eq!(policy.classify("unknown"), Compatibility::Unknown);
    }

    #[test]
    fn compares_the_complete_android_ndk_revision() {
        let policy = VersionPolicy {
            preferred: "28.2.13676358".into(),
            supported: ">=27, <29".into(),
        };
        assert_eq!(policy.classify("28.2.13676358"), Compatibility::Preferred);
        assert_eq!(policy.classify("28.2.13676359"), Compatibility::Supported);
    }
}
