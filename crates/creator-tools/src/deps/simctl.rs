use super::*;
use crate::error::Result;

pub type Simctl = apple_simctl::Simctl;

impl Checks for Simctl {
    fn check() -> Result<Vec<CheckInfo>> {
        Ok(Vec::new())
    }
}

// impl Simctl {
//     pub fn init() -> Result<Rc<Self>> {
//         Ok(Rc::new(Simctl::new()))
//     }
// }
