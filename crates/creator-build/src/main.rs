use std::cell::RefCell;
use std::rc::Rc;

pub type BoxError = Box<dyn std::error::Error>;

#[derive(Clone)]
pub enum Checks {
    Vec(ChecksVec),
    None,
}

impl Checks {
    pub fn run(&self) -> Result<(), BoxError> {
        match self {
            Checks::Vec(checks) => checks.run()?,
            Checks::None => (),
        };
        Ok(())
    }
}

#[derive(Clone, Default)]
pub struct ChecksVec {
    items: Rc<RefCell<Vec<Box<dyn Fn() -> Result<(), BoxError>>>>>,
}

impl ChecksVec {
    pub fn push(&mut self, item: impl Fn() -> Result<(), BoxError> + 'static) {
        self.items.borrow_mut().push(Box::new(item));
    }

    pub fn run(&self) -> Result<(), BoxError> {
        self.items.borrow().iter().try_for_each(|item| item())
    }
}

pub trait Command {
    type Output;

    fn run(&self) -> Result<Self::Output, BoxError>;
    fn checks(&self) -> Checks {
        Checks::None
    }
}

fn check_1() -> Result<(), BoxError> {
    println!("check 1");
    Ok(())
}

fn check_2() -> Result<(), BoxError> {
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
                let mut checks = ChecksVec::default();
                checks.push(check_1);
                checks.push(check_2);
                Checks::Vec(checks)
            },
        }
    }
}

impl Command for CommandX {
    type Output = ();

    fn run(&self) -> Result<Self::Output, BoxError> {
        self.checks.run()?;
        println!("run command");
        Ok(())
    }

    fn checks(&self) -> Checks {
        self.checks.clone()
    }
}

fn main() -> Result<(), BoxError> {
    CommandX::new().run()?;
    Ok(())
}
