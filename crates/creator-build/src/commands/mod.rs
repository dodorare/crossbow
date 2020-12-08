use crate::deps::Dependencies;
use crate::error::StdResult;

pub trait Command {
    type Output;
    type Deps: Dependencies;

    fn run(&self, deps: Self::Deps) -> StdResult<Self::Output>;
    fn check() -> StdResult<()> {
        Self::Deps::check()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::deps::{AndroidSdk, Rustc};
    use crate::error::StdResult;

    pub struct CommandX;

    impl Command for CommandX {
        type Output = ();
        type Deps = (AndroidSdk, Rustc);

        fn run(&self, (_android_sdk, _rustc): Self::Deps) -> StdResult<()> {
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
