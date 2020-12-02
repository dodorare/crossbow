use crate::checks::Checks;
use crate::error::StdResult;

pub trait Command {
    type Output;

    fn run(&self) -> StdResult<Self::Output>;
    fn checks(&self) -> Checks {
        Checks::default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::checks::check_android_sdk;

    fn check_1() -> StdResult<()> {
        println!("check 1");
        Ok(())
    }

    fn check_2() -> StdResult<()> {
        println!("check 2");
        Ok(())
    }

    pub struct CommandX {
        checks: Checks,
    }

    impl CommandX {
        pub fn new() -> Self {
            CommandX {
                checks: {
                    let mut checks = Checks::default();
                    checks.push(check_1);
                    checks.push(check_2);
                    checks.push(check_android_sdk);
                    checks
                },
            }
        }
    }

    impl Command for CommandX {
        type Output = ();

        fn run(&self) -> StdResult<Self::Output> {
            self.checks.run()?;
            // 1. check something (can u create something?)
            // 2. need to install
            // 3. installation
            // 4. check something (can u create something?)
            // 5. run command (create something)
            println!("run command");
            Ok(())
        }

        fn checks(&self) -> Checks {
            self.checks.clone()
        }
    }

    #[test]
    fn test_command() {
        CommandX::new().run().unwrap();
        CommandX::new().run().unwrap();
        CommandX::new().run().unwrap();
        CommandX::new().run().unwrap();
    }
}
