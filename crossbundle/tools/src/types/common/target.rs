#[derive(Debug, Clone)]
pub enum Target {
    Bin(String),
    Example(String),
    Lib,
}

pub trait IntoRustTriple {
    /// Returns the triple used by the rust build tools.
    fn rust_triple(&self) -> &'static str;
}
