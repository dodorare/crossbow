use crate::error::{Error, Result};

#[cfg(feature = "android")]
fn invalid_semver() -> Error {
    crate::error::AndroidError::InvalidSemver.into()
}

#[cfg(not(feature = "android"))]
fn invalid_semver() -> Error {
    Error::InvalidSemver
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct Version {
    pub major: u8,
    pub minor: u8,
    pub patch: u8,
}

impl Version {
    pub fn new(major: u8, minor: u8, patch: u8) -> Self {
        Self {
            major,
            minor,
            patch,
        }
    }

    /// Create `Version` by parsing from string representation.
    pub fn from_semver(version: &str) -> Result<Self> {
        let mut iter = version.split(|c| ['.', '-', '+'].contains(&c));
        let mut parse_component = || {
            iter.next()
                .ok_or_else(invalid_semver)?
                .parse()
                .map_err(|_| invalid_semver())
        };
        Ok(Self::new(
            parse_component()?,
            parse_component()?,
            parse_component()?,
        ))
    }

    pub fn to_code(&self, apk_id: u8) -> u32 {
        (apk_id as u32) << 24
            | (self.major as u32) << 16
            | (self.minor as u32) << 8
            | self.patch as u32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn invalid_version_uses_platform_error_shape() {
        let error = Version::from_semver("not-a-version").unwrap_err();
        #[cfg(feature = "android")]
        assert!(matches!(
            error,
            Error::Android(crate::error::AndroidError::InvalidSemver)
        ));
        #[cfg(not(feature = "android"))]
        assert!(matches!(error, Error::InvalidSemver));
    }
}
