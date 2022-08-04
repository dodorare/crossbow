use crate::error::Error;
use std::{path::Path, str::FromStr};

#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
pub enum Profile {
    Debug,
    Release,
}

impl Profile {
    pub fn is_debug(&self) -> bool {
        Self::Debug == *self
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

impl ToString for Profile {
    fn to_string(&self) -> String {
        match self {
            Self::Debug => "debug".to_string(),
            Self::Release => "release".to_string(),
        }
    }
}

impl Default for Profile {
    fn default() -> Self {
        Self::Debug
    }
}
