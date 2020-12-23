use std::path::Path;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum Profile {
    Debug,
    Release,
}

impl AsRef<Path> for Profile {
    fn as_ref(&self) -> &Path {
        Path::new(match self {
            Self::Debug => "debug",
            Self::Release => "release",
        })
    }
}

impl Default for Profile {
    fn default() -> Self {
        Self::Debug
    }
}
