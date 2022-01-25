/// Stands for what application wrapper to use on build.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ApplicationWrapper {
    NdkGlue,
    Sokol,
}
