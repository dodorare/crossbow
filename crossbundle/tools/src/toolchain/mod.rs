//! Side-effect-free platform toolchain discovery, diagnostics, and build planning.

mod compatibility;
mod doctor;
#[cfg(feature = "android")]
mod plan;

pub use compatibility::*;
pub use doctor::*;
#[cfg(feature = "android")]
pub use plan::*;
