use super::Dependency;
use crate::error::StdResult;

pub struct Rustc;

impl Dependency for Rustc {
    fn check() -> StdResult<()> {
        println!("checked rustc");
        Ok(())
    }
}
