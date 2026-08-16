use crate::error::Error;
use std::{path::Path, str::FromStr};

#[derive(Copy, Clone, Debug, Default, Eq, Hash, PartialEq)]
pub enum Profile {
    #[default]
    Debug,
    Release,
}

impl Profile {
    pub fn is_debug(&self) -> bool {
        Self::Debug == *self
    }

    pub(crate) fn cargo_name(self) -> &'static str {
        match self {
            Self::Debug => "dev",
            Self::Release => "release",
        }
    }
}

impl AsRef<Path> for Profile {
    fn as_ref(&self) -> &Path {
        Path::new(match self {
            Self::Debug => "debug",
            Self::Release => "release",
        })
    }
}

impl FromStr for Profile {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "debug" => Ok(Profile::Debug),
            "release" => Ok(Profile::Release),
            _ => Err(Error::InvalidProfile(s.to_owned())),
        }
    }
}

impl std::fmt::Display for Profile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Self::Debug => "debug",
            Self::Release => "release",
        })
    }
}
