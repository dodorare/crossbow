use super::*;
use crate::error::StdResult;

pub struct Rustc;

impl Dependency for Rustc {
    fn check(&self) -> StdResult<()> {
        println!("checked rustc");
        Ok(())
    }

    fn init() -> StdResult<Rc<Self>> {
        Ok(Rustc.into())
    }
}
