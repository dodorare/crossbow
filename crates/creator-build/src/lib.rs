// mod apple;

use std::cell::RefCell;
use std::rc::Rc;

pub type BoxError = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, BoxError>;

#[derive(Clone, Default)]
pub struct Checks {
    items: Rc<RefCell<Vec<Box<dyn Fn() -> Result<()>>>>>,
}

impl Checks {
    pub fn push(&mut self, item: impl Fn() -> Result<()> + 'static) {
        self.items.borrow_mut().push(Box::new(item));
    }

    pub fn run(&self) -> Result<()> {
        self.items.borrow().iter().try_for_each(|item| item())
    }
}

pub trait Command {
    type Output;

    fn run(&self) -> Result<Self::Output>;
    fn checks(&self) -> Checks {
        Checks::default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn check_1() -> Result<()> {
        println!("check 1");
        Ok(())
    }

    fn check_2() -> Result<()> {
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

        fn run(&self) -> Result<Self::Output> {
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
