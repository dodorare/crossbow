use std::path::Path;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum Profile {
    Dev,
    Release,
    Custom(String),
}

impl AsRef<Path> for Profile {
    fn as_ref(&self) -> &Path {
        Path::new(match self {
            Self::Dev => "debug",
            Self::Release => "release",
            Self::Custom(profile) => profile.as_str(),
        })
    }
}
