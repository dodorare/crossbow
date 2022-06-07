pub mod admob;
pub mod error;

pub use admob::*;
pub use error::*;

pub mod prelude {
    pub use super::admob::*;
    pub use super::google_admob;
}
