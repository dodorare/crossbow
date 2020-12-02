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
                    checks
                },
            }
        }
    }

    impl Command for CommandX {
        type Output = ();

        fn run(&self) -> StdResult<Self::Output> {
            self.checks.run()?;
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
    }
}
