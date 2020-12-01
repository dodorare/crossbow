use std::path::Path;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum CrateType {
    Bin,
    Lib,
    Staticlib,
    Cdylib,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum Artifact {
    Root(String),
    Example(String),
}

impl AsRef<Path> for Artifact {
    fn as_ref(&self) -> &Path {
        Path::new(match self {
            Artifact::Root(_) => "",
            Artifact::Example(_) => "examples",
        })
    }
}

impl Artifact {
    pub fn name(&self) -> &str {
        match self {
            Artifact::Root(name) => name,
            Artifact::Example(name) => name,
        }
    }

    pub fn file_name(&self, ty: CrateType, target: &str) -> String {
        match ty {
            CrateType::Bin => {
                if target.contains("windows") {
                    format!("{}.exe", self.name())
                } else if target.contains("wasm") {
                    format!("{}.wasm", self.name())
                } else {
                    self.name().to_string()
                }
            }
            CrateType::Lib => format!("lib{}.rlib", self.name().replace("-", "_")),
            CrateType::Staticlib => format!("lib{}.a", self.name().replace("-", "_")),
            CrateType::Cdylib => format!("lib{}.so", self.name().replace("-", "_")),
        }
    }
}
