use std::process::Command;

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum CargoTargetSelection {
    Bin(String),
    Example(String),
    Lib(String),
}

impl CargoTargetSelection {
    pub fn name(&self) -> &str {
        match self {
            Self::Bin(name) | Self::Example(name) | Self::Lib(name) => name,
        }
    }

    pub(crate) fn append_to(&self, command: &mut Command) {
        match self {
            Self::Bin(name) => {
                command.args(["--bin", name]);
            }
            Self::Example(name) => {
                command.args(["--example", name]);
            }
            Self::Lib(_) => {
                command.arg("--lib");
            }
        }
    }

    pub(crate) fn kind(&self) -> &'static str {
        match self {
            Self::Bin(_) => "bin",
            Self::Example(_) => "example",
            Self::Lib(_) => "lib",
        }
    }

    pub(crate) fn matches_kind(&self, kinds: &[String]) -> bool {
        match self {
            Self::Bin(_) => kinds.iter().any(|kind| kind == "bin"),
            Self::Example(_) => kinds.iter().any(|kind| kind == "example"),
            Self::Lib(_) => kinds.iter().any(|kind| is_library_kind(kind)),
        }
    }
}

pub(crate) fn is_library_kind(kind: &str) -> bool {
    matches!(
        kind,
        "lib" | "rlib" | "dylib" | "cdylib" | "staticlib" | "proc-macro"
    )
}

pub trait IntoRustTriple {
    /// Returns the triple used by the rust build tools.
    fn rust_triple(&self) -> &'static str;
}
