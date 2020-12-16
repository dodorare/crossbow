mod rust_compile;
mod generate_minimal_project;

pub use rust_compile::*;
pub use generate_minimal_project::*;

use crate::deps::Dependencies;
use crate::error::StdResult;

pub trait Command {
    type Deps: Dependencies;
    type Output;

    fn run(&self, deps: Self::Deps) -> StdResult<Self::Output>;
    fn check() -> StdResult<()> {
        Self::Deps::check()
    }
}

#[derive(Debug, Clone)]
pub enum BinOrLib {
    Bin(String),
    Lib,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::deps::{AndroidSdk, Rustc};
    use crate::error::StdResult;

    pub struct CommandX;

    impl Command for CommandX {
        type Deps = (AndroidSdk, Rustc);
        type Output = ();

        fn run(&self, (_android_sdk, _rustc): Self::Deps) -> StdResult<Self::Output> {
            println!("run command x");
            Ok(())
        }
    }

    #[test]
    fn test_command() {
        // init deps
        let android_sdk = AndroidSdk;
        let rustc = Rustc;

        // init command
        let cmdx = CommandX;

        // check deps if you want
        CommandX::check().unwrap();

        // run command with given deps
        cmdx.run((android_sdk, rustc)).unwrap();
    }
}
