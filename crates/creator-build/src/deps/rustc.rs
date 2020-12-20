use super::*;
use crate::error::Result;

pub struct Rustc;

impl Checks for Rustc {
    fn check() -> Result<HashSet<CheckInfo>> {
        Ok(HashSet::new())
    }
}
