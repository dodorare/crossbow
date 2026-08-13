use crate::error::{Error, Result};

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
                .ok_or(Error::InvalidSemver)?
                .parse()
                .map_err(|_| Error::InvalidSemver)
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
