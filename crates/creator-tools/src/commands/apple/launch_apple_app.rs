use crate::commands::Command;
use crate::{deps::*, error::*, types::AndroidTarget};
use std::path::PathBuf;
use std::rc::Rc;

pub struct LaunchAppleApp {}

impl LaunchAppleApp {
    pub fn new() -> Self {
        Self {}
    }
}

impl Command for LaunchAppleApp {
    type Deps = ();
    type Output = ();

    fn run(&self) -> Result<Self::Output> {
        Ok(())
    }
}
