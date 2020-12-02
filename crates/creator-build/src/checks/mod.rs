use std::cell::RefCell;
use std::rc::Rc;

use crate::error::StdResult;

#[derive(Clone, Default)]
pub struct Checks {
    items: Rc<RefCell<Vec<Box<dyn Fn() -> StdResult<()>>>>>,
}

impl Checks {
    pub fn push(&mut self, item: impl Fn() -> StdResult<()> + 'static) {
        self.items.borrow_mut().push(Box::new(item));
    }

    pub fn run(&self) -> StdResult<()> {
        self.items.borrow().iter().try_for_each(|item| item())
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
        Err("error check 2".into())
    }

    #[test]
    #[should_panic(expected = "error check 2")]
    fn test_checks() {
        let mut checks = Checks::default();
        checks.push(check_1);
        checks.push(check_2);
        checks.run().unwrap();
    }
}
