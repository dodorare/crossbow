use super::*;
use crate::error::StdResult;

pub struct Rustc;

impl Dependency for Rustc {
    type Input = ();

    fn check(&self) -> StdResult<()> {
        println!("checked rustc");
        Ok(())
    }

    fn init(_: Option<Self::Input>) -> StdResult<Arc<Self>> {
        Ok(Rustc.into())
    }
}
