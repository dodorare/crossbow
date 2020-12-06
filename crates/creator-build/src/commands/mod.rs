use crate::deps::{Dependency, IntoChecks};
use crate::error::StdResult;

use std::rc::Rc;

pub trait Command {
    type Output;
    type Deps: IntoChecks;

    fn run(&self) -> StdResult<Self::Output>;
    fn deps(&self) -> Rc<Self::Deps>;
    fn run_checks(&self) -> StdResult<()> {
        self.deps().into_checks().iter().try_for_each(|item| item())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    pub struct Rustc;

    impl Dependency for Rustc {
        fn check() -> StdResult<()> {
            println!("check rustc");
            Ok(())
        }
    }

    pub struct AndroidSdk;

    impl Dependency for AndroidSdk {
        fn check() -> StdResult<()> {
            println!("check android sdk");
            Ok(())
        }
    }

    pub struct CommandX {
        deps: Rc<(AndroidSdk, Rustc)>,
    }

    impl CommandX {
        pub fn new() -> Self {
            CommandX {
                deps: Rc::new((AndroidSdk, Rustc)),
            }
        }
    }

    impl Command for CommandX {
        type Output = ();
        type Deps = (AndroidSdk, Rustc);

        fn run(&self) -> StdResult<Self::Output> {
            println!("run command x");
            Ok(())
        }

        fn deps(&self) -> Rc<Self::Deps> {
            self.deps.clone()
        }
    }

    #[test]
    fn test_command() {
        let cmdx = CommandX::new();
        cmdx.run_checks().unwrap();
        cmdx.run().unwrap();
    }
}
