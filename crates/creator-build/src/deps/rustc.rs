use super::Dependency;
use crate::error::StdResult;

pub struct Rustc;

impl Dependency for Rustc {
    type Input = ();

    fn check() -> StdResult<()> {
        println!("checked rustc");
        Ok(())
    }

    fn get(_: Self::Input) -> StdResult<Self> {
        Ok(Rustc)
    }
}
