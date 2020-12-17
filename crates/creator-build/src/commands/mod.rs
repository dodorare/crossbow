mod generate_minimal_project;
mod rust_compile;

pub use generate_minimal_project::*;
pub use rust_compile::*;

use crate::deps::{Dependencies, OptionalDependencies};
use crate::error::StdResult;

pub trait Command {
    type Deps: Dependencies;
    type OptDeps: OptionalDependencies;
    type Output;

    fn run(&self, deps: Self::Deps, opt_deps: Self::OptDeps) -> StdResult<Self::Output>;
}

#[derive(Debug, Clone)]
pub enum Target {
    Bin(String),
    Example(String),
    Lib,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::deps::*;
    use crate::error::StdResult;
    use std::sync::Arc;

    pub struct CommandX;

    impl Command for CommandX {
        type Deps = (Arc<AndroidSdk>, Arc<Rustc>);
        type OptDeps = ();
        type Output = ();

        fn run(
            &self,
            (_android_sdk, _rustc): Self::Deps,
            (): Self::OptDeps,
        ) -> StdResult<Self::Output> {
            println!("run command x");
            Ok(())
        }
    }

    #[test]
    fn test_command() -> StdResult<()> {
        // Init deps
        let android_sdk = AndroidSdk::init(None)?;
        let rustc = Rustc::init(None)?;
        // Init command
        let cmdx = CommandX;
        // Check deps if you want
        let deps = (android_sdk, rustc);
        deps.check()?;
        // Run command with given deps
        cmdx.run(deps, ())?;
        Ok(())
    }
}
