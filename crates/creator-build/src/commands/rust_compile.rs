use super::Command;
use crate::error::StdResult;

pub struct RustCompile;

impl Command for RustCompile {
    type Deps = ();
    type Output = ();

    fn run(&self, (): Self::Deps) -> StdResult<Self::Output> {
        Ok(())
    }
}
